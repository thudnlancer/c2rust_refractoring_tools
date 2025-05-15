use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr;

type HashFun<T> = fn(&T) -> u64;
type TestFun<T> = fn(&T, &T) -> bool;

struct HashTable<K, V> {
    hash_function: HashFun<K>,
    test_function: TestFun<K>,
    cells: Vec<Option<(K, V)>>,
    size: usize,
    count: usize,
    resize_threshold: usize,
}

impl<K: Eq + Hash, V> HashTable<K, V> {
    fn new(items: usize, hash_function: HashFun<K>, test_function: TestFun<K>) -> Self {
        let size = Self::prime_size((1.0 + items as f64 / 0.75) as usize);
        let resize_threshold = (size as f64 * 0.75) as usize;
        
        Self {
            hash_function,
            test_function,
            cells: vec![None; size],
            size,
            count: 0,
            resize_threshold,
        }
    }

    fn prime_size(mut size: usize) -> usize {
        const PRIMES: [usize; 71] = [
            13, 19, 29, 41, 59, 79, 107, 149, 197, 263, 347, 457, 599, 787, 1031, 1361,
            1777, 2333, 3037, 3967, 5167, 6719, 8737, 11369, 14783, 19219, 24989, 32491,
            42257, 54941, 71429, 92861, 120721, 156941, 204047, 265271, 344857, 448321,
            582821, 757693, 985003, 1280519, 1664681, 2164111, 2813353, 3657361, 4754591,
            6180989, 8035301, 10445899, 13579681, 17653589, 22949669, 29834603, 38784989,
            50420551, 65546729, 85210757, 110774011, 144006217, 187208107, 243370577,
            316381771, 411296309, 534685237, 695090819, 903618083, 1174703521, 1527114613,
            1837299131, 2147483647,
        ];

        for &prime in PRIMES.iter() {
            if prime >= size {
                return prime;
            }
        }
        panic!("No suitable prime found for size {}", size);
    }

    fn find_cell(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        (self.hash_function)(key).hash(&mut hasher);
        let mut index = hasher.finish() as usize % self.size;

        while let Some((k, _)) = &self.cells[index] {
            if (self.test_function)(key, k) {
                break;
            }
            index = (index + 1) % self.size;
        }
        index
    }

    fn get(&self, key: &K) -> Option<&V> {
        let index = self.find_cell(key);
        self.cells[index].as_ref().map(|(_, v)| v)
    }

    fn contains(&self, key: &K) -> bool {
        let index = self.find_cell(key);
        self.cells[index].is_some()
    }

    fn grow(&mut self) {
        let old_cells = std::mem::take(&mut self.cells);
        self.size = Self::prime_size(self.size * 2);
        self.resize_threshold = (self.size as f64 * 0.75) as usize;
        self.cells = vec![None; self.size];
        self.count = 0;

        for (k, v) in old_cells.into_iter().flatten() {
            self.put(k, v);
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.count >= self.resize_threshold {
            self.grow();
        }

        let index = self.find_cell(&key);
        if self.cells[index].is_none() {
            self.count += 1;
        }
        self.cells[index] = Some((key, value));
    }

    fn remove(&mut self, key: &K) -> bool {
        let index = self.find_cell(key);
        if self.cells[index].is_some() {
            self.cells[index] = None;
            self.count -= 1;
            
            // Rehash remaining elements
            let mut i = (index + 1) % self.size;
            while self.cells[i].is_some() {
                let entry = self.cells[i].take().unwrap();
                let new_index = self.find_cell(&entry.0);
                if new_index != i {
                    self.cells[new_index] = Some(entry);
                } else {
                    self.cells[i] = Some(entry);
                }
                i = (i + 1) % self.size;
            }
            true
        } else {
            false
        }
    }

    fn clear(&mut self) {
        self.cells.fill(None);
        self.count = 0;
    }

    fn len(&self) -> usize {
        self.count
    }
}

// String-specific implementations
fn hash_string(key: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

fn cmp_string(s1: &str, s2: &str) -> bool {
    s1 == s2
}

fn make_string_hash_table(items: usize) -> HashTable<String, ()> {
    HashTable::new(items, hash_string, cmp_string)
}

// Case-insensitive string implementations
fn hash_string_nocase(key: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.to_lowercase().hash(&mut hasher);
    hasher.finish()
}

fn string_cmp_nocase(s1: &str, s2: &str) -> bool {
    s1.eq_ignore_ascii_case(s2)
}

fn make_nocase_string_hash_table(items: usize) -> HashTable<String, ()> {
    HashTable::new(items, hash_string_nocase, string_cmp_nocase)
}

// Pointer implementations
fn hash_pointer<T>(ptr: &T) -> u64 {
    ptr as *const T as u64
}

fn cmp_pointer<T>(ptr1: &T, ptr2: &T) -> bool {
    ptr::eq(ptr1, ptr2)
}