use std::alloc::{alloc, dealloc, Layout};
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

#[derive(Debug)]
pub struct HashTable<T> {
    buckets: Vec<Option<Box<HashEntry<T>>>>,
    n_buckets_used: usize,
    n_entries: usize,
    tuning: HashTuning,
}

impl<T> HashTable<T> {
    pub fn new(capacity: usize) -> Self {
        let mut buckets = Vec::with_capacity(capacity);
        buckets.resize_with(capacity, || None);
        
        HashTable {
            buckets,
            n_buckets_used: 0,
            n_entries: 0,
            tuning: HashTuning::default(),
        }
    }

    pub fn with_tuning(capacity: usize, tuning: HashTuning) -> Self {
        let mut buckets = Vec::with_capacity(capacity);
        buckets.resize_with(capacity, || None);
        
        HashTable {
            buckets,
            n_buckets_used: 0,
            n_entries: 0,
            tuning,
        }
    }

    pub fn len(&self) -> usize {
        self.n_entries
    }

    pub fn is_empty(&self) -> bool {
        self.n_entries == 0
    }

    pub fn capacity(&self) -> usize {
        self.buckets.len()
    }

    pub fn buckets_used(&self) -> usize {
        self.n_buckets_used
    }

    pub fn max_bucket_length(&self) -> usize {
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

    pub fn contains(&self, value: &T) -> bool 
    where
        T: PartialEq,
    {
        let index = self.hash(value) % self.buckets.len();
        let mut current = &self.buckets[index];
        while let Some(entry) = current {
            if &entry.data == value {
                return true;
            }
            current = &entry.next;
        }
        false
    }

    pub fn insert(&mut self, value: T) -> bool 
    where
        T: PartialEq + Clone,
    {
        if self.should_grow() {
            self.resize(self.new_size());
        }

        let index = self.hash(&value) % self.buckets.len();
        
        if let Some(bucket) = &mut self.buckets[index] {
            let mut current = bucket;
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
            self.n_buckets_used += 1;
        }
        
        self.n_entries += 1;
        true
    }

    pub fn remove(&mut self, value: &T) -> bool 
    where
        T: PartialEq,
    {
        let index = self.hash(value) % self.buckets.len();
        
        if let Some(bucket) = &mut self.buckets[index] {
            if bucket.data == *value {
                self.buckets[index] = bucket.next.take();
                self.n_entries -= 1;
                if self.buckets[index].is_none() {
                    self.n_buckets_used -= 1;
                }
                if self.should_shrink() {
                    self.resize(self.new_size());
                }
                return true;
            }
            
            let mut prev = bucket;
            while let Some(current) = &mut prev.next {
                if current.data == *value {
                    prev.next = current.next.take();
                    self.n_entries -= 1;
                    if self.should_shrink() {
                        self.resize(self.new_size());
                    }
                    return true;
                }
                prev = prev.next.as_mut().unwrap();
            }
        }
        
        false
    }

    fn hash(&self, value: &T) -> usize 
    where
        T: Hash,
    {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish() as usize
    }

    fn should_grow(&self) -> bool {
        self.n_buckets_used as f32 > self.tuning.growth_threshold * self.buckets.len() as f32
    }

    fn should_shrink(&self) -> bool {
        self.n_buckets_used as f32 < self.tuning.shrink_threshold * self.buckets.len() as f32
    }

    fn new_size(&self) -> usize {
        if self.tuning.is_n_buckets {
            (self.buckets.len() as f32 * self.tuning.growth_factor) as usize
        } else {
            (self.buckets.len() as f32 * self.tuning.growth_factor * self.tuning.growth_threshold) as usize
        }
    }

    fn resize(&mut self, new_size: usize) {
        if new_size == 0 {
            return;
        }

        let mut new_table = HashTable::with_tuning(new_size, self.tuning.clone());
        
        for bucket in &mut self.buckets {
            let mut current = bucket.take();
            while let Some(mut entry) = current {
                new_table.insert(entry.data);
                current = entry.next.take();
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