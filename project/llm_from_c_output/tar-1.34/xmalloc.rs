/*
 * xmalloc.rs -- memory allocation with out of memory checking
 *
 * Translated from C to Rust with equivalent functionality and safety.
 */

use std::alloc::{alloc, alloc_zeroed, realloc, Layout};
use std::ptr::{null_mut, NonNull};
use std::process;
use std::mem;

/// Allocates memory with error checking
pub fn xmalloc(n: usize) -> NonNull<u8> {
    if n == 0 {
        return NonNull::new(1 as *mut u8).unwrap(); // Return minimal allocation for zero size
    }

    let layout = Layout::from_size_align(n, mem::align_of::<u8>()).unwrap();
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            xalloc_die();
        }
        NonNull::new_unchecked(ptr)
    }
}

/// Reallocates memory with error checking
pub fn xrealloc(p: Option<NonNull<u8>>, n: usize) -> Option<NonNull<u8>> {
    if n == 0 {
        if let Some(ptr) = p {
            unsafe {
                let layout = Layout::from_size_align(1, mem::align_of::<u8>()).unwrap();
                dealloc(ptr.as_ptr(), layout);
            }
        }
        return None;
    }

    let layout = Layout::from_size_align(n, mem::align_of::<u8>()).unwrap();
    unsafe {
        let ptr = realloc(
            p.map_or(null_mut(), |p| p.as_ptr()),
            layout,
            n
        );
        if ptr.is_null() {
            xalloc_die();
        }
        NonNull::new(ptr)
    }
}

/// Reallocates memory with size doubling strategy
pub fn x2realloc(p: Option<NonNull<u8>>, pn: &mut usize) -> NonNull<u8> {
    x2nrealloc(p, pn, 1)
}

/// Allocates zeroed memory with error checking
pub fn xzalloc(n: usize) -> NonNull<u8> {
    xcalloc(n, 1)
}

/// Allocates zeroed memory for array with error checking
pub fn xcalloc(n: usize, s: usize) -> NonNull<u8> {
    if n == 0 || s == 0 {
        return NonNull::new(1 as *mut u8).unwrap(); // Return minimal allocation for zero size
    }

    let layout = Layout::from_size_align(n * s, mem::align_of::<u8>()).unwrap();
    unsafe {
        let ptr = alloc_zeroed(layout);
        if ptr.is_null() {
            xalloc_die();
        }
        NonNull::new_unchecked(ptr)
    }
}

/// Duplicates memory region
pub fn xmemdup(p: &[u8]) -> NonNull<u8> {
    let s = p.len();
    let new_ptr = xmalloc(s);
    unsafe {
        std::ptr::copy_nonoverlapping(p.as_ptr(), new_ptr.as_ptr(), s);
    }
    new_ptr
}

/// Duplicates string
pub fn xstrdup(string: &str) -> NonNull<u8> {
    let bytes = string.as_bytes();
    let mut vec = Vec::with_capacity(bytes.len() + 1);
    vec.extend_from_slice(bytes);
    vec.push(0); // null terminator
    NonNull::new(Box::into_raw(vec.into_boxed_slice()).cast()).unwrap()
}

/// Helper function for exponential reallocation
pub fn x2nrealloc(p: Option<NonNull<u8>>, pn: &mut usize, s: usize) -> NonNull<u8> {
    if *pn == 0 {
        *pn = 1;
    } else {
        *pn = (*pn).checked_mul(2).unwrap_or_else(|| xalloc_die());
    }

    xrealloc(p, (*pn).checked_mul(s).unwrap_or_else(|| xalloc_die()))
        .unwrap_or_else(|| xalloc_die())
}

/// Out of memory handler
fn xalloc_die() -> ! {
    eprintln!("Memory allocation failed");
    process::abort()
}