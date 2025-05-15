use std::collections::LinkedList;
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
        Self {
            shrink_threshold: DEFAULT_SHRINK_THRESHOLD,
            shrink_factor: DEFAULT_SHRINK_FACTOR,
            growth_threshold: DEFAULT_GROWTH_THRESHOLD,
            growth_factor: DEFAULT_GROWTH_FACTOR,
            is_n_buckets: false,
        }
    }
}

pub struct HashTable<T: Hash + Eq> {
    buckets: Vec<LinkedList<T>>,
    n_buckets_used: usize,
    n_entries: usize,
    tuning: HashTuning,
}

impl<T: Hash + Eq> HashTable<T> {
    pub fn new(candidate: usize, tuning: Option<HashTuning>) -> Self {
        let tuning = tuning.unwrap_or_default();
        let n_buckets = Self::compute_bucket_size(candidate, &tuning);
        Self {
            buckets: vec![LinkedList::new(); n_buckets],
            n_buckets_used: 0,
            n_entries: 0,
            tuning,
        }
    }

    fn compute_bucket_size(candidate: usize, tuning: &HashTuning) -> usize {
        if !tuning.is_n_buckets {
            let new_candidate = (candidate as f32 / tuning.growth_threshold) as usize;
            new_candidate.next_prime()
        } else {
            candidate.next_prime()
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
        self.buckets.iter().map(|bucket| bucket.len()).max().unwrap_or(0)
    }

    pub fn is_ok(&self) -> bool {
        let mut n_buckets_used = 0;
        let mut n_entries = 0;

        for bucket in &self.buckets {
            if !bucket.is_empty() {
                n_buckets_used += 1;
                n_entries += bucket.len();
            }
        }

        n_buckets_used == self.n_buckets_used && n_entries == self.n_entries
    }

    pub fn lookup(&self, entry: &T) -> Option<&T> {
        let index = self.hash(entry);
        self.buckets[index].iter().find(|&e| e == entry)
    }

    pub fn get_first(&self) -> Option<&T> {
        self.buckets.iter().find(|bucket| !bucket.is_empty()).and_then(|bucket| bucket.front())
    }

    pub fn get_next(&self, entry: &T) -> Option<&T> {
        let index = self.hash(entry);
        let bucket = &self.buckets[index];
        
        let mut found = false;
        for e in bucket {
            if found {
                return Some(e);
            }
            if e == entry {
                found = true;
            }
        }

        if found {
            for bucket in &self.buckets[index + 1..] {
                if !bucket.is_empty() {
                    return bucket.front();
                }
            }
        }

        None
    }

    pub fn get_entries(&self, buffer: &mut Vec<&T>, buffer_size: usize) -> usize {
        let mut counter = 0;
        for bucket in &self.buckets {
            for entry in bucket {
                if counter >= buffer_size {
                    return counter;
                }
                buffer.push(entry);
                counter += 1;
            }
        }
        counter
    }

    pub fn do_for_each<F>(&self, mut processor: F) -> usize
    where
        F: FnMut(&T) -> bool,
    {
        let mut counter = 0;
        for bucket in &self.buckets {
            for entry in bucket {
                if !processor(entry) {
                    return counter;
                }
                counter += 1;
            }
        }
        counter
    }

    pub fn clear(&mut self) {
        for bucket in &mut self.buckets {
            bucket.clear();
        }
        self.n_buckets_used = 0;
        self.n_entries = 0;
    }

    pub fn rehash(&mut self, candidate: usize) -> bool {
        let new_size = Self::compute_bucket_size(candidate, &self.tuning);
        if new_size == self.buckets.len() {
            return true;
        }

        let mut new_table = Self::new(new_size, Some(self.tuning.clone()));
        for bucket in &self.buckets {
            for entry in bucket {
                if !new_table.insert(entry.clone()) {
                    return false;
                }
            }
        }

        *self = new_table;
        true
    }

    pub fn insert(&mut self, entry: T) -> bool {
        if self.n_buckets_used as f32 > self.tuning.growth_threshold * self.buckets.len() as f32 {
            let candidate = if self.tuning.is_n_buckets {
                (self.buckets.len() as f32 * self.tuning.growth_factor) as usize
            } else {
                (self.buckets.len() as f32 * self.tuning.growth_factor * self.tuning.growth_threshold) as usize
            };

            if !self.rehash(candidate) {
                return false;
            }
        }

        let index = self.hash(&entry);
        let bucket = &mut self.buckets[index];
        if bucket.is_empty() {
            self.n_buckets_used += 1;
        }
        if !bucket.iter().any(|e| e == &entry) {
            bucket.push_back(entry);
            self.n_entries += 1;
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, entry: &T) -> Option<T> {
        let index = self.hash(entry);
        let bucket = &mut self.buckets[index];
        let mut split = bucket.split_off(0);
        let mut found = None;

        while let Some(e) = split.pop_front() {
            if &e == entry {
                found = Some(e);
                self.n_entries -= 1;
                break;
            } else {
                bucket.push_back(e);
            }
        }

        if bucket.is_empty() {
            self.n_buckets_used -= 1;
        }

        if self.n_buckets_used as f32 < self.tuning.shrink_threshold * self.buckets.len() as f32 {
            let candidate = if self.tuning.is_n_buckets {
                (self.buckets.len() as f32 * self.tuning.shrink_factor) as usize
            } else {
                (self.buckets.len() as f32 * self.tuning.shrink_factor * self.tuning.growth_threshold) as usize
            };

            self.rehash(candidate);
        }

        found
    }

    fn hash(&self, entry: &T) -> usize {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        entry.hash(&mut hasher);
        (hasher.finish() as usize) % self.buckets.len()
    }
}

trait Prime {
    fn is_prime(&self) -> bool;
    fn next_prime(&self) -> Self;
}

impl Prime for usize {
    fn is_prime(&self) -> bool {
        if *self < 2 {
            return false;
        }
        if *self % 2 == 0 {
            return *self == 2;
        }
        let mut divisor = 3;
        while divisor * divisor <= *self {
            if *self % divisor == 0 {
                return false;
            }
            divisor += 2;
        }
        true
    }

    fn next_prime(&self) -> Self {
        let mut candidate = if *self < 2 { 2 } else { *self + 1 };
        while !candidate.is_prime() {
            candidate += 1;
        }
        candidate
    }
}