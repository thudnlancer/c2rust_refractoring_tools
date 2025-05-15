use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

struct StrCache {
    next: Option<Box<StrCache>>,
    end: u16,
    bytesfree: u16,
    count: u16,
    buffer: Vec<u8>,
}

struct HugeString {
    next: Option<Box<HugeString>>,
    buffer: Vec<u8>,
}

pub struct StringCache {
    strcache: Option<Box<StrCache>>,
    fullcache: Option<Box<StrCache>>,
    hugestrings: Option<Box<HugeString>>,
    total_buffers: usize,
    total_strings: usize,
    total_size: usize,
    strings: HashMap<Vec<u8>, Vec<u8>>,
    total_adds: usize,
}

impl StringCache {
    pub fn new() -> Self {
        StringCache {
            strcache: None,
            fullcache: None,
            hugestrings: None,
            total_buffers: 0,
            total_strings: 0,
            total_size: 0,
            strings: HashMap::new(),
            total_adds: 0,
        }
    }

    fn new_cache(&mut self, buflen: u16) -> &mut StrCache {
        let buffer_size = buflen as usize + 14;
        let mut new = Box::new(StrCache {
            next: None,
            end: 0,
            count: 0,
            bytesfree: buflen,
            buffer: vec![0; buffer_size],
        });

        self.total_buffers += 1;
        new.next = self.strcache.take();
        let new_ref = &mut *new;
        self.strcache = Some(new);
        new_ref
    }

    fn copy_string(&mut self, sp: &mut StrCache, str: &[u8], len: u16) -> &[u8] {
        let start = sp.end as usize;
        let end = start + len as usize;
        sp.buffer[start..end].copy_from_slice(str);
        sp.buffer[end] = 0;
        sp.end = end as u16;
        sp.bytesfree -= len + 1;
        sp.count += 1;
        &sp.buffer[start..=end]
    }

    fn add_string(&mut self, str: &[u8]) -> &[u8] {
        let len = str.len() as u16;
        let sz = len + 1;
        self.total_strings += 1;
        self.total_size += sz as usize;

        if sz as usize > 8192 - 2 * mem::size_of::<usize>() - 14 {
            let sp = self.new_cache(sz);
            return self.copy_string(sp, str, len);
        }

        let mut current = &mut self.strcache;
        while let Some(ref mut cache) = current {
            if cache.bytesfree > sz {
                break;
            }
            current = &mut cache.next;
        }

        let sp = if let Some(cache) = current {
            cache
        } else {
            let default_size = (8192 - 2 * mem::size_of::<usize>() - 14) as u16;
            self.new_cache(default_size)
        };

        let res = self.copy_string(sp, str, len);

        if self.total_strings > 20 && sp.bytesfree as usize < self.total_size / self.total_strings + 1 {
            let mut taken = current.take();
            if let Some(mut cache) = taken {
                let next = cache.next.take();
                cache.next = self.fullcache.take();
                self.fullcache = Some(cache);
                *current = next;
            }
        }

        res
    }

    fn add_hugestring(&mut self, str: &[u8]) -> &[u8] {
        let len = str.len();
        let mut new = Box::new(HugeString {
            next: None,
            buffer: vec![0; len + 1],
        });
        new.buffer[..len].copy_from_slice(str);
        new.buffer[len] = 0;
        new.next = self.hugestrings.take();
        let new_ref = &mut *new;
        self.hugestrings = Some(new);
        &new_ref.buffer
    }

    pub fn add_hash(&mut self, str: &[u8]) -> &[u8] {
        if str.len() > 32767 * 2 {
            return self.add_hugestring(str);
        }

        self.total_adds += 1;

        if let Some(existing) = self.strings.get(str) {
            return existing;
        }

        let cached = if str.len() + 1 > 8192 - 2 * mem::size_of::<usize>() - 14 {
            self.add_hugestring(str)
        } else {
            self.add_string(str)
        };

        self.strings.insert(str.to_vec(), cached.to_vec());
        cached
    }

    pub fn is_cached(&self, str: &[u8]) -> bool {
        // Check strcache and fullcache
        let mut current = &self.strcache;
        while let Some(cache) = current {
            let start = cache.buffer.as_ptr() as usize;
            let end = start + cache.end as usize;
            let ptr = str.as_ptr() as usize;
            if ptr >= start && ptr < end {
                return true;
            }
            current = &cache.next;
        }

        // Check hugestrings
        let mut current = &self.hugestrings;
        while let Some(string) = current {
            if string.buffer.as_ptr() == str.as_ptr() {
                return true;
            }
            current = &string.next;
        }

        false
    }

    pub fn strcache_add(&mut self, str: &[u8]) -> &[u8] {
        self.add_hash(str)
    }

    pub fn strcache_add_len(&mut self, str: &[u8], len: usize) -> &[u8] {
        if len < str.len() && str[len] != 0 {
            let mut owned = str[..len].to_vec();
            owned.push(0);
            self.add_hash(&owned)
        } else {
            self.add_hash(str)
        }
    }

    pub fn init(&mut self) {
        self.strings = HashMap::with_capacity(8000);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache() {
        let mut cache = StringCache::new();
        cache.init();

        let s1 = b"test string";
        let cached = cache.strcache_add(s1);
        assert_eq!(s1, &cached[..s1.len()]);
        assert!(cache.is_cached(cached));

        let s2 = b"another string";
        let cached2 = cache.strcache_add_len(s2, 7);
        assert_eq!(b"another", &cached2[..7]);
        assert!(cache.is_cached(cached2));
    }
}