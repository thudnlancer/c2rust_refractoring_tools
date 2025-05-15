use std::alloc::{alloc, dealloc, Layout};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ptr;

const HASH_DELETED_ITEM: *mut () = ptr::null_mut();

struct HashTable<T> {
    vec: Vec<Option<T>>,
    size: usize,
    capacity: usize,
    fill: usize,
    empty_slots: usize,
    collisions: usize,
    lookups: usize,
    rehashes: u32,
    hash1: fn(&T) -> u64,
    hash2: fn(&T) -> u64,
    compare: fn(&T, &T) -> Ordering,
}

impl<T> HashTable<T> {
    fn new(
        size: usize,
        hash1: fn(&T) -> u64,
        hash2: fn(&T) -> u64,
        compare: fn(&T, &T) -> Ordering,
    ) -> Self {
        let rounded_size = round_up_2(size);
        let capacity = rounded_size - (rounded_size / 16);
        let vec = vec![None; rounded_size];

        HashTable {
            vec,
            size: rounded_size,
            capacity,
            fill: 0,
            empty_slots: rounded_size,
            collisions: 0,
            lookups: 0,
            rehashes: 0,
            hash1,
            hash2,
            compare,
        }
    }

    fn find_slot(&mut self, key: &T) -> usize {
        let mut hash1 = (self.hash1)(key) as usize;
        let mut hash2 = 0;
        let mut deleted_slot = None;

        self.lookups += 1;

        loop {
            hash1 = hash1 & (self.size - 1);
            let slot = hash1;

            match self.vec[slot] {
                None => return deleted_slot.unwrap_or(slot),
                Some(ref item) if ptr::eq(item as *const _, HASH_DELETED_ITEM as *const _) => {
                    if deleted_slot.is_none() {
                        deleted_slot = Some(slot);
                    }
                }
                Some(ref item) => {
                    if ptr::eq(item as *const _, key as *const _)
                        || (self.compare)(item, key) == Ordering::Equal
                    {
                        return slot;
                    }
                    self.collisions += 1;
                }
            }

            if hash2 == 0 {
                hash2 = ((self.hash2)(key) | 1) as usize;
            }
            hash1 = hash1.wrapping_add(hash2);
        }
    }

    fn find_item(&mut self, key: &T) -> Option<&T> {
        let slot = self.find_slot(key);
        self.vec[slot].as_ref().filter(|&item| !ptr::eq(item as *const _, HASH_DELETED_ITEM as *const _))
    }

    fn insert(&mut self, item: T) -> Option<T> {
        let slot = self.find_slot(&item);
        let old_item = self.vec[slot].take();

        self.insert_at(slot, item);

        old_item.filter(|item| !ptr::eq(item as *const _, HASH_DELETED_ITEM as *const _))
    }

    fn insert_at(&mut self, slot: usize, item: T) {
        let old_item = self.vec[slot].take();

        if old_item.is_none() || ptr::eq(old_item.as_ref().unwrap() as *const _, HASH_DELETED_ITEM as *const _) {
            self.fill += 1;
            if old_item.is_none() {
                self.empty_slots -= 1;
            }
        }

        self.vec[slot] = Some(item);

        if self.empty_slots < self.size - self.capacity {
            self.rehash();
        }
    }

    fn delete(&mut self, key: &T) -> Option<T> {
        let slot = self.find_slot(key);
        self.delete_at(slot)
    }

    fn delete_at(&mut self, slot: usize) -> Option<T> {
        if let Some(item) = self.vec[slot].take() {
            if !ptr::eq(&item as *const _, HASH_DELETED_ITEM as *const _) {
                self.vec[slot] = Some(unsafe { mem::transmute(HASH_DELETED_ITEM) });
                self.fill -= 1;
                return Some(item);
            }
        }
        None
    }

    fn rehash(&mut self) {
        let old_size = self.size;
        let old_vec = mem::replace(&mut self.vec, vec![None; self.size * 2]);

        self.size *= 2;
        self.capacity = self.size - (self.size / 16);
        self.rehashes += 1;
        self.vec = vec![None; self.size];

        for item in old_vec.into_iter().flatten() {
            if !ptr::eq(&item as *const _, HASH_DELETED_ITEM as *const _) {
                let slot = self.find_slot(&item);
                self.vec[slot] = Some(item);
            }
        }

        self.empty_slots = self.size - self.fill;
    }
}

fn round_up_2(mut n: usize) -> usize {
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    n + 1
}

fn jhash(k: &[u8]) -> u32 {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0xdeadbeef + k.len() as u32;

    let mut chunks = k.chunks_exact(12);
    for chunk in chunks.by_ref() {
        let mut vals = chunk.chunks_exact(4).map(|c| {
            let mut val = [0; 4];
            val.copy_from_slice(c);
            u32::from_le_bytes(val)
        });

        a += vals.next().unwrap();
        b += vals.next().unwrap();
        c += vals.next().unwrap();

        a = a.wrapping_sub(c);
        a ^= c.rotate_left(4);
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a.rotate_left(6);
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b.rotate_left(8);
        b = b.wrapping_add(a);
        a = a.wrapping_sub(c);
        a ^= c.rotate_left(16);
        c = c.wrapping_add(b);
        b = b.wrapping_sub(a);
        b ^= a.rotate_left(19);
        a = a.wrapping_add(c);
        c = c.wrapping_sub(b);
        c ^= b.rotate_left(4);
        b = b.wrapping_add(a);
    }

    let remainder = chunks.remainder();
    if !remainder.is_empty() {
        let mut val = [0; 4];
        let len = remainder.len().min(4);
        val[..len].copy_from_slice(&remainder[..len]);
        a += u32::from_le_bytes(val);

        if remainder.len() > 4 {
            let mut val = [0; 4];
            let len = (remainder.len() - 4).min(4);
            val[..len].copy_from_slice(&remainder[4..4 + len]);
            b += u32::from_le_bytes(val);
        }

        if remainder.len() > 8 {
            let mut val = [0; 4];
            let len = remainder.len() - 8;
            val[..len].copy_from_slice(&remainder[8..8 + len]);
            c += u32::from_le_bytes(val);
        }
    }

    c ^= b;
    c = c.wrapping_sub(b.rotate_left(14));
    a ^= c;
    a = a.wrapping_sub(c.rotate_left(11));
    b ^= a;
    b = b.wrapping_sub(a.rotate_left(25));
    c ^= b;
    c = c.wrapping_sub(b.rotate_left(16));
    a ^= c;
    a = a.wrapping_sub(c.rotate_left(4));
    b ^= a;
    b = b.wrapping_sub(a.rotate_left(14));
    c ^= b;
    c = c.wrapping_sub(b.rotate_left(24));

    c
}

fn jhash_string(s: &str) -> u32 {
    jhash(s.as_bytes())
}