// xalloc.rs -- memory allocation with out of memory checking

use std::alloc::{alloc, alloc_zeroed, dealloc, realloc, Layout};
use std::ptr::{copy_nonoverlapping, null_mut};
use std::mem;

/// Allocate memory with error checking
pub fn xmalloc(n: usize) -> *mut u8 {
    if n == 0 {
        return null_mut();
    }
    
    let layout = Layout::from_size_align(n, mem::align_of::<u8>()).unwrap();
    unsafe {
        let p = alloc(layout);
        if p.is_null() {
            xalloc_die();
        }
        p
    }
}

/// Reallocate memory with error checking
pub fn xrealloc(p: *mut u8, n: usize) -> *mut u8 {
    if n == 0 {
        if !p.is_null() {
            unsafe {
                dealloc(p, Layout::from_size_align(1, mem::align_of::<u8>()).unwrap());
            }
        }
        return null_mut();
    }

    let layout = Layout::from_size_align(n, mem::align_of::<u8>()).unwrap();
    unsafe {
        let r = realloc(p, layout, n);
        if r.is_null() {
            xalloc_die();
        }
        r
    }
}

/// Reallocate memory with size doubling strategy
pub fn x2realloc(p: *mut u8, pn: &mut usize) -> *mut u8 {
    x2nrealloc(p, pn, 1)
}

/// Allocate zeroed memory with error checking
pub fn xzalloc(n: usize) -> *mut u8 {
    xcalloc(n, 1)
}

/// Allocate zeroed memory for array with error checking
pub fn xcalloc(n: usize, s: usize) -> *mut u8 {
    if n == 0 || s == 0 {
        return null_mut();
    }

    if let Some(size) = n.checked_mul(s) {
        let layout = Layout::from_size_align(size, mem::align_of::<u8>()).unwrap();
        unsafe {
            let p = alloc_zeroed(layout);
            if p.is_null() {
                xalloc_die();
            }
            p
        }
    } else {
        xalloc_die();
        null_mut()
    }
}

/// Duplicate memory region
pub fn xmemdup(p: *const u8, s: usize) -> *mut u8 {
    let new_ptr = xmalloc(s);
    unsafe {
        copy_nonoverlapping(p, new_ptr, s);
    }
    new_ptr
}

/// Duplicate string
pub fn xstrdup(string: &str) -> *mut u8 {
    let len = string.len() + 1;
    let new_ptr = xmalloc(len);
    unsafe {
        copy_nonoverlapping(string.as_ptr(), new_ptr, len);
    }
    new_ptr
}

/// Helper function for growing allocations
pub fn x2nrealloc(p: *mut u8, pn: &mut usize, s: usize) -> *mut u8 {
    if *pn == 0 {
        *pn = 128 / s;
        if *pn == 0 {
            *pn = 1;
        }
        return xmalloc(*pn * s);
    }

    *pn = (*pn).checked_mul(2).unwrap_or_else(|| xalloc_die());
    xrealloc(p, *pn * s)
}

/// Out of memory handler
fn xalloc_die() -> ! {
    eprintln!("Memory allocation failed");
    std::process::exit(1);
}