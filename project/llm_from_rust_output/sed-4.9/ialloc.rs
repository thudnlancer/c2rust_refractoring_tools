use std::alloc::{self, Layout};
use std::ptr;

pub type idx_t = isize;

#[cold]
fn alloc_nomem() -> *mut u8 {
    std::ptr::null_mut()
}

pub fn ireallocarray(p: *mut u8, n: idx_t, s: idx_t) -> *mut u8 {
    if n == 0 || s == 0 {
        return ireallocarray(p, 1, 1);
    }

    let size = match n.checked_mul(s) {
        Some(size) if size >= 0 => size as usize,
        _ => return alloc_nomem(),
    };

    if p.is_null() {
        unsafe { alloc::alloc(Layout::from_size_align(size, 1).unwrap()) }
    } else {
        unsafe { alloc::realloc(p, Layout::from_size_align(1, 1).unwrap(), size) }
    }
}

pub fn icalloc(n: idx_t, s: idx_t) -> *mut u8 {
    let size = match n.checked_mul(s) {
        Some(size) if size >= 0 => size as usize,
        _ => return alloc_nomem(),
    };

    unsafe { alloc::alloc_zeroed(Layout::from_size_align(size, 1).unwrap()) }
}

pub fn irealloc(p: *mut u8, s: idx_t) -> *mut u8 {
    let size = if s == 0 { 1 } else { s as usize };

    if p.is_null() {
        unsafe { alloc::alloc(Layout::from_size_align(size, 1).unwrap()) }
    } else {
        unsafe { alloc::realloc(p, Layout::from_size_align(1, 1).unwrap(), size) }
    }
}

pub fn imalloc(s: idx_t) -> *mut u8 {
    let size = if s == 0 { 1 } else { s as usize };
    unsafe { alloc::alloc(Layout::from_size_align(size, 1).unwrap()) }
}