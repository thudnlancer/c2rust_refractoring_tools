use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::mem;
use std::ptr;

type HashFunc<T> = fn(&T) -> u64;
type HashCmpFunc<T> = fn(&T, &T) -> Ordering;
type HashMapFunc<T> = fn(&T);
type HashMapArgFunc<T, A> = fn(&T, &mut A);

struct HashTable<T> {
    vec: Vec<Option<T>>,
    hash_1: HashFunc<T>,
    hash_2: HashFunc<T>,
    compare: HashCmpFunc<T>,
    size: u64,
    capacity: u64,
    fill: u64,
    empty_slots: u64,
    collisions: u64,
    lookups: u64,
    rehashes: u32,
}

impl<T> HashTable<T> {
    fn new(size: u64, hash_1: HashFunc<T>, hash_2: HashFunc<T>, compare: HashCmpFunc<T>) -> Self {
        let size = round_up_2(size);
        let capacity = size - (size / 16); // 93.75% loading factor
        HashTable {
            vec: vec![None; size as usize],
            hash_1,
            hash_2,
            compare,
            size,
            capacity,
            fill: 0,
            empty_slots: size,
            collisions: 0,
            lookups: 0,
            rehashes: 0,
        }
    }

    fn load(&mut self, items: &[T]) where T: Clone {
        for item in items {
            self.insert(item.clone());
        }
    }

    fn find_slot(&mut self, key: &T) -> &mut Option<T> {
        self.lookups += 1;
        let mut hash_1 = (self.hash_1)(key);
        let mut hash_2 = 0;
        let mut deleted_slot: Option<&mut Option<T>> = None;

        loop {
            hash_1 &= self.size - 1;
            let slot = &mut self.vec[hash_1 as usize];

            match slot {
                None => return deleted_slot.unwrap_or(slot),
                Some(item) => {
                    if (self.compare)(key, item) == Ordering::Equal {
                        return slot;
                    }
                    self.collisions += 1;
                }
            }

            if hash_2 == 0 {
                hash_2 = (self.hash_2)(key) | 1;
            }
            hash_1 += hash_2;
        }
    }

    fn find_item(&mut self, key: &T) -> Option<&T> {
        let slot = self.find_slot(key);
        slot.as_ref()
    }

    fn insert(&mut self, item: T) -> Option<T> {
        let slot = self.find_slot(&item);
        let old_item = slot.take();
        *slot = Some(item);
        if old_item.is_none() {
            self.fill += 1;
            self.empty_slots -= 1;
        }
        if self.empty_slots < self.size - self.capacity {
            self.rehash();
        }
        old_item
    }

    fn delete(&mut self, key: &T) -> Option<T> {
        let slot = self.find_slot(key);
        let item = slot.take();
        if item.is_some() {
            self.fill -= 1;
        }
        item
    }

    fn rehash(&mut self) {
        let old_size = self.size;
        let old_vec = mem::replace(&mut self.vec, vec![None; (self.size * 2) as usize]);
        self.size *= 2;
        self.capacity = self.size - (self.size >> 4);
        self.rehashes += 1;

        for item in old_vec.into_iter().flatten() {
            let slot = self.find_slot(&item);
            *slot = Some(item);
        }
        self.empty_slots = self.size - self.fill;
    }

    fn map<F>(&self, mut f: F) where F: FnMut(&T) {
        for item in self.vec.iter().flatten() {
            f(item);
        }
    }

    fn map_arg<F, A>(&self, mut f: F, arg: &mut A) where F: FnMut(&T, &mut A) {
        for item in self.vec.iter().flatten() {
            f(item, arg);
        }
    }
}

fn round_up_2(mut n: u64) -> u64 {
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n |= n >> 32;
    n + 1
}

fn jhash(key: &[u8]) -> u32 {
    let mut a = 0xdeadbeef + key.len() as u32;
    let mut b = a;
    let mut c = a;

    let mut i = 0;
    while i + 12 <= key.len() {
        a += u32::from_le_bytes([key[i], key[i+1], key[i+2], key[i+3]]);
        b += u32::from_le_bytes([key[i+4], key[i+5], key[i+6], key[i+7]]);
        c += u32::from_le_bytes([key[i+8], key[i+9], key[i+10], key[i+11]]);
        jhash_mix(&mut a, &mut b, &mut c);
        i += 12;
    }

    if i < key.len() {
        if i + 4 <= key.len() {
            a += u32::from_le_bytes([key[i], key[i+1], key[i+2], key[i+3]]);
            i += 4;
        }
        if i + 4 <= key.len() {
            b += u32::from_le_bytes([key[i], key[i+1], key[i+2], key[i+3]]);
            i += 4;
        }
        if i < key.len() {
            c += (key[i] as u32) << 24;
            if i + 1 < key.len() {
                c += (key[i+1] as u32) << 16;
                if i + 2 < key.len() {
                    c += (key[i+2] as u32) << 8;
                    if i + 3 < key.len() {
                        c += key[i+3] as u32;
                    }
                }
            }
        }
    }

    jhash_final(&mut a, &mut b, &mut c);
    c
}

fn jhash_string(key: &str) -> u32 {
    let mut a = 0xdeadbeef;
    let mut b = a;
    let mut c = a;
    let mut len = 0;

    for chunk in key.as_bytes().chunks(12) {
        a += chunk.iter().take(4).fold(0, |acc, &x| (acc << 8) | x as u32);
        b += chunk.iter().skip(4).take(4).fold(0, |acc, &x| (acc << 8) | x as u32);
        c += chunk.iter().skip(8).take(4).fold(0, |acc, &x| (acc << 8) | x as u32);
        jhash_mix(&mut a, &mut b, &mut c);
        len += chunk.len();
    }

    jhash_final(&mut a, &mut b, &mut c);
    c + len as u32
}

fn jhash_mix(a: &mut u32, b: &mut u32, c: &mut u32) {
    *a = a.wrapping_sub(*c);
    *a ^= c.rotate_left(4);
    *c = c.wrapping_add(*b);
    *b = b.wrapping_sub(*a);
    *b ^= a.rotate_left(6);
    *a = a.wrapping_add(*c);
    *c = c.wrapping_sub(*b);
    *c ^= b.rotate_left(8);
    *b = b.wrapping_add(*a);
    *a = a.wrapping_sub(*c);
    *a ^= c.rotate_left(16);
    *c = c.wrapping_add(*b);
    *b = b.wrapping_sub(*a);
    *b ^= a.rotate_left(19);
    *a = a.wrapping_add(*c);
    *c = c.wrapping_sub(*b);
    *c ^= b.rotate_left(4);
    *b = b.wrapping_add(*a);
}

fn jhash_final(a: &mut u32, b: &mut u32, c: &mut u32) {
    *c ^= *b;
    *c = c.wrapping_sub(b.rotate_left(14));
    *a ^= *c;
    *a = a.wrapping_sub(c.rotate_left(11));
    *b ^= *a;
    *b = b.wrapping_sub(a.rotate_left(25));
    *c ^= *b;
    *c = c.wrapping_sub(b.rotate_left(16));
    *a ^= *c;
    *a = a.wrapping_sub(c.rotate_left(4));
    *b ^= *a;
    *b = b.wrapping_sub(a.rotate_left(14));
    *c ^= *b;
    *c = c.wrapping_sub(b.rotate_left(24));
}