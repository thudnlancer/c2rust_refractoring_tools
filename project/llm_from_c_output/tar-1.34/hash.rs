use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::mem;
use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct HashTuning {
    pub shrink_threshold: f32,
    pub shrink_factor: f32,
    pub growth_threshold: f32,
    pub growth_factor: f32,
    pub is_n_buckets: bool,
}

impl Default for HashTuning {
    fn default() -> Self {
        HashTuning {
            shrink_threshold: 0.0f32,
            shrink_factor: 1.0f32,
            growth_threshold: 0.8f32,
            growth_factor: 1.414f32,
            is_n_buckets: false,
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

struct HashEntry<T> {
    data: T,
    next: Option<Box<HashEntry<T>>>,
}

impl<T> HashTable<T> {
    pub fn new(
        candidate: usize,
        tuning: Option<HashTuning>,
        hasher: fn(&T, usize) -> usize,
        comparator: fn(&T, &T) -> bool,
    ) -> Option<Self> {
        let tuning = tuning.unwrap_or_default();
        let n_buckets = Self::compute_bucket_size(candidate, &tuning)?;
        
        Some(HashTable {
            buckets: vec![None; n_buckets],
            n_buckets_used: 0,
            n_entries: 0,
            tuning,
            hasher,
            comparator,
        })
    }

    fn compute_bucket_size(candidate: usize, tuning: &HashTuning) -> Option<usize> {
        let candidate = if !tuning.is_n_buckets {
            (candidate as f32 / tuning.growth_threshold) as usize
        } else {
            candidate
        };
        Some(Self::next_prime(candidate.max(10) | 1))
    }

    fn next_prime(mut candidate: usize) -> usize {
        while !Self::is_prime(candidate) {
            candidate += 2;
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
        let mut i = 5;
        let mut w = 2;
        while i * i <= candidate {
            if candidate % i == 0 {
                return false;
            }
            i += w;
            w = 6 - w;
        }
        true
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
                let mut current = bucket;
                while let Some(entry) = current {
                    len += 1;
                    current = &entry.next;
                }
                len
            })
            .max()
            .unwrap_or(0)
    }

    pub fn lookup(&self, entry: &T) -> Option<&T> {
        let index = (self.hasher)(entry, self.buckets.len());
        let mut current = &self.buckets[index];
        while let Some(entry_ptr) = current {
            if (self.comparator)(entry, &entry_ptr.data) {
                return Some(&entry_ptr.data);
            }
            current = &entry_ptr.next;
        }
        None
    }

    pub fn insert(&mut self, entry: T) -> Option<&T> {
        if self.n_buckets_used > (self.tuning.growth_threshold * self.buckets.len() as f32) as usize {
            let new_size = if self.tuning.is_n_buckets {
                (self.buckets.len() as f32 * self.tuning.growth_factor) as usize
            } else {
                (self.buckets.len() as f32 * self.tuning.growth_factor * self.tuning.growth_threshold) as usize
            };
            self.rehash(new_size)?;
        }

        let index = (self.hasher)(&entry, self.buckets.len());
        let mut current = &mut self.buckets[index];

        if current.is_none() {
            self.n_buckets_used += 1;
        }

        while let Some(entry_ptr) = current {
            if (self.comparator)(&entry, &entry_ptr.data) {
                return Some(&entry_ptr.data);
            }
            current = &mut entry_ptr.next;
        }

        *current = Some(Box::new(HashEntry {
            data: entry,
            next: None,
        }));
        self.n_entries += 1;
        None
    }

    fn rehash(&mut self, new_size: usize) -> Option<()> {
        let new_size = Self::compute_bucket_size(new_size, &self.tuning)?;
        if new_size == self.buckets.len() {
            return Some(());
        }

        let mut new_table = HashTable {
            buckets: vec![None; new_size],
            n_buckets_used: 0,
            n_entries: 0,
            tuning: self.tuning,
            hasher: self.hasher,
            comparator: self.comparator,
        };

        for bucket in &mut self.buckets {
            let mut current = bucket.take();
            while let Some(mut entry) = current {
                let next = entry.next.take();
                let index = (new_table.hasher)(&entry.data, new_table.buckets.len());
                
                if new_table.buckets[index].is_none() {
                    new_table.n_buckets_used += 1;
                }
                
                entry.next = new_table.buckets[index].take();
                new_table.buckets[index] = Some(entry);
                new_table.n_entries += 1;
                
                current = next;
            }
        }

        *self = new_table;
        Some(())
    }

    pub fn remove(&mut self, entry: &T) -> Option<T> {
        let index = (self.hasher)(entry, self.buckets.len());
        let mut prev = None;
        let mut current = &mut self.buckets[index];

        while let Some(entry_ptr) = current {
            if (self.comparator)(entry, &entry_ptr.data) {
                let data = if let Some(mut prev_entry) = prev {
                    let removed = prev_entry.next.take().unwrap();
                    prev_entry.next = removed.next;
                    removed.data
                } else {
                    let removed = current.take().unwrap();
                    *current = removed.next;
                    if current.is_none() {
                        self.n_buckets_used -= 1;
                    }
                    removed.data
                };
                self.n_entries -= 1;
                return Some(data);
            }
            prev = current.as_mut();
            current = &mut prev.as_mut().unwrap().next;
        }
        None
    }

    pub fn clear(&mut self) {
        for bucket in &mut self.buckets {
            *bucket = None;
        }
        self.n_buckets_used = 0;
        self.n_entries = 0;
    }
}

impl<T> Drop for HashTable<T> {
    fn drop(&mut self) {
        self.clear();
    }
}

pub fn hash_string(string: &str, n_buckets: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    string.hash(&mut hasher);
    (hasher.finish() as usize) % n_buckets
}

pub fn raw_hasher<T>(data: &T, n: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    ptr::hash(data, &mut hasher);
    (hasher.finish() as usize) % n
}

pub fn raw_comparator<T>(a: &T, b: &T) -> bool {
    ptr::eq(a, b)
}