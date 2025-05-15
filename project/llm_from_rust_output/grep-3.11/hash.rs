use std::alloc::{alloc, dealloc, Layout};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr;

const DEFAULT_SHRINK_THRESHOLD: f32 = 0.0;
const DEFAULT_SHRINK_FACTOR: f32 = 1.0;
const DEFAULT_GROWTH_THRESHOLD: f32 = 0.8;
const DEFAULT_GROWTH_FACTOR: f32 = 1.414;

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

struct HashTable<T> {
    buckets: Vec<Option<Box<HashEntry<T>>>>,
    n_entries: usize,
    tuning: HashTuning,
}

impl<T: Hash + PartialEq> HashTable<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buckets = Vec::with_capacity(capacity);
        buckets.resize_with(capacity, || None);
        
        HashTable {
            buckets,
            n_entries: 0,
            tuning: HashTuning::default(),
        }
    }

    pub fn insert(&mut self, value: T) -> bool {
        let index = self.hash(&value);
        
        if let Some(entry) = &mut self.buckets[index] {
            let mut current = entry;
            loop {
                if current.data == value {
                    return false;
                }
                if current.next.is_none() {
                    break;
                }
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(Box::new(HashEntry {
                data: value,
                next: None,
            }));
        } else {
            self.buckets[index] = Some(Box::new(HashEntry {
                data: value,
                next: None,
            }));
        }
        
        self.n_entries += 1;
        self.maybe_resize();
        true
    }

    pub fn contains(&self, value: &T) -> bool {
        let index = self.hash(value);
        
        if let Some(entry) = &self.buckets[index] {
            let mut current = entry;
            loop {
                if &current.data == value {
                    return true;
                }
                if current.next.is_none() {
                    break;
                }
                current = current.next.as_ref().unwrap();
            }
        }
        false
    }

    pub fn remove(&mut self, value: &T) -> bool {
        let index = self.hash(value);
        
        if let Some(entry) = &mut self.buckets[index] {
            if entry.data == *value {
                self.buckets[index] = entry.next.take();
                self.n_entries -= 1;
                self.maybe_shrink();
                return true;
            }
            
            let mut prev = entry;
            while let Some(current) = &mut prev.next {
                if current.data == *value {
                    prev.next = current.next.take();
                    self.n_entries -= 1;
                    self.maybe_shrink();
                    return true;
                }
                prev = prev.next.as_mut().unwrap();
            }
        }
        false
    }

    fn hash(&self, value: &T) -> usize {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }

    fn maybe_resize(&mut self) {
        let load_factor = self.n_entries as f32 / self.buckets.len() as f32;
        if load_factor > self.tuning.growth_threshold {
            let new_size = (self.buckets.len() as f32 * self.tuning.growth_factor) as usize;
            self.resize(new_size);
        }
    }

    fn maybe_shrink(&mut self) {
        let load_factor = self.n_entries as f32 / self.buckets.len() as f32;
        if load_factor < self.tuning.shrink_threshold {
            let new_size = (self.buckets.len() as f32 * self.tuning.shrink_factor) as usize;
            self.resize(new_size);
        }
    }

    fn resize(&mut self, new_size: usize) {
        let mut new_table = HashTable::new(new_size);
        for bucket in self.buckets.drain(..) {
            let mut current = bucket;
            while let Some(mut entry) = current {
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
            while let Some(mut entry) = current {
                current = entry.next.take();
            }
        }
    }
}