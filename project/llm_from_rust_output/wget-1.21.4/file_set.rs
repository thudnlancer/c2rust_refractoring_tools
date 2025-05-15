use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_void};
use std::ptr;
use std::mem;

#[derive(Debug, Clone, Copy)]
pub struct Timespec {
    pub tv_sec: i64,
    pub tv_nsec: i64,
}

#[derive(Debug, Clone, Copy)]
pub struct Stat {
    pub st_dev: u64,
    pub st_ino: u64,
    pub st_nlink: u64,
    pub st_mode: u32,
    pub st_uid: u32,
    pub st_gid: u32,
    pub __pad0: i32,
    pub st_rdev: u64,
    pub st_size: i64,
    pub st_blksize: i64,
    pub st_blocks: i64,
    pub st_atim: Timespec,
    pub st_mtim: Timespec,
    pub st_ctim: Timespec,
    pub __glibc_reserved: [i64; 3],
}

#[derive(Debug, Clone)]
pub struct FTriple {
    pub name: CString,
    pub st_ino: u64,
    pub st_dev: u64,
}

impl FTriple {
    pub fn new(name: &CStr, stats: &Stat) -> Self {
        FTriple {
            name: name.to_owned(),
            st_ino: stats.st_ino,
            st_dev: stats.st_dev,
        }
    }
}

pub struct HashTable {
    // Wrapper around the C hash table implementation
    inner: *mut libc::c_void,
}

impl HashTable {
    pub fn new() -> Option<Self> {
        // Implementation would depend on the C library's hash table creation function
        None
    }

    pub fn insert(&mut self, entry: FTriple) -> Result<(), ()> {
        unsafe {
            let ent = Box::into_raw(Box::new(entry));
            let ent_from_table = hash_insert(self.inner, ent as *const c_void) as *mut FTriple;
            
            if ent_from_table.is_null() {
                return Err(());
            }
            
            if ent_from_table != ent {
                Box::from_raw(ent);
            }
            Ok(())
        }
    }

    pub fn lookup(&self, key: &FTriple) -> Option<&FTriple> {
        unsafe {
            let found = hash_lookup(self.inner, key as *const FTriple as *const c_void) as *mut FTriple;
            if found.is_null() {
                None
            } else {
                Some(&*found)
            }
        }
    }
}

// These would need to be implemented or linked to the C library
extern "C" {
    fn hash_insert(table: *mut libc::c_void, entry: *const c_void) -> *mut c_void;
    fn hash_lookup(table: *const libc::c_void, entry: *const c_void) -> *mut c_void;
}

pub fn record_file(ht: &mut HashTable, file: &CStr, stats: &Stat) -> Result<(), ()> {
    let ent = FTriple::new(file, stats);
    ht.insert(ent)
}

pub fn seen_file(ht: &HashTable, file: &CStr, stats: &Stat) -> bool {
    let key = FTriple::new(file, stats);
    ht.lookup(&key).is_some()
}