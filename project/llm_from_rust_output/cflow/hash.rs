use std::alloc::{alloc, dealloc, Layout};
use std::cmp::Ordering;
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

struct HashEntry {
    data: *mut libc::c_void,
    next: *mut HashEntry,
}

pub struct HashTable {
    buckets: Vec<*mut HashEntry>,
    n_buckets_used: usize,
    n_entries: usize,
    tuning: HashTuning,
    free_entry_list: *mut HashEntry,
}

impl HashTable {
    pub fn new(
        candidate: usize,
        tuning: Option<HashTuning>,
        hasher: Option<fn(*const libc::c_void, usize) -> usize>,
        comparator: Option<fn(*const libc::c_void, *const libc::c_void) -> bool>,
        data_freer: Option<fn(*mut libc::c_void)>,
    ) -> Option<Self> {
        let tuning = tuning.unwrap_or_default();
        let n_buckets = Self::compute_bucket_size(candidate, &tuning)?;
        
        let mut buckets = Vec::with_capacity(n_buckets);
        for _ in 0..n_buckets {
            buckets.push(ptr::null_mut());
        }

        Some(HashTable {
            buckets,
            n_buckets_used: 0,
            n_entries: 0,
            tuning,
            free_entry_list: ptr::null_mut(),
        })
    }

    fn compute_bucket_size(candidate: usize, tuning: &HashTuning) -> Option<usize> {
        let candidate = if !tuning.is_n_buckets {
            let new_candidate = candidate as f32 / tuning.growth_threshold;
            if new_candidate > usize::MAX as f32 {
                return None;
            }
            new_candidate as usize
        } else {
            candidate
        };
        
        Some(Self::next_prime(candidate))
    }

    fn next_prime(candidate: usize) -> usize {
        if candidate < 10 {
            10
        } else {
            let mut candidate = candidate | 1;
            while candidate != usize::MAX && !Self::is_prime(candidate) {
                candidate += 2;
            }
            candidate
        }
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
        while square < candidate && candidate % divisor != 0 {
            divisor += 2;
            square += 4 * divisor;
            divisor += 2;
        }
        candidate % divisor != 0
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
            let mut current = *bucket;
            while !current.is_null() {
                len += 1;
                current = unsafe { (*current).next };
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
            if !bucket.is_null() {
                n_buckets_used += 1;
                let mut current = *bucket;
                while !current.is_null() {
                    n_entries += 1;
                    current = unsafe { (*current).next };
                }
            }
        }
        
        n_buckets_used == self.n_buckets_used && n_entries == self.n_entries
    }

    pub fn print_statistics(&self, stream: *mut libc::FILE) {
        unsafe {
            libc::fprintf(
                stream,
                b"# entries:         %lu\n\0".as_ptr() as *const libc::c_char,
                self.n_entries,
            );
            libc::fprintf(
                stream,
                b"# buckets:         %lu\n\0".as_ptr() as *const libc::c_char,
                self.buckets.len(),
            );
            libc::fprintf(
                stream,
                b"# buckets used:    %lu (%.2f%%)\n\0".as_ptr() as *const libc::c_char,
                self.n_buckets_used,
                100.0 * self.n_buckets_used as f64 / self.buckets.len() as f64,
            );
            libc::fprintf(
                stream,
                b"max bucket length: %lu\n\0".as_ptr() as *const libc::c_char,
                self.get_max_bucket_length(),
            );
        }
    }

    pub fn lookup(&self, entry: *const libc::c_void) -> *mut libc::c_void {
        let hash = self.hash(entry);
        let mut current = self.buckets[hash];
        while !current.is_null() {
            if unsafe { (*current).data == entry as *mut libc::c_void } {
                return unsafe { (*current).data };
            }
            current = unsafe { (*current).next };
        }
        ptr::null_mut()
    }

    fn hash(&self, data: *const libc::c_void) -> usize {
        let mut hasher = DefaultHasher::new();
        (data as usize).hash(&mut hasher);
        hasher.finish() as usize % self.buckets.len()
    }

    pub fn clear(&mut self) {
        for bucket in &mut self.buckets {
            let mut current = *bucket;
            while !current.is_null() {
                let next = unsafe { (*current).next };
                unsafe { libc::free(current as *mut libc::c_void) };
                current = next;
            }
            *bucket = ptr::null_mut();
        }
        self.n_buckets_used = 0;
        self.n_entries = 0;
    }
}

impl Drop for HashTable {
    fn drop(&mut self) {
        self.clear();
    }
}