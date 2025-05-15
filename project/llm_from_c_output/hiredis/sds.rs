use std::mem;
use std::ptr;
use std::slice;
use std::os::raw::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::fmt;
use std::cmp;
use std::str;
use std::vec::Vec;
use std::ops::{Deref, DerefMut};
use std::convert::TryInto;

const SDS_MAX_PREALLOC: usize = 1024 * 1024;
const SDS_TYPE_MASK: u8 = 7;
const SDS_TYPE_BITS: u8 = 3;

#[repr(u8)]
enum SdsType {
    Type5 = 0,
    Type8 = 1,
    Type16 = 2,
    Type32 = 3,
    Type64 = 4,
}

#[repr(C, packed)]
struct Sdshdr5 {
    flags: u8,
    buf: [u8; 0],
}

#[repr(C, packed)]
struct Sdshdr8 {
    len: u8,
    alloc: u8,
    flags: u8,
    buf: [u8; 0],
}

#[repr(C, packed)]
struct Sdshdr16 {
    len: u16,
    alloc: u16,
    flags: u8,
    buf: [u8; 0],
}

#[repr(C, packed)]
struct Sdshdr32 {
    len: u32,
    alloc: u32,
    flags: u8,
    buf: [u8; 0],
}

#[repr(C, packed)]
struct Sdshdr64 {
    len: u64,
    alloc: u64,
    flags: u8,
    buf: [u8; 0],
}

pub struct Sds {
    ptr: *mut u8,
}

impl Sds {
    pub fn new(init: &str) -> Self {
        unsafe {
            let len = init.len();
            let type_ = sds_req_type(len);
            let hdrlen = sds_hdr_size(type_);
            
            let ptr = sds_malloc(hdrlen + len + 1) as *mut u8;
            if ptr.is_null() {
                panic!("Failed to allocate memory for Sds");
            }
            
            let buf_ptr = ptr.add(hdrlen);
            ptr::copy_nonoverlapping(init.as_ptr(), buf_ptr, len);
            *buf_ptr.add(len) = 0;
            
            match type_ {
                SdsType::Type5 => {
                    let fp = buf_ptr.sub(1);
                    *fp = (SdsType::Type5 as u8) | ((len as u8) << SDS_TYPE_BITS);
                }
                SdsType::Type8 => {
                    let sh = (buf_ptr as *mut u8).sub(mem::size_of::<Sdshdr8>()) as *mut Sdshdr8;
                    (*sh).len = len as u8;
                    (*sh).alloc = len as u8;
                    *buf_ptr.sub(1) = SdsType::Type8 as u8;
                }
                SdsType::Type16 => {
                    let sh = (buf_ptr as *mut u8).sub(mem::size_of::<Sdshdr16>()) as *mut Sdshdr16;
                    (*sh).len = len as u16;
                    (*sh).alloc = len as u16;
                    *buf_ptr.sub(1) = SdsType::Type16 as u8;
                }
                SdsType::Type32 => {
                    let sh = (buf_ptr as *mut u8).sub(mem::size_of::<Sdshdr32>()) as *mut Sdshdr32;
                    (*sh).len = len as u32;
                    (*sh).alloc = len as u32;
                    *buf_ptr.sub(1) = SdsType::Type32 as u8;
                }
                SdsType::Type64 => {
                    let sh = (buf_ptr as *mut u8).sub(mem::size_of::<Sdshdr64>()) as *mut Sdshdr64;
                    (*sh).len = len as u64;
                    (*sh).alloc = len as u64;
                    *buf_ptr.sub(1) = SdsType::Type64 as u8;
                }
            }
            
            Sds { ptr: buf_ptr }
        }
    }
    
    pub fn len(&self) -> usize {
        unsafe {
            let flags = *self.ptr.sub(1);
            match flags & SDS_TYPE_MASK {
                0 => (flags >> SDS_TYPE_BITS) as usize,
                1 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr8>()) as *const Sdshdr8;
                    (*sh).len as usize
                }
                2 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr16>()) as *const Sdshdr16;
                    (*sh).len as usize
                }
                3 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr32>()) as *const Sdshdr32;
                    (*sh).len as usize
                }
                4 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr64>()) as *const Sdshdr64;
                    (*sh).len as usize
                }
                _ => 0,
            }
        }
    }
    
    pub fn avail(&self) -> usize {
        unsafe {
            let flags = *self.ptr.sub(1);
            match flags & SDS_TYPE_MASK {
                0 => 0,
                1 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr8>()) as *const Sdshdr8;
                    ((*sh).alloc - (*sh).len) as usize
                }
                2 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr16>()) as *const Sdshdr16;
                    ((*sh).alloc - (*sh).len) as usize
                }
                3 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr32>()) as *const Sdshdr32;
                    ((*sh).alloc - (*sh).len) as usize
                }
                4 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr64>()) as *const Sdshdr64;
                    ((*sh).alloc - (*sh).len) as usize
                }
                _ => 0,
            }
        }
    }
    
    pub fn make_room_for(&mut self, addlen: usize) -> bool {
        unsafe {
            let avail = self.avail();
            if avail >= addlen {
                return true;
            }
            
            let len = self.len();
            let oldtype = *self.ptr.sub(1) & SDS_TYPE_MASK;
            let oldhdrlen = sds_hdr_size_by_type(oldtype);
            
            let mut newlen = len + addlen;
            if newlen < SDS_MAX_PREALLOC {
                newlen *= 2;
            } else {
                newlen += SDS_MAX_PREALLOC;
            }
            
            let type_ = sds_req_type(newlen);
            let hdrlen = sds_hdr_size(type_);
            
            if oldtype == type_ as u8 {
                let sh = self.ptr.sub(oldhdrlen) as *mut c_void;
                let newsh = sds_realloc(sh, hdrlen + newlen + 1);
                if newsh.is_null() {
                    return false;
                }
                self.ptr = (newsh as *mut u8).add(hdrlen);
            } else {
                let newsh = sds_malloc(hdrlen + newlen + 1);
                if newsh.is_null() {
                    return false;
                }
                
                ptr::copy_nonoverlapping(
                    self.ptr,
                    (newsh as *mut u8).add(hdrlen),
                    len + 1
                );
                
                sds_free(self.ptr.sub(oldhdrlen) as *mut c_void);
                self.ptr = (newsh as *mut u8).add(hdrlen);
                *self.ptr.sub(1) = type_ as u8;
                self.set_len(len);
            }
            
            self.set_alloc(newlen);
            true
        }
    }
    
    fn set_len(&mut self, newlen: usize) {
        unsafe {
            let flags = *self.ptr.sub(1);
            match flags & SDS_TYPE_MASK {
                0 => {
                    let fp = self.ptr.sub(1);
                    *fp = (SdsType::Type5 as u8) | ((newlen as u8) << SDS_TYPE_BITS);
                }
                1 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr8>()) as *mut Sdshdr8;
                    (*sh).len = newlen as u8;
                }
                2 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr16>()) as *mut Sdshdr16;
                    (*sh).len = newlen as u16;
                }
                3 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr32>()) as *mut Sdshdr32;
                    (*sh).len = newlen as u32;
                }
                4 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr64>()) as *mut Sdshdr64;
                    (*sh).len = newlen as u64;
                }
                _ => {}
            }
        }
    }
    
    fn set_alloc(&mut self, newlen: usize) {
        unsafe {
            let flags = *self.ptr.sub(1);
            match flags & SDS_TYPE_MASK {
                0 => {}
                1 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr8>()) as *mut Sdshdr8;
                    (*sh).alloc = newlen as u8;
                }
                2 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr16>()) as *mut Sdshdr16;
                    (*sh).alloc = newlen as u16;
                }
                3 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr32>()) as *mut Sdshdr32;
                    (*sh).alloc = newlen as u32;
                }
                4 => {
                    let sh = self.ptr.sub(mem::size_of::<Sdshdr64>()) as *mut Sdshdr64;
                    (*sh).alloc = newlen as u64;
                }
                _ => {}
            }
        }
    }
    
    pub fn cat(&mut self, t: &str) -> bool {
        let len = t.len();
        let curlen = self.len();
        
        if !self.make_room_for(len) {
            return false;
        }
        
        unsafe {
            ptr::copy_nonoverlapping(
                t.as_ptr(),
                self.ptr.add(curlen),
                len
            );
            self.set_len(curlen + len);
            *self.ptr.add(curlen + len) = 0;
        }
        
        true
    }
    
    pub fn as_str(&self) -> &str {
        unsafe {
            let len = self.len();
            str::from_utf8_unchecked(slice::from_raw_parts(self.ptr, len))
        }
    }
}

impl Drop for Sds {
    fn drop(&mut self) {
        unsafe {
            let flags = *self.ptr.sub(1);
            let hdrlen = sds_hdr_size_by_type(flags & SDS_TYPE_MASK);
            sds_free(self.ptr.sub(hdrlen) as *mut c_void);
        }
    }
}

impl Deref for Sds {
    type Target = str;
    
    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Sds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Debug for Sds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Sds(\"{}\")", self.as_str())
    }
}

unsafe fn sds_hdr_size(type_: SdsType) -> usize {
    match type_ {
        SdsType::Type5 => mem::size_of::<Sdshdr5>(),
        SdsType::Type8 => mem::size_of::<Sdshdr8>(),
        SdsType::Type16 => mem::size_of::<Sdshdr16>(),
        SdsType::Type32 => mem::size_of::<Sdshdr32>(),
        SdsType::Type64 => mem::size_of::<Sdshdr64>(),
    }
}

unsafe fn sds_hdr_size_by_type(type_: u8) -> usize {
    match type_ {
        0 => mem::size_of::<Sdshdr5>(),
        1 => mem::size_of::<Sdshdr8>(),
        2 => mem::size_of::<Sdshdr16>(),
        3 => mem::size_of::<Sdshdr32>(),
        4 => mem::size_of::<Sdshdr64>(),
        _ => 0,
    }
}

fn sds_req_type(string_size: usize) -> SdsType {
    if string_size < 32 {
        SdsType::Type5
    } else if string_size < 0xff {
        SdsType::Type8
    } else if string_size < 0xffff {
        SdsType::Type16
    } else if string_size < 0xffffffff {
        SdsType::Type32
    } else {
        SdsType::Type64
    }
}

extern {
    fn malloc(size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

unsafe fn sds_malloc(size: usize) -> *mut c_void {
    malloc(size)
}

unsafe fn sds_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    realloc(ptr, size)
}

unsafe fn sds_free(ptr: *mut c_void) {
    free(ptr)
}