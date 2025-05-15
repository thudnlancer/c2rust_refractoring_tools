use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr;
use std::mem;
use std::cmp;

const HASH_MAX_FULLNESS: f64 = 0.75;
const HASH_RESIZE_FACTOR: usize = 2;

struct Cell<K, V> {
    key: Option<K>,
    value: Option<V>,
}

pub struct HashTable<K, V> {
    hash_function: fn(&K) -> u64,
    test_function: fn(&K, &K) -> bool,
    cells: Vec<Cell<K, V>>,
    size: usize,
    count: usize,
    resize_threshold: usize,
    prime_offset: usize,
}

pub struct HashTableIterator<'a, K, V> {
    pos: usize,
    end: usize,
    table: &'a HashTable<K, V>,
}

impl<K, V> HashTable<K, V>
where
    K: Eq + Hash,
{
    pub fn new(items: usize) -> Self {
        let primes = [
            13, 19, 29, 41, 59, 79, 107, 149, 197, 263, 347, 457, 599, 787, 1031,
            1361, 1777, 2333, 3037, 3967, 5167, 6719, 8737, 11369, 14783,
            19219, 24989, 32491, 42257, 54941, 71429, 92861, 120721, 156941,
            204047, 265271, 344857, 448321, 582821, 757693, 985003, 1280519,
            1664681, 2164111, 2813353, 3657361, 4754591, 6180989, 8035301,
            10445899, 13579681, 17653589, 22949669, 29834603, 38784989,
            50420551, 65546729, 85210757, 110774011, 144006217, 187208107,
            243370577, 316381771, 411296309, 534685237, 695090819, 903618083,
            1174703521, 1527114613, 1837299131, 2147483647
        ];

        let mut prime_offset = 0;
        let size = (1.0 + items as f64 / HASH_MAX_FULLNESS) as usize;
        let size = primes.iter()
            .skip_while(|&&p| p < size)
            .next()
            .copied()
            .unwrap_or(primes[primes.len() - 1]);

        let mut cells = Vec::with_capacity(size);
        for _ in 0..size {
            cells.push(Cell { key: None, value: None });
        }

        HashTable {
            hash_function: |k| {
                let mut hasher = DefaultHasher::new();
                k.hash(&mut hasher);
                hasher.finish()
            },
            test_function: |a, b| a == b,
            cells,
            size,
            count: 0,
            resize_threshold: (size as f64 * HASH_MAX_FULLNESS) as usize,
            prime_offset,
        }
    }

    fn find_cell(&self, key: &K) -> usize {
        let hash = (self.hash_function)(key);
        let mut pos = (hash % self.size as u64) as usize;
        
        while let Some(cell) = self.cells.get(pos) {
            match &cell.key {
                Some(k) if (self.test_function)(k, key) => break,
                None => break,
                _ => pos = (pos + 1) % self.size,
            }
        }
        pos
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        let pos = self.find_cell(key);
        self.cells[pos].value.as_ref()
    }

    pub fn contains(&self, key: &K) -> bool {
        let pos = self.find_cell(key);
        self.cells[pos].key.is_some()
    }

    fn grow(&mut self) {
        let primes = [
            13, 19, 29, 41, 59, 79, 107, 149, 197, 263, 347, 457, 599, 787, 1031,
            1361, 1777, 2333, 3037, 3967, 5167, 6719, 8737, 11369, 14783,
            19219, 24989, 32491, 42257, 54941, 71429, 92861, 120721, 156941,
            204047, 265271, 344857, 448321, 582821, 757693, 985003, 1280519,
            1664681, 2164111, 2813353, 3657361, 4754591, 6180989, 8035301,
            10445899, 13579681, 17653589, 22949669, 29834603, 38784989,
            50420551, 65546729, 85210757, 110774011, 144006217, 187208107,
            243370577, 316381771, 411296309, 534685237, 695090819, 903618083,
            1174703521, 1527114613, 1837299131, 2147483647
        ];

        let new_size = self.size * HASH_RESIZE_FACTOR;
        let new_size = primes.iter()
            .skip_while(|&&p| p < new_size)
            .next()
            .copied()
            .unwrap_or(primes[primes.len() - 1]);

        let mut new_cells = Vec::with_capacity(new_size);
        for _ in 0..new_size {
            new_cells.push(Cell { key: None, value: None });
        }

        let old_cells = mem::replace(&mut self.cells, new_cells);
        self.size = new_size;
        self.resize_threshold = (new_size as f64 * HASH_MAX_FULLNESS) as usize;
        self.count = 0;

        for cell in old_cells {
            if let Some(key) = cell.key {
                self.put(key, cell.value.unwrap());
            }
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if self.count >= self.resize_threshold {
            self.grow();
        }

        let pos = self.find_cell(&key);
        if self.cells[pos].key.is_none() {
            self.count += 1;
        }
        self.cells[pos] = Cell {
            key: Some(key),
            value: Some(value),
        };
    }

    pub fn remove(&mut self, key: &K) -> Option<V> {
        let pos = self.find_cell(key);
        if self.cells[pos].key.is_none() {
            return None;
        }

        let value = self.cells[pos].value.take();
        self.cells[pos].key = None;
        self.count -= 1;

        // Rehash following cells
        let mut next_pos = (pos + 1) % self.size;
        while self.cells[next_pos].key.is_some() {
            let cell_key = self.cells[next_pos].key.take().unwrap();
            let cell_value = self.cells[next_pos].value.take().unwrap();
            
            let new_pos = self.find_cell(&cell_key);
            if new_pos != next_pos {
                self.cells[new_pos] = Cell {
                    key: Some(cell_key),
                    value: Some(cell_value),
                };
            } else {
                self.cells[next_pos] = Cell {
                    key: Some(cell_key),
                    value: Some(cell_value),
                };
            }
            
            next_pos = (next_pos + 1) % self.size;
        }

        value
    }

    pub fn clear(&mut self) {
        for cell in &mut self.cells {
            cell.key = None;
            cell.value = None;
        }
        self.count = 0;
    }

    pub fn iter(&self) -> HashTableIterator<K, V> {
        HashTableIterator {
            pos: 0,
            end: self.size,
            table: self,
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

impl<'a, K, V> Iterator for HashTableIterator<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.pos < self.end {
            let cell = &self.table.cells[self.pos];
            self.pos += 1;
            if let (Some(key), Some(value)) = (&cell.key, &cell.value) {
                return Some((key, value));
            }
        }
        None
    }
}

// String-specific implementations
pub fn hash_string(key: &str) -> u64 {
    let mut h = 0;
    for &b in key.as_bytes() {
        h = h.wrapping_mul(31).wrapping_add(b as u64);
    }
    h
}

pub fn make_string_hash_table(items: usize) -> HashTable<String, String> {
    HashTable {
        hash_function: |k| hash_string(k),
        test_function: |a, b| a == b,
        cells: Vec::with_capacity(items),
        size: 0,
        count: 0,
        resize_threshold: 0,
        prime_offset: 0,
    }
}

pub fn make_nocase_string_hash_table(items: usize) -> HashTable<String, String> {
    HashTable {
        hash_function: |k| {
            let mut h = 0;
            for c in k.chars() {
                h = h.wrapping_mul(31).wrapping_add(c.to_ascii_lowercase() as u64);
            }
            h
        },
        test_function: |a, b| a.eq_ignore_ascii_case(b),
        cells: Vec::with_capacity(items),
        size: 0,
        count: 0,
        resize_threshold: 0,
        prime_offset: 0,
    }
}