// zutil.rs - Target dependent utility functions for the compression library

use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};
use std::ffi::CString;
use std::os::raw::{c_char, c_int, c_ulong, c_void};
use std::ptr;

pub const ZLIB_VERSION: &str = "1.2.11";

pub const Z_NEED_DICT: c_int = 2;
pub const Z_STREAM_END: c_int = 1;
pub const Z_OK: c_int = 0;
pub const Z_ERRNO: c_int = -1;
pub const Z_STREAM_ERROR: c_int = -2;
pub const Z_DATA_ERROR: c_int = -3;
pub const Z_MEM_ERROR: c_int = -4;
pub const Z_BUF_ERROR: c_int = -5;
pub const Z_VERSION_ERROR: c_int = -6;

pub const DEF_WBITS: c_int = 15; // MAX_WBITS
pub const DEF_MEM_LEVEL: c_int = 8;

pub const STORED_BLOCK: c_int = 0;
pub const STATIC_TREES: c_int = 1;
pub const DYN_TREES: c_int = 2;

pub const MIN_MATCH: usize = 3;
pub const MAX_MATCH: usize = 258;

pub const PRESET_DICT: c_int = 0x20;
pub const OS_CODE: c_int = 0x03; // Unix

pub static Z_ERRMSG: [&str; 10] = [
    "need dictionary",      // Z_NEED_DICT
    "stream end",           // Z_STREAM_END
    "",                     // Z_OK
    "file error",           // Z_ERRNO
    "stream error",         // Z_STREAM_ERROR
    "data error",           // Z_DATA_ERROR
    "insufficient memory",  // Z_MEM_ERROR
    "buffer error",         // Z_BUF_ERROR
    "incompatible version", // Z_VERSION_ERROR
    "",
];

pub fn zlib_version() -> &'static str {
    ZLIB_VERSION
}

pub fn zlib_compile_flags() -> c_ulong {
    let mut flags = 0;

    match std::mem::size_of::<u32>() {
        2 => (),
        4 => flags += 1,
        8 => flags += 2,
        _ => flags += 3,
    }

    match std::mem::size_of::<c_ulong>() {
        2 => (),
        4 => flags += 1 << 2,
        8 => flags += 2 << 2,
        _ => flags += 3 << 2,
    }

    match std::mem::size_of::<*mut c_void>() {
        2 => (),
        4 => flags += 1 << 4,
        8 => flags += 2 << 4,
        _ => flags += 3 << 4,
    }

    match std::mem::size_of::<i64>() { // z_off_t is typically i64
        2 => (),
        4 => flags += 1 << 6,
        8 => flags += 2 << 6,
        _ => flags += 3 << 6,
    }

    #[cfg(debug_assertions)]
    {
        flags += 1 << 8;
    }

    // Other flags would be added here based on compilation features
    flags
}

pub fn z_error(msg: &str) -> ! {
    eprintln!("{}", msg);
    std::process::exit(1);
}

pub fn z_error_msg(err: c_int) -> &'static str {
    let idx = Z_NEED_DICT - err;
    if idx >= 0 && (idx as usize) < Z_ERRMSG.len() {
        Z_ERRMSG[idx as usize]
    } else {
        ""
    }
}

pub fn zmemcpy(dest: &mut [u8], source: &[u8]) {
    dest.copy_from_slice(source);
}

pub fn zmemcmp(s1: &[u8], s2: &[u8]) -> c_int {
    s1.cmp(s2) as c_int
}

pub fn zmemzero(dest: &mut [u8]) {
    dest.fill(0);
}

pub struct ZAllocator;

impl ZAllocator {
    pub fn alloc(&self, items: usize, size: usize) -> *mut c_void {
        let total_size = items * size;
        if total_size == 0 {
            return ptr::null_mut();
        }

        unsafe {
            let layout = Layout::from_size_align(total_size, 1).unwrap();
            alloc(layout) as *mut c_void
        }
    }

    pub fn free(&self, ptr: *mut c_void) {
        if !ptr.is_null() {
            unsafe {
                // Note: We don't know the size here, which is unsafe
                // In a real implementation, we'd need to track allocations
                dealloc(ptr as *mut u8, Layout::new::<u8>());
            }
        }
    }

    pub fn calloc(&self, items: usize, size: usize) -> *mut c_void {
        let total_size = items * size;
        if total_size == 0 {
            return ptr::null_mut();
        }

        unsafe {
            let layout = Layout::from_size_align(total_size, 1).unwrap();
            alloc_zeroed(layout) as *mut c_void
        }
    }
}