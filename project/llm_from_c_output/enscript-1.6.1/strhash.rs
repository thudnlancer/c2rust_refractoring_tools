use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr;

const HASH_SIZE: usize = 8192;

struct HashEntry {
    key: Vec<u8>,
    data: *mut std::ffi::c_void,
    next: Option<Box<HashEntry>>,
}

pub struct StringHash {
    table: Vec<Option<Box<HashEntry>>>,
    next_idx: usize,
    next_item: Option<*mut HashEntry>,
}

impl StringHash {
    pub fn new() -> Option<Box<Self>> {
        let mut table = Vec::with_capacity(HASH_SIZE);
        for _ in 0..HASH_SIZE {
            table.push(None);
        }

        Some(Box::new(Self {
            table,
            next_idx: 0,
            next_item: None,
        }))
    }

    pub fn put(
        &mut self,
        key: &[u8],
        data: *mut std::ffi::c_void,
        old_data: &mut Option<*mut std::ffi::c_void>,
    ) -> bool {
        if key.is_empty() {
            return false;
        }

        *old_data = None;
        let pos = self.hash(key);

        let mut prev: Option<&mut Box<HashEntry>> = None;
        let mut current = &mut self.table[pos];

        while let Some(entry) = current {
            match entry.key.len().cmp(&key.len()) {
                std::cmp::Ordering::Equal => {
                    if entry.key == key {
                        *old_data = Some(entry.data);
                        entry.data = data;
                        return true;
                    } else if entry.key > key {
                        break;
                    }
                }
                std::cmp::Ordering::Greater => break,
                _ => (),
            }
            prev = Some(entry);
            current = &mut entry.next;
        }

        let mut new_entry = Box::new(HashEntry {
            key: key.to_vec(),
            data,
            next: None,
        });

        if let Some(prev_entry) = prev {
            new_entry.next = prev_entry.next.take();
            prev_entry.next = Some(new_entry);
        } else {
            new_entry.next = self.table[pos].take();
            self.table[pos] = Some(new_entry);
        }

        true
    }

    pub fn get(&self, key: &[u8], data: &mut Option<*mut std::ffi::c_void>) -> bool {
        if key.is_empty() {
            return false;
        }

        *data = None;
        let pos = self.hash(key);

        let mut current = &self.table[pos];
        while let Some(entry) = current {
            match entry.key.len().cmp(&key.len()) {
                std::cmp::Ordering::Equal => {
                    if entry.key == key {
                        *data = Some(entry.data);
                        return true;
                    } else if entry.key > key {
                        break;
                    }
                }
                std::cmp::Ordering::Greater => break,
                _ => (),
            }
            current = &entry.next;
        }

        false
    }

    pub fn delete(
        &mut self,
        key: &[u8],
        data: &mut Option<*mut std::ffi::c_void>,
    ) -> bool {
        if key.is_empty() {
            return false;
        }

        *data = None;
        let pos = self.hash(key);

        let mut prev: Option<&mut Box<HashEntry>> = None;
        let mut current = &mut self.table[pos];

        while let Some(entry) = current {
            match entry.key.len().cmp(&key.len()) {
                std::cmp::Ordering::Equal => {
                    if entry.key == key {
                        *data = Some(entry.data);
                        if let Some(prev_entry) = prev {
                            prev_entry.next = entry.next.take();
                        } else {
                            *current = entry.next.take();
                        }

                        self.next_idx = 0;
                        self.next_item = None;
                        return true;
                    } else if entry.key > key {
                        break;
                    }
                }
                std::cmp::Ordering::Greater => break,
                _ => (),
            }
            prev = Some(entry);
            current = &mut entry.next;
        }

        false
    }

    pub fn get_first(
        &mut self,
        key: &mut Option<Vec<u8>>,
        key_len: &mut usize,
        data: &mut Option<*mut std::ffi::c_void>,
    ) -> bool {
        self.next_idx = 0;
        self.next_item = None;

        for i in 0..HASH_SIZE {
            if let Some(entry) = &self.table[i] {
                *key = Some(entry.key.clone());
                *key_len = entry.key.len();
                *data = Some(entry.data);
                self.next_idx = i + 1;
                self.next_item = Some(unsafe { ptr::NonNull::from(entry.as_ref()).as_ptr() });
                return true;
            }
        }

        false
    }

    pub fn get_next(
        &mut self,
        key: &mut Option<Vec<u8>>,
        key_len: &mut usize,
        data: &mut Option<*mut std::ffi::c_void>,
    ) -> bool {
        if let Some(mut item) = self.next_item {
            unsafe {
                if let Some(next) = &(*item).next {
                    *key = Some(next.key.clone());
                    *key_len = next.key.len();
                    *data = Some(next.data);
                    self.next_item = Some(ptr::NonNull::from(next.as_ref()).as_ptr());
                    return true;
                }
            }

            self.next_item = None;
        }

        for i in self.next_idx..HASH_SIZE {
            if let Some(entry) = &self.table[i] {
                *key = Some(entry.key.clone());
                *key_len = entry.key.len();
                *data = Some(entry.data);
                self.next_idx = i + 1;
                self.next_item = Some(unsafe { ptr::NonNull::from(entry.as_ref()).as_ptr() });
                return true;
            }
        }

        false
    }

    fn hash(&self, key: &[u8]) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() as usize) % HASH_SIZE
    }
}

impl Drop for StringHash {
    fn drop(&mut self) {
        for entry in &mut self.table {
            let mut current = entry.take();
            while let Some(mut e) = current {
                current = e.next.take();
            }
        }
    }
}