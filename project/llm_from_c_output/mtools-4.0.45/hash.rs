use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

type HashFunc = fn(&dyn std::any::Any) -> u64;
type ComparFunc = fn(&dyn std::any::Any, &dyn std::any::Any) -> bool;

struct HashTable {
    f1: HashFunc,
    f2: HashFunc,
    compar: ComparFunc,
    size: usize,
    fill: usize,
    inuse: usize,
    max: usize,
    entries: Vec<Option<Box<dyn std::any::Any>>>,
}

static DELETED: usize = 0;
static UNALLOCATED: usize = 1;

impl HashTable {
    fn new(f1: HashFunc, f2: HashFunc, compar: ComparFunc, size: usize) -> Result<Self, &'static str> {
        let mut ht = HashTable {
            f1,
            f2,
            compar,
            size: 0,
            fill: 0,
            inuse: 0,
            max: 0,
            entries: Vec::new(),
        };
        ht.alloc(size)?;
        Ok(ht)
    }

    fn alloc(&mut self, size: usize) -> Result<(), &'static str> {
        const SIZES: [usize; 30] = [
            5, 11, 23, 47, 97, 197, 397, 797, 1597, 3203, 6421, 12853,
            25717, 51437, 102877, 205759, 411527, 823117, 1646237,
            3292489, 6584983, 13169977, 26339969, 52679969, 105359939,
            210719881, 421439783, 842879579, 1685759167, 0
        ];

        let mut new_size = 0;
        for &s in &SIZES {
            if s == 0 {
                break;
            }
            if s > size * 4 {
                new_size = s;
                break;
            }
        }

        if new_size == 0 {
            for &s in &SIZES {
                if s == 0 {
                    break;
                }
                if s > size * 2 {
                    new_size = s;
                    break;
                }
            }
        }

        if new_size == 0 {
            for &s in &SIZES {
                if s == 0 {
                    break;
                }
                if s > size {
                    new_size = s;
                    break;
                }
            }
        }

        if new_size == 0 {
            return Err("No suitable size found");
        }

        if new_size < self.size {
            new_size = self.size;
        }

        self.max = new_size * 4 / 5 - 2;
        self.size = new_size;
        self.fill = 0;
        self.inuse = 0;
        self.entries = vec![None; new_size];

        Ok(())
    }

    fn free(self) {
        // Rust's ownership system will automatically clean up when the HashTable goes out of scope
    }

    fn hash_add(&mut self, e: Box<dyn std::any::Any>, hint: Option<&mut usize>) -> Result<(), &'static str> {
        if self.fill >= self.max {
            self.rehash()?;
        }
        if self.fill == self.size {
            return Err("Out of memory");
        }
        self.mt_hash_add(e, hint)
    }

    fn mt_hash_add(&mut self, e: Box<dyn std::any::Any>, hint: Option<&mut usize>) -> Result<(), &'static str> {
        let mut pos = (self.f1)(&e) as usize % self.size;
        let mut f2 = None;
        let mut ctr = 0;

        while self.entries[pos].is_some() {
            if f2.is_none() {
                f2 = Some((self.f2)(&e) as usize % (self.size - 1));
            }
            pos = (pos + f2.unwrap() + 1) % self.size;
            ctr += 1;
        }

        if self.entries[pos].is_none() {
            self.fill += 1;
        }
        self.inuse += 1;
        self.entries[pos] = Some(e);
        if let Some(h) = hint {
            *h = pos;
        }
        Ok(())
    }

    fn rehash(&mut self) -> Result<(), &'static str> {
        let old_size = self.size;
        let old_entries = std::mem::take(&mut self.entries);

        self.alloc((self.inuse + 1) * 4 + self.fill / 5)?;

        for i in 0..old_size {
            if let Some(e) = old_entries[i] {
                self.mt_hash_add(e, None)?;
            }
        }

        Ok(())
    }

    fn hash_lookup(&self, e: &dyn std::any::Any, e2: &mut Option<Box<dyn std::any::Any>>, hint: Option<&mut usize>, is_identity: bool) -> Result<(), &'static str> {
        let mut pos = (self.f1)(e) as usize % self.size;
        let mut ttl = self.size;
        let mut f2 = None;
        let mut upos = None;

        while ttl > 0 && self.entries[pos].is_some() {
            let entry = self.entries[pos].as_ref().unwrap();
            if is_identity {
                if std::ptr::eq(entry.as_ref(), e) {
                    break;
                }
            } else if !(self.compar)(entry.as_ref(), e) {
                break;
            }

            if f2.is_none() {
                f2 = Some((self.f2)(e) as usize % (self.size - 1));
            }
            if upos.is_none() {
                upos = Some(pos);
            }
            pos = (pos + f2.unwrap() + 1) % self.size;
            ttl -= 1;
        }

        if self.entries[pos].is_none() || ttl == 0 {
            return Err("Entry not found");
        }

        if let Some(u) = upos {
            if let Some(h) = hint {
                *h = u;
            }
            *e2 = self.entries[pos].clone();
        } else {
            if let Some(h) = hint {
                *h = pos;
            }
            *e2 = self.entries[pos].clone();
        }

        Ok(())
    }

    fn hash_remove(&mut self, e: &dyn std::any::Any, hint: usize) -> Result<(), &'static str> {
        if hint < self.size {
            if let Some(entry) = &self.entries[hint] {
                if std::ptr::eq(entry.as_ref(), e) {
                    self.inuse -= 1;
                    self.entries[hint] = None;
                    return Ok(());
                }
            }
        }

        let mut e2 = None;
        let mut new_hint = 0;
        self.hash_lookup(e, &mut e2, Some(&mut new_hint), true)?;
        self.inuse -= 1;
        self.entries[new_hint] = None;
        Ok(())
    }
}

fn default_hash(e: &dyn std::any::Any) -> u64 {
    let mut hasher = DefaultHasher::new();
    e.hash(&mut hasher);
    hasher.finish()
}

fn default_compar(a: &dyn std::any::Any, b: &dyn std::any::Any) -> bool {
    a == b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_table() {
        let ht = HashTable::new(default_hash, default_hash, default_compar, 10).unwrap();
        assert_eq!(ht.size, 23); // Next prime > 10*2
    }
}