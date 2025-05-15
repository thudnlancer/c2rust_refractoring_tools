use std::alloc::{alloc, dealloc, Layout};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr;

const DEFAULT_SHRINK_THRESHOLD: f32 = 0.0;
const DEFAULT_SHRINK_FACTOR: f32 = 1.0;
const DEFAULT_GROWTH_THRESHOLD: f32 = 0.8;
const DEFAULT_GROWTH_FACTOR: f32 = 1.414;

#[derive(Debug, Clone)]
struct HashTuning {
    shrink_threshold: f32,
    shrink_factor: f32,
    growth_threshold: f32,
    growth_factor: f32,
    is_n_buckets: bool,
}

impl Default for HashTuning {
    fn default() -> Self {
        HashTuning {
            shrink_threshold: DEFAULT_SHRINK_THRESHOLD,
            shrink_factor: DEFAULT_SHRINK_FACTOR,
            growth_threshold: DEFAULT_GROWTH_THRESHOLD,
            growth_factor: DEFAULT_GROWTH_FACTOR,
            is_n_buckets: false,
        }
    }
}

#[derive(Debug)]
struct HashEntry<T> {
    data: T,
    next: Option<Box<HashEntry<T>>>,
}

impl<T> HashEntry<T> {
    fn new(data: T) -> Self {
        HashEntry { data, next: None }
    }
}

type HashHasher<T> = fn(&T, usize) -> usize;
type HashComparator<T> = fn(&T, &T) -> bool;
type HashDataFreer<T> = fn(T);

#[derive(Debug)]
pub struct HashTable<T> {
    buckets: Vec<Option<Box<HashEntry<T>>>>,
    n_buckets_used: usize,
    n_entries: usize,
    tuning: HashTuning,
    hasher: HashHasher<T>,
    comparator: HashComparator<T>,
    data_freer: Option<HashDataFreer<T>>,
}

impl<T> HashTable<T> {
    pub fn new(
        initial_size: usize,
        tuning: Option<HashTuning>,
        hasher: Option<HashHasher<T>>,
        comparator: Option<HashComparator<T>>,
        data_freer: Option<HashDataFreer<T>>,
    ) -> Self {
        let tuning = tuning.unwrap_or_default();
        let hasher = hasher.unwrap_or(default_hasher);
        let comparator = comparator.unwrap_or(default_comparator);
        
        let size = compute_bucket_size(initial_size, &tuning);
        let mut buckets = Vec::with_capacity(size);
        buckets.resize_with(size, || None);

        HashTable {
            buckets,
            n_buckets_used: 0,
            n_entries: 0,
            tuning,
            hasher,
            comparator,
            data_freer,
        }
    }

    pub fn get_n_buckets(&self) -> usize {
        self.buckets.len()
    }

    pub fn get_n_buckets_used(&self) -> usize {
        self.n_buckets_used
    }

    pub fn get_n_entries(&self) -> usize {
        self.n_entries
    }

    pub fn get_max_bucket_length(&self) -> usize {
        self.buckets
            .iter()
            .map(|bucket| {
                let mut len = 0;
                let mut current = bucket.as_ref();
                while let Some(entry) = current {
                    len += 1;
                    current = entry.next.as_ref();
                }
                len
            })
            .max()
            .unwrap_or(0)
    }

    pub fn is_ok(&self) -> bool {
        let mut n_buckets_used = 0;
        let mut n_entries = 0;

        for bucket in &self.buckets {
            let mut current = bucket.as_ref();
            if current.is_some() {
                n_buckets_used += 1;
            }

            while let Some(entry) = current {
                n_entries += 1;
                current = entry.next.as_ref();
            }
        }

        n_buckets_used == self.n_buckets_used && n_entries == self.n_entries
    }

    pub fn lookup(&self, entry: &T) -> Option<&T> {
        let index = (self.hasher)(entry, self.buckets.len());
        let mut current = self.buckets[index].as_ref();

        while let Some(entry_ref) = current {
            if (self.comparator)(entry, &entry_ref.data) {
                return Some(&entry_ref.data);
            }
            current = entry_ref.next.as_ref();
        }

        None
    }

    pub fn get_first(&self) -> Option<&T> {
        for bucket in &self.buckets {
            if let Some(entry) = bucket.as_ref() {
                return Some(&entry.data);
            }
        }
        None
    }

    pub fn get_next(&self, entry: &T) -> Option<&T> {
        let index = (self.hasher)(entry, self.buckets.len());
        let mut current = self.buckets[index].as_ref();

        // Find the entry in its bucket
        while let Some(entry_ref) = current {
            if (self.comparator)(entry, &entry_ref.data) {
                // Check if there's a next entry in the chain
                if let Some(next) = entry_ref.next.as_ref() {
                    return Some(&next.data);
                }
                break;
            }
            current = entry_ref.next.as_ref();
        }

        // Look for next non-empty bucket
        for i in index + 1..self.buckets.len() {
            if let Some(entry) = self.buckets[i].as_ref() {
                return Some(&entry.data);
            }
        }

        None
    }

    pub fn insert(&mut self, entry: T) -> Result<(), ()> {
        if self.n_buckets_used as f32 > self.tuning.growth_threshold * self.buckets.len() as f32 {
            let new_size = if self.tuning.is_n_buckets {
                (self.buckets.len() as f32 * self.tuning.growth_factor) as usize
            } else {
                (self.buckets.len() as f32 * self.tuning.growth_factor * self.tuning.growth_threshold) as usize
            };

            if !self.rehash(new_size) {
                return Err(());
            }
        }

        let index = (self.hasher)(&entry, self.buckets.len());
        let bucket = &mut self.buckets[index];

        if bucket.is_none() {
            self.n_buckets_used += 1;
        }

        let new_entry = Box::new(HashEntry::new(entry));
        *bucket = Some(new_entry);
        self.n_entries += 1;

        Ok(())
    }

    pub fn remove(&mut self, entry: &T) -> Option<T> {
        let index = (self.hasher)(entry, self.buckets.len());
        let bucket = &mut self.buckets[index];

        if let Some(mut entry_ref) = bucket.take() {
            if (self.comparator)(entry, &entry_ref.data) {
                self.n_entries -= 1;
                if entry_ref.next.is_none() {
                    self.n_buckets_used -= 1;
                } else {
                    *bucket = entry_ref.next.take();
                }
                return Some(entry_ref.data);
            }

            let mut prev = &mut entry_ref;
            while let Some(mut next_entry) = prev.next.take() {
                if (self.comparator)(entry, &next_entry.data) {
                    prev.next = next_entry.next.take();
                    self.n_entries -= 1;
                    *bucket = Some(entry_ref);
                    return Some(next_entry.data);
                }
                prev.next = Some(next_entry);
                prev = prev.next.as_mut().unwrap();
            }
            *bucket = Some(entry_ref);
        }

        None
    }

    fn rehash(&mut self, new_size: usize) -> bool {
        let new_size = compute_bucket_size(new_size, &self.tuning);
        if new_size == 0 || new_size == self.buckets.len() {
            return false;
        }

        let mut new_buckets = Vec::with_capacity(new_size);
        new_buckets.resize_with(new_size, || None);

        for mut bucket in self.buckets.drain(..) {
            while let Some(mut entry) = bucket.take() {
                let next = entry.next.take();
                let index = (self.hasher)(&entry.data, new_buckets.len());
                
                if new_buckets[index].is_none() {
                    self.n_buckets_used += 1;
                }
                
                entry.next = new_buckets[index].take();
                new_buckets[index] = Some(entry);
                bucket = next;
            }
        }

        self.buckets = new_buckets;
        true
    }
}

impl<T> Drop for HashTable<T> {
    fn drop(&mut self) {
        if let Some(freer) = self.data_freer {
            for bucket in &mut self.buckets {
                while let Some(entry) = bucket.take() {
                    freer(entry.data);
                    *bucket = entry.next;
                }
            }
        }
    }
}

fn default_hasher<T: Hash>(data: &T, n_buckets: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    data.hash(&mut hasher);
    (hasher.finish() as usize) % n_buckets
}

fn default_comparator<T: PartialEq>(a: &T, b: &T) -> bool {
    a == b
}

fn compute_bucket_size(candidate: usize, tuning: &HashTuning) -> usize {
    if !tuning.is_n_buckets {
        let new_candidate = candidate as f32 / tuning.growth_threshold;
        if new_candidate > usize::MAX as f32 {
            return 0;
        }
        next_prime(new_candidate as usize)
    } else {
        next_prime(candidate)
    }
}

fn next_prime(mut candidate: usize) -> usize {
    if candidate < 10 {
        candidate = 10;
    }
    candidate |= 1;
    while !is_prime(candidate) {
        candidate = candidate.checked_add(2).unwrap_or(usize::MAX);
        if candidate == usize::MAX {
            break;
        }
    }
    candidate
}

fn is_prime(candidate: usize) -> bool {
    if candidate <= 1 {
        return false;
    }
    if candidate <= 3 {
        return true;
    }
    if candidate % 2 == 0 || candidate % 3 == 0 {
        return false;
    }
    
    let mut divisor = 5;
    let mut square = divisor * divisor;
    while square <= candidate {
        if candidate % divisor == 0 || candidate % (divisor + 2) == 0 {
            return false;
        }
        divisor += 6;
        square = divisor * divisor;
    }
    true
}