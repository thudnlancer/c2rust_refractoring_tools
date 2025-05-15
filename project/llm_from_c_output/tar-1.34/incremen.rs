use std::cmp::Ordering;
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::mem;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr;
use std::time::{SystemTime, UNIX_EPOCH};

use libc::{dev_t, ino_t, time_t};

const DIRF_INIT: u32 = 0x0001;
const DIRF_NFS: u32 = 0x0002;
const DIRF_FOUND: u32 = 0x0004;
const DIRF_NEW: u32 = 0x0008;
const DIRF_RENAMED: u32 = 0x0010;

const PD_FORCE_CHILDREN: u32 = 0x10;
const PD_FORCE_INIT: u32 = 0x20;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Children {
    NoChildren,
    ChangedChildren,
    AllChildren,
}

#[derive(Debug)]
struct DumpDir {
    contents: Vec<u8>,
    total: usize,
    elc: usize,
    elv: Vec<*mut u8>,
}

impl DumpDir {
    fn new(contents: &[u8]) -> Self {
        let mut total = 0;
        let mut elc = 0;
        let mut ctsize = 1;
        
        let mut q = contents;
        while !q.is_empty() {
            let len = unsafe { CStr::from_ptr(q.as_ptr() as *const i8).to_bytes().len() } + 1;
            ctsize += len;
            total += 1;
            q = &q[len..];
        }
        
        let mut dump = unsafe {
            let layout = std::alloc::Layout::new::<DumpDir>()
                .extend(std::alloc::Layout::from_size_align(ctsize, 1).unwrap())
                .unwrap()
                .0;
            let ptr = std::alloc::alloc(layout) as *mut DumpDir;
            ptr.write(DumpDir {
                contents: Vec::new(),
                total,
                elc,
                elv: Vec::new(),
            });
            (*ptr).contents = Vec::from_raw_parts(ptr.add(1) as *mut u8, ctsize, ctsize);
            &mut *ptr
        };
        
        unsafe {
            ptr::copy_nonoverlapping(contents.as_ptr(), dump.contents.as_mut_ptr(), ctsize);
        }
        
        dump.elv = vec![ptr::null_mut(); dump.elc + 1];
        
        let mut p = dump.contents.as_mut_ptr();
        let mut i = 0;
        while unsafe { *p != 0 } {
            let len = unsafe { CStr::from_ptr(p as *const i8).to_bytes().len() };
            if true { // TODO: cmask check
                dump.elv[i] = p.add(1);
                i += 1;
            }
            p = p.add(len + 1);
        }
        dump.elv[i] = ptr::null_mut();
        
        DumpDir {
            contents: unsafe { Vec::from_raw_parts(dump.contents.as_mut_ptr(), ctsize, ctsize) },
            total: dump.total,
            elc: dump.elc,
            elv: dump.elv,
        }
    }
    
    fn locate(&self, name: &str) -> Option<*mut u8> {
        if self.elv.is_empty() {
            return None;
        }
        
        let key = CString::new(name).unwrap();
        let key_ptr = key.as_ptr();
        
        unsafe {
            let mut base = 0;
            let mut nmemb = self.elc;
            
            while nmemb > 0 {
                let half = nmemb / 2;
                let mid = base + half;
                let mid_ptr = self.elv[mid];
                let cmp = libc::strcmp(key_ptr, mid_ptr as *const i8);
                
                match cmp.cmp(&0) {
                    Ordering::Less => nmemb = half,
                    Ordering::Greater => {
                        base = mid + 1;
                        nmemb -= half + 1;
                    }
                    Ordering::Equal => return Some(mid_ptr.sub(1)),
                }
            }
        }
        
        None
    }
}

struct DumpDirIter<'a> {
    dump: &'a DumpDir,
    all: bool,
    next: usize,
}

impl<'a> DumpDirIter<'a> {
    fn new(dump: &'a DumpDir, all: bool) -> Self {
        DumpDirIter { dump, all, next: 0 }
    }
    
    fn next(&mut self) -> Option<*mut u8> {
        if self.all {
            if self.next >= self.dump.contents.len() {
                return None;
            }
            
            let ret = unsafe { self.dump.contents.as_ptr().add(self.next) as *mut u8 };
            let len = unsafe { CStr::from_ptr(ret as *const i8).to_bytes().len() } + 1;
            self.next += len;
            
            Some(ret)
        } else {
            if self.next >= self.dump.elc {
                return None;
            }
            
            let ret = unsafe { self.dump.elv[self.next].sub(1) };
            self.next += 1;
            
            Some(ret)
        }
    }
}

#[derive(Debug)]
struct Directory {
    next: Option<Box<Directory>>,
    mtime: SystemTime,
    device_number: dev_t,
    inode_number: ino_t,
    dump: Option<Box<DumpDir>>,
    idump: Option<Box<DumpDir>>,
    children: Children,
    flags: u32,
    orig: Option<Box<Directory>>,
    tagfile: Option<CString>,
    caname: CString,
    name: CString,
}

impl Directory {
    fn new(name: &str, caname: CString) -> Self {
        let mut namelen = name.len();
        if namelen > 1 && name.ends_with('/') {
            namelen -= 1;
        }
        
        let name_cstr = CString::new(&name[..namelen]).unwrap();
        
        Directory {
            next: None,
            mtime: UNIX_EPOCH,
            device_number: 0,
            inode_number: 0,
            dump: None,
            idump: None,
            children: Children::ChangedChildren,
            flags: 0,
            orig: None,
            tagfile: None,
            caname,
            name: name_cstr,
        }
    }
    
    fn is_inited(&self) -> bool {
        self.flags & DIRF_INIT != 0
    }
    
    fn is_nfs(&self) -> bool {
        self.flags & DIRF_NFS != 0
    }
    
    fn is_found(&self) -> bool {
        self.flags & DIRF_FOUND != 0
    }
    
    fn is_renamed(&self) -> bool {
        self.flags & DIRF_RENAMED != 0
    }
    
    fn set_flag(&mut self, flag: u32) {
        self.flags |= flag;
    }
    
    fn clear_flag(&mut self, flag: u32) {
        self.flags &= !flag;
    }
}

struct DirectoryTable {
    by_name: HashMap<CString, Box<Directory>>,
    by_meta: HashMap<(dev_t, ino_t), Box<Directory>>,
    head: Option<Box<Directory>>,
    tail: Option<*mut Directory>,
}

impl DirectoryTable {
    fn new() -> Self {
        DirectoryTable {
            by_name: HashMap::new(),
            by_meta: HashMap::new(),
            head: None,
            tail: None,
        }
    }
    
    fn insert(&mut self, dir: Box<Directory>) {
        let caname = dir.caname.clone();
        let device = dir.device_number;
        let inode = dir.inode_number;
        
        if let Some(tail) = self.tail {
            unsafe {
                (*tail).next = Some(dir);
                self.tail = Some((*tail).next.as_mut().unwrap().as_mut());
            }
        } else {
            self.head = Some(dir);
            self.tail = Some(self.head.as_mut().unwrap().as_mut());
        }
        
        self.by_name.insert(caname, self.head.as_mut().unwrap().clone());
        self.by_meta.insert((device, inode), self.head.as_mut().unwrap().clone());
    }
    
    fn find_by_name(&self, name: &str) -> Option<&Directory> {
        let caname = normalize_filename(name);
        self.by_name.get(&caname).map(|d| &**d)
    }
    
    fn find_by_meta(&self, dev: dev_t, ino: ino_t) -> Option<&Directory> {
        self.by_meta.get(&(dev, ino)).map(|d| &**d)
    }
}

fn normalize_filename(name: &str) -> CString {
    // TODO: Implement proper normalization
    CString::new(name).unwrap()
}

fn main() {
    // TODO: Implement main functionality
}