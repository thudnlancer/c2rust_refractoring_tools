use std::alloc::{alloc, dealloc, Layout};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::ptr;

const SIZES: [usize; 30] = [
    5, 11, 23, 47, 97, 197, 397, 797, 1597, 3203, 6421, 12853, 25717, 51437, 102877, 205759,
    411527, 823117, 1646237, 3292489, 6584983, 13169977, 26339969, 52679969, 105359939, 210719881,
    421439783, 842879579, 1685759167, 0,
];

#[derive(Debug)]
enum Entry<T> {
    Unallocated,
    Deleted,
    Occupied(T),
}

pub struct HashTable<T> {
    hash1: fn(&T) -> u64,
    hash2: fn(&T) -> u64,
    compar: fn(&T, &T) -> bool,
    size: usize,
    fill: usize,
    inuse: usize,
    max: usize,
    entries: Vec<Entry<T>>,
}

impl<T> HashTable<T> {
    pub fn new(
        hash1: fn(&T) -> u64,
        hash2: fn(&T) -> u64,
        compar: fn(&T, &T) -> bool,
        size: usize,
    ) -> Option<Self> {
        let mut ht = HashTable {
            hash1,
            hash2,
            compar,
            size: 0,
            fill: 0,
            inuse: 0,
            max: 0,
            entries: Vec::new(),
        };
        if ht.alloc_ht(size).is_err() {
            return None;
        }
        Some(ht)
    }

    fn alloc_ht(&mut self, size: usize) -> Result<(), ()> {
        let mut i = 0;
        while i < SIZES.len() && SIZES[i] != 0 {
            if SIZES[i] > size * 4 {
                break;
            }
            i += 1;
        }

        if SIZES[i] == 0 {
            i = 0;
            while i < SIZES.len() && SIZES[i] != 0 {
                if SIZES[i] > size * 2 {
                    break;
                }
                i += 1;
            }
        }

        if SIZES[i] == 0 {
            i = 0;
            while i < SIZES.len() && SIZES[i] != 0 {
                if SIZES[i] > size {
                    break;
                }
                i += 1;
            }
        }

        if SIZES[i] == 0 {
            return Err(());
        }

        let mut size = SIZES[i];
        if size < self.size {
            size = self.size;
        }

        self.max = size * 4 / 5 - 2;
        self.size = size;
        self.fill = 0;
        self.inuse = 0;
        self.entries = vec![Entry::Unallocated; size];

        Ok(())
    }

    pub fn add(&mut self, item: T) -> Result<usize, ()> {
        if self.fill >= self.max {
            self.rehash()?;
        }
        if self.fill == self.size {
            return Err(());
        }
        self.mt_hash_add(item)
    }

    fn mt_hash_add(&mut self, item: T) -> Result<usize, ()> {
        let hash1 = (self.hash1)(&item);
        let mut pos = (hash1 as usize) % self.size;
        let hash2 = (self.hash2)(&item);
        let mut f2 = (hash2 as usize) % (self.size - 1);
        let mut ctr = 0;

        while let Entry::Occupied(_) = self.entries[pos] {
            pos = (pos + f2 + 1) % self.size;
            ctr += 1;
            if ctr > self.size {
                return Err(());
            }
        }

        if let Entry::Unallocated = self.entries[pos] {
            self.fill += 1;
        }
        self.inuse += 1;
        self.entries[pos] = Entry::Occupied(item);
        Ok(pos)
    }

    fn rehash(&mut self) -> Result<(), ()> {
        let old_entries = std::mem::replace(&mut self.entries, Vec::new());
        let old_size = self.size;

        let new_size = (self.inuse + 1) * 4 + self.fill / 5;
        if self.alloc_ht(new_size).is_err() {
            self.entries = old_entries;
            return Err(());
        }

        for entry in old_entries {
            if let Entry::Occupied(item) = entry {
                self.mt_hash_add(item)?;
            }
        }

        Ok(())
    }

    pub fn lookup(&self, item: &T) -> Option<(usize, &T)> {
        self.mt_hash_lookup(item, false)
    }

    fn mt_hash_lookup(&self, item: &T, is_identity: bool) -> Option<(usize, &T)> {
        let hash1 = (self.hash1)(item);
        let mut pos = (hash1 as usize) % self.size;
        let hash2 = (self.hash2)(item);
        let mut f2 = (hash2 as usize) % (self.size - 1);
        let mut ttl = self.size;
        let mut upos = None;

        while ttl > 0 {
            match &self.entries[pos] {
                Entry::Unallocated => break,
                Entry::Deleted => {
                    if upos.is_none() {
                        upos = Some(pos);
                    }
                }
                Entry::Occupied(existing) => {
                    if is_identity && ptr::eq(existing as *const _, item as *const _) {
                        break;
                    }
                    if !is_identity && !(self.compar)(existing, item) {
                        break;
                    }
                }
            }

            pos = (pos + f2 + 1) % self.size;
            ttl -= 1;
        }

        if ttl == 0 || matches!(self.entries[pos], Entry::Unallocated) {
            return None;
        }

        if let Some(upos) = upos {
            // In Rust we can't easily do the swap like in C, would need unsafe
            // So we just return the found position
        }

        if let Entry::Occupied(item) = &self.entries[pos] {
            Some((pos, item))
        } else {
            None
        }
    }

    pub fn remove(&mut self, item: &T, hint: Option<usize>) -> Result<(), ()> {
        if let Some(hint) = hint {
            if hint < self.size {
                if let Entry::Occupied(existing) = &self.entries[hint] {
                    if ptr::eq(existing as *const _, item as *const _) {
                        self.inuse -= 1;
                        self.entries[hint] = Entry::Deleted;
                        return Ok(());
                    }
                }
            }
        }

        if let Some((pos, _)) = self.mt_hash_lookup(item, true) {
            self.inuse -= 1;
            self.entries[pos] = Entry::Deleted;
            Ok(())
        } else {
            Err(())
        }
    }
}

impl<T> Drop for HashTable<T> {
    fn drop(&mut self) {
        // No need for explicit free in Rust, Vec will be dropped automatically
    }
}