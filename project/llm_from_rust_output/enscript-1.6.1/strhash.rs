use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ptr;

struct HashList {
    next: Option<Box<HashList>>,
    key: Vec<u8>,
    data: Option<Box<dyn std::any::Any>>,
}

pub struct StringHash {
    hash_table: Vec<Option<Box<HashList>>>,
    next_idx: usize,
    next_item: Option<*mut HashList>,
}

impl StringHash {
    pub fn new() -> Option<Box<Self>> {
        let mut hash_table = Vec::with_capacity(8192);
        for _ in 0..8192 {
            hash_table.push(None);
        }
        Some(Box::new(Self {
            hash_table,
            next_idx: 0,
            next_item: None,
        }))
    }

    pub fn put(
        &mut self,
        key: &[u8],
        data: Option<Box<dyn std::any::Any>>,
    ) -> Option<Option<Box<dyn std::any::Any>>> {
        if key.is_empty() {
            return None;
        }

        let pos = self.count_hash(key);
        let mut prev = None;
        let mut current = &mut self.hash_table[pos];

        while let Some(node) = current {
            match node.key.len().cmp(&key.len()) {
                std::cmp::Ordering::Equal => {
                    if node.key == key {
                        let old_data = node.data.take();
                        node.data = data;
                        return Some(old_data);
                    } else if node.key < key {
                        break;
                    }
                }
                std::cmp::Ordering::Greater => break,
                _ => (),
            }
            prev = Some(node);
            current = &mut prev.unwrap().next;
        }

        let mut new_node = Box::new(HashList {
            next: current.take(),
            key: key.to_vec(),
            data,
        });

        if let Some(prev_node) = prev {
            prev_node.next = Some(new_node);
        } else {
            self.hash_table[pos] = Some(new_node);
        }

        Some(None)
    }

    pub fn get(&self, key: &[u8]) -> Option<&dyn std::any::Any> {
        if key.is_empty() {
            return None;
        }

        let pos = self.count_hash(key);
        let mut current = &self.hash_table[pos];

        while let Some(node) = current {
            match node.key.len().cmp(&key.len()) {
                std::cmp::Ordering::Equal => {
                    if node.key == key {
                        return node.data.as_ref().map(|d| &**d);
                    } else if node.key < key {
                        break;
                    }
                }
                std::cmp::Ordering::Greater => break,
                _ => (),
            }
            current = &node.next;
        }

        None
    }

    pub fn delete(&mut self, key: &[u8]) -> Option<Option<Box<dyn std::any::Any>>> {
        if key.is_empty() {
            return None;
        }

        let pos = self.count_hash(key);
        let mut prev = None;
        let mut current = &mut self.hash_table[pos];

        while let Some(node) = current {
            match node.key.len().cmp(&key.len()) {
                std::cmp::Ordering::Equal => {
                    if node.key == key {
                        let old_data = node.data.take();
                        *current = node.next.take();
                        self.next_idx = 0;
                        self.next_item = None;
                        return Some(old_data);
                    } else if node.key < key {
                        break;
                    }
                }
                std::cmp::Ordering::Greater => break,
                _ => (),
            }
            prev = Some(node);
            current = &mut prev.unwrap().next;
        }

        None
    }

    pub fn get_first(
        &mut self,
    ) -> Option<(&[u8], Option<&dyn std::any::Any>)> {
        self.next_idx = 0;
        while self.next_idx < 8192 {
            if let Some(node) = &self.hash_table[self.next_idx] {
                self.next_item = Some(ptr::NonNull::from(&**node).as_ptr();
                return Some((&node.key, node.data.as_ref().map(|d| &**d)));
            }
            self.next_idx += 1;
        }
        None
    }

    pub fn get_next(
        &mut self,
    ) -> Option<(&[u8], Option<&dyn std::any::Any>)> {
        while self.next_idx < 8192 {
            unsafe {
                if let Some(mut item) = self.next_item {
                    if let Some(next) = &(*item).next {
                        self.next_item = Some(ptr::NonNull::from(&**next).as_ptr());
                        return Some((&next.key, next.data.as_ref().map(|d| &**d)));
                    }
                }
                self.next_idx += 1;
                if self.next_idx < 8192 {
                    if let Some(node) = &self.hash_table[self.next_idx] {
                        self.next_item = Some(ptr::NonNull::from(&**node).as_ptr());
                        return Some((&node.key, node.data.as_ref().map(|d| &**d)));
                    }
                }
            }
        }
        None
    }

    fn count_hash(&self, key: &[u8]) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        (hasher.finish() % 8192) as usize
    }
}