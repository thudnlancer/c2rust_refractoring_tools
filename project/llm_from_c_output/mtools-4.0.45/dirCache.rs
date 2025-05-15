use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::ptr;
use std::mem;
use std::slice;
use std::ops::BitOrAssign;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

const DC_BITMAP_SIZE: usize = 128;
const BITS_PER_INT: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq)]
enum DirCacheEntryType {
    Free,
    Used,
    End,
}

struct Directory {
    // Placeholder for directory structure
}

struct DirCacheEntry {
    entry_type: DirCacheEntryType,
    long_name: Option<Vec<u16>>,
    short_name: Vec<u16>,
    begin_slot: u32,
    end_slot: u32,
    end_mark_pos: i32,
    dir: Directory,
}

struct DirCache {
    entries: Vec<Option<Box<DirCacheEntry>>>,
    nr_entries: u32,
    nr_hashed: u32,
    bm0: [u32; DC_BITMAP_SIZE],
    bm1: [u32; DC_BITMAP_SIZE],
    bm2: [u32; DC_BITMAP_SIZE],
}

impl DirCache {
    fn new(initial_slots: u32) -> Option<Self> {
        if initial_slots as i32 < 0 {
            eprintln!("Bad slot {}", initial_slots);
            return None;
        }

        let capacity = (initial_slots + 1) * 2;
        let mut entries = Vec::with_capacity(capacity as usize);
        entries.resize_with(capacity as usize, || None);

        Some(Self {
            entries,
            nr_entries: capacity,
            nr_hashed: 0,
            bm0: [0; DC_BITMAP_SIZE],
            bm1: [0; DC_BITMAP_SIZE],
            bm2: [0; DC_BITMAP_SIZE],
        })
    }

    fn grow(&mut self, slot: u32) -> Result<(), ()> {
        if slot as i32 < 0 {
            eprintln!("Bad slot {}", slot);
            return Err(());
        }

        if self.nr_entries <= slot {
            let new_capacity = (slot + 1) * 2;
            self.entries.resize_with(new_capacity as usize, || None);
            self.nr_entries = new_capacity;
        }
        Ok(())
    }

    fn add_bit(bitmap: &mut [u32; DC_BITMAP_SIZE], hash: u32, check_only: bool) -> bool {
        let bit = 1u32 << (hash % BITS_PER_INT as u32);
        let entry = (hash / BITS_PER_INT as u32) as usize % DC_BITMAP_SIZE;

        if check_only {
            bitmap[entry] & bit != 0
        } else {
            bitmap[entry] |= bit;
            true
        }
    }

    fn add_hash(&mut self, hash: u32, check_only: bool) -> bool {
        Self::add_bit(&mut self.bm0, hash, check_only) &&
        Self::add_bit(&mut self.bm1, hash.rotate_left(12), check_only) &&
        Self::add_bit(&mut self.bm2, hash.rotate_left(24), check_only)
    }

    fn calc_hash(name: &[u16]) -> u32 {
        let mut hasher = DefaultHasher::new();
        name.hash(&mut hasher);
        hasher.finish() as u32
    }

    fn add_name_to_hash(&mut self, name: &[u16]) {
        let hash = Self::calc_hash(name);
        self.add_hash(hash, false);
    }

    fn hash_dce(&mut self, dce: &DirCacheEntry) {
        if dce.begin_slot != self.nr_hashed {
            return;
        }
        self.nr_hashed = dce.end_slot;
        if let Some(ref long_name) = dce.long_name {
            self.add_name_to_hash(long_name);
        }
        self.add_name_to_hash(&dce.short_name);
    }

    fn is_hashed(&mut self, name: &[u16]) -> bool {
        let hash = Self::calc_hash(name);
        self.add_hash(hash, true)
    }

    fn alloc_entry(
        &mut self,
        begin_slot: u32,
        end_slot: u32,
        entry_type: DirCacheEntryType,
    ) -> Option<Box<DirCacheEntry>> {
        if self.grow(end_slot).is_err() {
            return None;
        }

        let entry = Box::new(DirCacheEntry {
            entry_type,
            long_name: None,
            short_name: Vec::new(),
            begin_slot,
            end_slot,
            end_mark_pos: -1,
            dir: Directory {},
        });

        if self.free_range(begin_slot, end_slot).is_some() {
            for i in begin_slot..end_slot {
                self.entries[i as usize] = Some(entry.clone());
            }
            Some(entry)
        } else {
            None
        }
    }

    fn free_range(&mut self, begin_slot: u32, end_slot: u32) -> Option<i32> {
        if end_slot < begin_slot {
            eprintln!("Bad slots {} {} in free range", begin_slot, end_slot);
            return None;
        }

        let mut begin = begin_slot;
        let mut need_write_end = None;

        while begin < end_slot {
            let entry = match self.entries[begin as usize].as_ref() {
                Some(e) => e,
                None => {
                    begin += 1;
                    continue;
                }
            };

            let clear_end = std::cmp::min(entry.end_slot, end_slot);
            let clear_begin = begin;

            for i in clear_begin..clear_end {
                self.entries[i as usize] = None;
            }

            if entry.end_mark_pos != -1 && entry.end_mark_pos < begin as i32 {
                need_write_end = Some(begin as i32);
            }

            begin = clear_end;
        }

        need_write_end
    }

    fn add_used_entry(
        &mut self,
        begin_slot: u32,
        end_slot: u32,
        long_name: Option<Vec<u16>>,
        short_name: Vec<u16>,
        dir: Directory,
    ) -> Option<&DirCacheEntry> {
        if end_slot < begin_slot {
            eprintln!("Bad slots {} {} in add used entry", begin_slot, end_slot);
            return None;
        }

        let mut entry = self.alloc_entry(begin_slot, end_slot, DirCacheEntryType::Used)?;
        entry.long_name = long_name;
        entry.short_name = short_name;
        entry.dir = dir;
        self.hash_dce(&entry);

        let entry_ptr = &*entry as *const DirCacheEntry;
        self.entries[begin_slot as usize] = Some(entry);
        unsafe { Some(&*entry_ptr) }
    }

    fn merge_free_slots(&mut self, slot: u32) {
        if slot == 0 {
            return;
        }

        let prev_idx = (slot - 1) as usize;
        let curr_idx = slot as usize;

        if let (Some(prev), Some(curr)) = (
            self.entries[prev_idx].as_ref(),
            self.entries[curr_idx].as_ref(),
        ) {
            if prev.entry_type == DirCacheEntryType::Free && curr.entry_type == DirCacheEntryType::Free {
                for i in curr.begin_slot..curr.end_slot {
                    self.entries[i as usize] = self.entries[prev_idx].clone();
                }
                self.entries[prev_idx].as_mut().unwrap().end_slot = curr.end_slot;
                self.entries[prev_idx].as_mut().unwrap().end_mark_pos = curr.end_mark_pos;
                self.entries[curr_idx] = None;
            }
        }
    }

    fn add_free_end_entry(
        &mut self,
        begin_slot: u32,
        end_slot: u32,
        is_at_end: bool,
    ) -> Option<&DirCacheEntry> {
        if begin_slot < self.nr_hashed {
            self.nr_hashed = begin_slot;
        }

        if end_slot < begin_slot {
            eprintln!("Bad slots {} {} in add free entry", begin_slot, end_slot);
            return None;
        }

        if end_slot == begin_slot {
            return None;
        }

        let mut entry = self.alloc_entry(begin_slot, end_slot, DirCacheEntryType::Free)?;
        if is_at_end {
            entry.end_mark_pos = begin_slot as i32;
        }

        self.merge_free_slots(begin_slot);
        self.merge_free_slots(end_slot);

        let entry_ptr = &*entry as *const DirCacheEntry;
        self.entries[begin_slot as usize] = Some(entry);
        unsafe { Some(&*entry_ptr) }
    }

    fn add_free_entry(&mut self, begin_slot: u32, end_slot: u32) -> Option<&DirCacheEntry> {
        self.add_free_end_entry(begin_slot, end_slot, false)
    }

    fn add_end_entry(&mut self, pos: u32) -> Option<&DirCacheEntry> {
        self.alloc_entry(pos, pos + 1, DirCacheEntryType::End)
            .map(|e| {
                let entry_ptr = &*e as *const DirCacheEntry;
                self.entries[pos as usize] = Some(e);
                unsafe { &*entry_ptr }
            })
    }

    fn lookup(&mut self, pos: u32) -> Option<&DirCacheEntry> {
        if self.grow(pos + 1).is_err() {
            return None;
        }
        self.entries[pos as usize].as_ref().map(|e| &**e)
    }
}

struct Stream {
    dir_cache: Option<DirCache>,
}

impl Stream {
    fn get_dir_cache(&mut self) -> &mut Option<DirCache> {
        &mut self.dir_cache
    }

    fn free_dir_cache(&mut self) {
        if let Some(cache) = self.dir_cache.take() {
            if let Some(n) = cache.free_range(0, cache.nr_entries) {
                // low_level_dir_write_end(self, n);
            }
        }
    }
}

fn alloc_dir_cache(stream: &mut Stream, slot: u32) -> Option<&mut DirCache> {
    if slot as i32 < 0 {
        eprintln!("Bad slot {}", slot);
        return None;
    }

    let cache = stream.get_dir_cache();
    if cache.is_none() {
        *cache = DirCache::new(slot);
    } else if let Some(c) = cache {
        if c.grow(slot).is_err() {
            return None;
        }
    }
    cache.as_mut()
}