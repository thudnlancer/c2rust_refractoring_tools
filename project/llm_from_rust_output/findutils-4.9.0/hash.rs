use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ptr;

const DEFAULT_SHRINK_THRESHOLD: f32 = 0.0;
const DEFAULT_SHRINK_FACTOR: f32 = 1.0;
const DEFAULT_GROWTH_THRESHOLD: f32 = 0.8;
const DEFAULT_GROWTH_FACTOR: f32 = 1.414;

#[derive(Debug, Clone, Copy)]
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

struct HashEntry<T> {
    data: T,
    next: Option<Box<HashEntry<T>>>,
}

pub struct HashTable<T> {
    buckets: Vec<Option<Box<HashEntry<T>>>>,
    n_buckets_used: usize,
    n_entries: usize,
    tuning: HashTuning,
}

impl<T> HashTable<T> {
    pub fn new(candidate: usize, tuning: Option<HashTuning>) -> Self {
        let tuning = tuning.unwrap_or_default();
        let n_buckets = Self::compute_bucket_size(candidate, &tuning);
        
        HashTable {
            buckets: vec![None; n_buckets],
            n_buckets_used: 0,
            n_entries: 0,
            tuning,
        }
    }

    fn compute_bucket_size(candidate: usize, tuning: &HashTuning) -> usize {
        let mut candidate = if !tuning.is_n_buckets {
            let new_candidate = candidate as f32 / tuning.growth_threshold;
            if new_candidate > usize::MAX as f32 {
                return 0;
            }
            new_candidate as usize
        } else {
            candidate
        };

        candidate = Self::next_prime(candidate);
        candidate
    }

    fn next_prime(mut candidate: usize) -> usize {
        if candidate < 10 {
            candidate = 10;
        }
        
        candidate |= 1;
        while candidate != usize::MAX && !Self::is_prime(candidate) {
            candidate = candidate.wrapping_add(2);
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
        while square < candidate {
            if candidate % divisor == 0 || candidate % (divisor + 2) == 0 {
                return false;
            }
            divisor += 6;
            square = divisor * divisor;
        }
        true
    }

    pub fn len(&self) -> usize {
        self.n_entries
    }

    pub fn is_empty(&self) -> bool {
        self.n_entries == 0
    }

    pub fn contains<Q: PartialEq<T>>(&self, key: &Q) -> bool {
        self.find(key).is_some()
    }

    fn find<Q: PartialEq<T>>(&self, key: &Q) -> Option<&T> {
        let index = self.bucket_index(key);
        let mut entry = self.buckets[index].as_ref();
        
        while let Some(e) = entry {
            if key == &e.data {
                return Some(&e.data);
            }
            entry = e.next.as_ref();
        }
        None
    }

    fn bucket_index<Q: Hash>(&self, key: &Q) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        (hash % self.buckets.len() as u64) as usize
    }

    pub fn insert(&mut self, value: T) -> bool 
    where
        T: PartialEq + Hash,
    {
        if self.n_buckets_used as f32 > self.tuning.growth_threshold * self.buckets.len() as f32 {
            self.rehash();
        }

        let index = self.bucket_index(&value);
        let mut entry = &mut self.buckets[index];
        
        if entry.is_none() {
            self.n_buckets_used += 1;
        }

        while let Some(e) = entry {
            if e.data == value {
                return false;
            }
            entry = &mut e.next;
        }

        *entry = Some(Box::new(HashEntry {
            data: value,
            next: None,
        }));
        self.n_entries += 1;
        true
    }

    pub fn remove<Q: PartialEq<T> + Hash>(&mut self, key: &Q) -> Option<T> {
        let index = self.bucket_index(key);
        let mut prev = None;
        let mut current = &mut self.buckets[index];

        while let Some(mut entry) = current.take() {
            if key == &entry.data {
                *current = entry.next.take();
                self.n_entries -= 1;
                
                if self.buckets[index].is_none() {
                    self.n_buckets_used -= 1;
                    
                    if (self.n_buckets_used as f32) < self.tuning.shrink_threshold * self.buckets.len() as f32 {
                        self.rehash();
                    }
                }
                return Some(entry.data);
            }
            
            let next = entry.next.take();
            if prev.is_none() {
                *current = Some(entry);
                prev = current;
            } else {
                prev.as_mut().unwrap().next = Some(entry);
                prev = &mut prev.as_mut().unwrap().next;
            }
            current = &mut prev.as_mut().unwrap().next;
        }
        None
    }

    fn rehash(&mut self) {
        let new_size = if self.n_buckets_used as f32 > self.tuning.growth_threshold * self.buckets.len() as f32 {
            if self.tuning.is_n_buckets {
                (self.buckets.len() as f32 * self.tuning.growth_factor) as usize
            } else {
                (self.buckets.len() as f32 * self.tuning.growth_factor * self.tuning.growth_threshold) as usize
            }
        } else if (self.n_buckets_used as f32) < self.tuning.shrink_threshold * self.buckets.len() as f32 {
            if self.tuning.is_n_buckets {
                (self.buckets.len() as f32 * self.tuning.shrink_factor) as usize
            } else {
                (self.buckets.len() as f32 * self.tuning.shrink_factor * self.tuning.growth_threshold) as usize
            }
        } else {
            return;
        };

        let new_size = Self::compute_bucket_size(new_size, &self.tuning);
        if new_size == 0 || new_size == self.buckets.len() {
            return;
        }

        let mut new_table = HashTable {
            buckets: vec![None; new_size],
            n_buckets_used: 0,
            n_entries: 0,
            tuning: self.tuning,
        };

        for bucket in self.buckets.drain(..) {
            let mut current = bucket;
            while let Some(mut entry) = current.take() {
                current = entry.next.take();
                new_table.insert(entry.data);
            }
        }

        *self = new_table;
    }
}

impl<T> Drop for HashTable<T> {
    fn drop(&mut self) {
        for bucket in &mut self.buckets {
            let mut current = bucket.take();
            while let Some(mut entry) = current.take() {
                current = entry.next.take();
            }
        }
    }
}