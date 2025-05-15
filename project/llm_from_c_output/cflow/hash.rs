use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::mem;
use std::fmt;
use std::error::Error;

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
            shrink_threshold: 0.0,
            shrink_factor: 1.0,
            growth_threshold: 0.8,
            growth_factor: 1.414,
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
    hasher: fn(&T, usize) -> usize,
    comparator: fn(&T, &T) -> bool,
}

impl<T> HashTable<T> {
    pub fn new(
        candidate: usize,
        tuning: Option<HashTuning>,
        hasher: fn(&T, usize) -> usize,
        comparator: fn(&T, &T) -> bool,
    ) -> Result<Self, Box<dyn Error>> {
        let tuning = tuning.unwrap_or_default();
        let n_buckets = Self::compute_bucket_size(candidate, &tuning)?;
        
        Ok(HashTable {
            buckets: vec![None; n_buckets],
            n_buckets_used: 0,
            n_entries: 0,
            tuning,
            hasher,
            comparator,
        })
    }

    fn compute_bucket_size(candidate: usize, tuning: &HashTuning) -> Result<usize, Box<dyn Error>> {
        let candidate = if !tuning.is_n_buckets {
            ((candidate as f32) / tuning.growth_threshold) as usize
        } else {
            candidate
        };
        
        let candidate = Self::next_prime(candidate.max(10) | 1);
        Ok(candidate)
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
        
        while square < candidate {
            if candidate % divisor == 0 || candidate % (divisor + 2) == 0 {
                return false;
            }
            divisor += 6;
            square = divisor * divisor;
        }
        
        true
    }

    fn next_prime(mut candidate: usize) -> usize {
        if candidate < 10 {
            return 10;
        }
        candidate |= 1;
        
        while !Self::is_prime(candidate) {
            candidate += 2;
        }
        
        candidate
    }

    pub fn insert(&mut self, entry: T) -> Result<(), Box<dyn Error>> {
        if self.n_buckets_used as f32 > self.tuning.growth_threshold * self.buckets.len() as f32 {
            let new_size = if self.tuning.is_n_buckets {
                (self.buckets.len() as f32 * self.tuning.growth_factor) as usize
            } else {
                (self.buckets.len() as f32 * self.tuning.growth_factor * self.tuning.growth_threshold) as usize
            };
            
            self.rehash(new_size)?;
        }

        let hash = (self.hasher)(&entry, self.buckets.len());
        let bucket = &mut self.buckets[hash];
        
        if bucket.is_none() {
            self.n_buckets_used += 1;
        }
        
        let new_entry = Box::new(HashEntry {
            data: entry,
            next: None,
        });
        
        if let Some(ref mut head) = bucket {
            let mut current = head;
            while current.next.is_some() {
                current = current.next.as_mut().unwrap();
            }
            current.next = Some(new_entry);
        } else {
            *bucket = Some(new_entry);
        }
        
        self.n_entries += 1;
        Ok(())
    }

    pub fn lookup(&self, entry: &T) -> Option<&T> {
        let hash = (self.hasher)(entry, self.buckets.len());
        
        if let Some(ref head) = self.buckets[hash] {
            let mut current = head;
            loop {
                if (self.comparator)(entry, &current.data) {
                    return Some(&current.data);
                }
                if let Some(ref next) = current.next {
                    current = next;
                } else {
                    break;
                }
            }
        }
        
        None
    }

    pub fn delete(&mut self, entry: &T) -> Option<T> {
        let hash = (self.hasher)(entry, self.buckets.len());
        
        if let Some(ref mut head) = self.buckets[hash] {
            if (self.comparator)(entry, &head.data) {
                let old_data = mem::replace(&mut head.data, unsafe { mem::zeroed() });
                let next = head.next.take();
                self.buckets[hash] = next;
                self.n_entries -= 1;
                if self.buckets[hash].is_none() {
                    self.n_buckets_used -= 1;
                    
                    if (self.n_buckets_used as f32) < (self.tuning.shrink_threshold * self.buckets.len() as f32) {
                        let new_size = if self.tuning.is_n_buckets {
                            (self.buckets.len() as f32 * self.tuning.shrink_factor) as usize
                        } else {
                            (self.buckets.len() as f32 * self.tuning.shrink_factor * self.tuning.growth_threshold) as usize
                        };
                        
                        let _ = self.rehash(new_size);
                    }
                }
                return Some(old_data);
            }
            
            let mut prev = head;
            while let Some(ref mut next) = prev.next {
                if (self.comparator)(entry, &next.data) {
                    let old_data = mem::replace(&mut next.data, unsafe { mem::zeroed() });
                    let next_next = next.next.take();
                    prev.next = next_next;
                    self.n_entries -= 1;
                    return Some(old_data);
                }
                prev = prev.next.as_mut().unwrap();
            }
        }
        
        None
    }

    fn rehash(&mut self, new_size: usize) -> Result<(), Box<dyn Error>> {
        let new_size = Self::compute_bucket_size(new_size, &self.tuning)?;
        if new_size == self.buckets.len() {
            return Ok(());
        }
        
        let mut new_table = HashTable {
            buckets: vec![None; new_size],
            n_buckets_used: 0,
            n_entries: 0,
            tuning: self.tuning.clone(),
            hasher: self.hasher,
            comparator: self.comparator,
        };
        
        for bucket in &mut self.buckets {
            while let Some(mut entry) = bucket.take() {
                let next = entry.next.take();
                let hash = (new_table.hasher)(&entry.data, new_table.buckets.len());
                
                if new_table.buckets[hash].is_none() {
                    new_table.n_buckets_used += 1;
                }
                
                entry.next = new_table.buckets[hash].take();
                new_table.buckets[hash] = Some(entry);
                new_table.n_entries += 1;
                
                *bucket = next;
            }
        }
        
        *self = new_table;
        Ok(())
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
            max_len = max_len.max(len);
        }
        max_len
    }

    pub fn is_ok(&self) -> bool {
        let mut n_buckets_used = 0;
        let mut n_entries = 0;
        
        for bucket in &self.buckets {
            let mut current = bucket;
            while let Some(entry) = current {
                if current == bucket {
                    n_buckets_used += 1;
                }
                n_entries += 1;
                current = &entry.next;
            }
        }
        
        n_buckets_used == self.n_buckets_used && n_entries == self.n_entries
    }
}

impl<T: fmt::Debug> fmt::Debug for HashTable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, bucket) in self.buckets.iter().enumerate() {
            if bucket.is_some() {
                writeln!(f, "{}:", i)?;
                let mut current = bucket;
                while let Some(entry) = current {
                    writeln!(f, "  {:?}", entry.data)?;
                    current = &entry.next;
                }
            }
        }
        Ok(())
    }
}

pub fn hash_string(s: &str, n_buckets: usize) -> usize {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    (hasher.finish() as usize) % n_buckets
}

pub fn raw_comparator<T>(a: &T, b: &T) -> bool {
    ptr::eq(a, b)
}