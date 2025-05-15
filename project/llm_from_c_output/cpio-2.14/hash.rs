use std::collections::hash_map::DefaultHasher;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ptr;

const DEFAULT_GROWTH_THRESHOLD: f32 = 0.8;
const DEFAULT_GROWTH_FACTOR: f32 = 1.414;
const DEFAULT_SHRINK_THRESHOLD: f32 = 0.0;
const DEFAULT_SHRINK_FACTOR: f32 = 1.0;

#[derive(Debug, Clone)]
pub struct HashTuning {
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

struct HashEntry<T> {
    data: Option<T>,
    next: Option<Box<HashEntry<T>>>,
}

impl<T> HashEntry<T> {
    fn new(data: T) -> Self {
        HashEntry {
            data: Some(data),
            next: None,
        }
    }
}

pub struct HashTable<T> {
    buckets: Vec<Option<Box<HashEntry<T>>>>,
    n_buckets_used: usize,
    n_entries: usize,
    tuning: HashTuning,
    hasher: fn(&T, usize) -> usize,
    comparator: fn(&T, &T) -> bool,
}

impl<T> HashTable<T> {
    pub fn new(
        candidate: usize,
        tuning: Option<HashTuning>,
        hasher: Option<fn(&T, usize) -> usize>,
        comparator: Option<fn(&T, &T) -> bool>,
    ) -> Option<Self> {
        let tuning = tuning.unwrap_or_default();
        let hasher = hasher.unwrap_or(raw_hasher);
        let comparator = comparator.unwrap_or(raw_comparator);

        let n_buckets = compute_bucket_size(candidate, &tuning)?;
        let mut buckets = Vec::with_capacity(n_buckets);
        buckets.resize_with(n_buckets, || None);

        Some(HashTable {
            buckets,
            n_buckets_used: 0,
            n_entries: 0,
            tuning,
            hasher,
            comparator,
        })
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
        let mut max_len = 0;
        for bucket in &self.buckets {
            let mut len = 0;
            let mut current = bucket;
            while let Some(entry) = current {
                len += 1;
                current = &entry.next;
            }
            if len > max_len {
                max_len = len;
            }
        }
        max_len
    }

    pub fn is_ok(&self) -> bool {
        let mut n_buckets_used = 0;
        let mut n_entries = 0;

        for bucket in &self.buckets {
            if let Some(entry) = bucket {
                n_buckets_used += 1;
                n_entries += 1;
                let mut current = &entry.next;
                while let Some(entry) = current {
                    n_entries += 1;
                    current = &entry.next;
                }
            }
        }

        n_buckets_used == self.n_buckets_used && n_entries == self.n_entries
    }

    pub fn lookup(&self, entry: &T) -> Option<&T> {
        let bucket_idx = (self.hasher)(entry, self.buckets.len());
        let mut current = self.buckets[bucket_idx].as_ref();

        while let Some(entry_ref) = current {
            if (self.comparator)(entry, entry_ref.data.as_ref().unwrap()) {
                return entry_ref.data.as_ref();
            }
            current = entry_ref.next.as_ref();
        }

        None
    }

    pub fn insert(&mut self, entry: T) -> Option<T> {
        if self.n_buckets_used > (self.tuning.growth_threshold * self.buckets.len() as f32) as usize {
            let candidate = (self.buckets.len() as f32 * self.tuning.growth_factor) as usize;
            if !self.rehash(candidate) {
                return None;
            }
        }

        let bucket_idx = (self.hasher)(&entry, self.buckets.len());
        let bucket = &mut self.buckets[bucket_idx];

        if let Some(bucket_entry) = bucket {
            let mut current = bucket_entry;
            loop {
                if (self.comparator)(&entry, current.data.as_ref().unwrap()) {
                    return Some(entry);
                }
                if current.next.is_none() {
                    break;
                }
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(Box::new(HashEntry::new(entry)));
        } else {
            *bucket = Some(Box::new(HashEntry::new(entry)));
            self.n_buckets_used += 1;
        }

        self.n_entries += 1;
        None
    }

    pub fn remove(&mut self, entry: &T) -> Option<T> {
        let bucket_idx = (self.hasher)(entry, self.buckets.len());
        let bucket = &mut self.buckets[bucket_idx];

        if let Some(bucket_entry) = bucket {
            if (self.comparator)(entry, bucket_entry.data.as_ref().unwrap()) {
                let data = bucket_entry.data.take();
                if let Some(next) = bucket_entry.next.take() {
                    *bucket = Some(next);
                } else {
                    *bucket = None;
                    self.n_buckets_used -= 1;
                }
                self.n_entries -= 1;
                return data;
            }

            let mut prev = bucket_entry;
            while let Some(current) = &mut prev.next {
                if (self.comparator)(entry, current.data.as_ref().unwrap()) {
                    let data = current.data.take();
                    prev.next = current.next.take();
                    self.n_entries -= 1;
                    return data;
                }
                prev = current;
            }
        }

        None
    }

    fn rehash(&mut self, candidate: usize) -> bool {
        let new_size = compute_bucket_size(candidate, &self.tuning).unwrap_or(0);
        if new_size == 0 || new_size == self.buckets.len() {
            return false;
        }

        let mut new_buckets = Vec::with_capacity(new_size);
        new_buckets.resize_with(new_size, || None);

        for mut bucket in self.buckets.drain(..) {
            while let Some(mut entry) = bucket.take() {
                if let Some(next) = entry.next.take() {
                    bucket = Some(next);
                }
                let entry_data = entry.data.take().unwrap();
                let new_bucket_idx = (self.hasher)(&entry_data, new_size);
                let new_bucket = &mut new_buckets[new_bucket_idx];
                
                if new_bucket.is_none() {
                    self.n_buckets_used += 1;
                }
                
                entry.data = Some(entry_data);
                entry.next = new_bucket.take();
                *new_bucket = Some(entry);
            }
        }

        self.buckets = new_buckets;
        true
    }
}

fn raw_hasher<T>(data: &T, n: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    ptr::hash(data, &mut hasher);
    hasher.finish() as usize % n
}

fn raw_comparator<T>(a: &T, b: &T) -> bool {
    ptr::eq(a, b)
}

fn compute_bucket_size(candidate: usize, tuning: &HashTuning) -> Option<usize> {
    let candidate = if !tuning.is_n_buckets {
        (candidate as f32 / tuning.growth_threshold) as usize
    } else {
        candidate
    };
    next_prime(candidate)
}

fn is_prime(candidate: usize) -> bool {
    if candidate < 2 {
        return false;
    }
    if candidate % 2 == 0 {
        return candidate == 2;
    }
    if candidate % 3 == 0 {
        return candidate == 3;
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

fn next_prime(mut candidate: usize) -> Option<usize> {
    if candidate <= 2 {
        return Some(2);
    }
    if candidate == 3 {
        return Some(3);
    }

    candidate |= 1;
    while candidate < usize::MAX {
        if is_prime(candidate) {
            return Some(candidate);
        }
        candidate += 2;
    }
    None
}