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

pub struct HashTable<T: Eq + Hash> {
    buckets: Vec<Option<Box<HashEntry<T>>>>,
    n_buckets_used: usize,
    n_entries: usize,
    tuning: HashTuning,
}

struct HashEntry<T> {
    data: T,
    next: Option<Box<HashEntry<T>>>,
}

impl<T: Eq + Hash> HashTable<T> {
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
        if !tuning.is_n_buckets {
            let new_candidate = (candidate as f32 / tuning.growth_threshold) as usize;
            next_prime(new_candidate)
        } else {
            next_prime(candidate)
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

    pub fn hash_string(s: &str, n_buckets: usize) -> usize {
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        (hasher.finish() as usize) % n_buckets
    }

    pub fn lookup(&self, entry: &T) -> Option<&T> {
        let index = self.bucket_index(entry);
        let mut current = &self.buckets[index];
        while let Some(entry_ref) = current {
            if &entry_ref.data == entry {
                return Some(&entry_ref.data);
            }
            current = &entry_ref.next;
        }
        None
    }

    pub fn insert(&mut self, entry: T) -> Option<T> {
        if self.n_buckets_used > (self.tuning.growth_threshold * self.buckets.len() as f32) as usize {
            let new_size = if self.tuning.is_n_buckets {
                (self.buckets.len() as f32 * self.tuning.growth_factor) as usize
            } else {
                (self.buckets.len() as f32 * self.tuning.growth_factor * self.tuning.growth_threshold) as usize
            };
            self.rehash(new_size);
        }

        let index = self.bucket_index(&entry);
        let mut current = &mut self.buckets[index];
        
        if current.is_none() {
            self.buckets[index] = Some(Box::new(HashEntry {
                data: entry,
                next: None,
            }));
            self.n_buckets_used += 1;
            self.n_entries += 1;
            return None;
        }

        let mut prev = current;
        while let Some(entry_ref) = prev {
            if entry_ref.data == entry {
                return Some(entry);
            }
            prev = &mut entry_ref.next;
        }

        let new_entry = Box::new(HashEntry {
            data: entry,
            next: self.buckets[index].take().map(|e| e),
        });
        self.buckets[index] = Some(new_entry);
        self.n_entries += 1;
        None
    }

    pub fn remove(&mut self, entry: &T) -> Option<T> {
        let index = self.bucket_index(entry);
        let mut prev = &mut self.buckets[index];
        
        if let Some(entry_ref) = prev {
            if &entry_ref.data == entry {
                let data = entry_ref.data;
                self.buckets[index] = entry_ref.next.take();
                self.n_entries -= 1;
                if self.buckets[index].is_none() {
                    self.n_buckets_used -= 1;
                    
                    if self.n_buckets_used < (self.tuning.shrink_threshold * self.buckets.len() as f32) as usize {
                        let new_size = if self.tuning.is_n_buckets {
                            (self.buckets.len() as f32 * self.tuning.shrink_factor) as usize
                        } else {
                            (self.buckets.len() as f32 * self.tuning.shrink_factor * self.tuning.growth_threshold) as usize
                        };
                        self.rehash(new_size);
                    }
                }
                return Some(data);
            }
        }

        while let Some(entry_ref) = prev {
            if let Some(next) = &mut entry_ref.next {
                if &next.data == entry {
                    let data = next.data;
                    entry_ref.next = next.next.take();
                    self.n_entries -= 1;
                    return Some(data);
                }
            }
            prev = &mut entry_ref.next;
        }
        
        None
    }

    fn bucket_index(&self, entry: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        entry.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    fn rehash(&mut self, new_size: usize) {
        let new_size = next_prime(new_size);
        if new_size == self.buckets.len() {
            return;
        }

        let mut new_buckets = vec![None; new_size];
        let mut new_n_buckets_used = 0;
        
        for bucket in self.buckets.drain(..) {
            let mut current = bucket;
            while let Some(mut entry) = current {
                let next = entry.next.take();
                let index = {
                    let mut hasher = DefaultHasher::new();
                    entry.data.hash(&mut hasher);
                    (hasher.finish() as usize) % new_size
                };
                
                if new_buckets[index].is_none() {
                    new_n_buckets_used += 1;
                }
                
                entry.next = new_buckets[index].take();
                new_buckets[index] = Some(entry);
                current = next;
            }
        }
        
        self.buckets = new_buckets;
        self.n_buckets_used = new_n_buckets_used;
    }
}

fn is_prime(candidate: usize) -> bool {
    if candidate < 2 {
        return false;
    }
    if candidate == 2 {
        return true;
    }
    if candidate % 2 == 0 {
        return false;
    }
    
    let mut divisor = 3;
    while divisor * divisor <= candidate {
        if candidate % divisor == 0 {
            return false;
        }
        divisor += 2;
    }
    true
}

fn next_prime(mut candidate: usize) -> usize {
    if candidate < 10 {
        candidate = 10;
    }
    if candidate % 2 == 0 {
        candidate += 1;
    }
    while !is_prime(candidate) {
        candidate += 2;
    }
    candidate
}