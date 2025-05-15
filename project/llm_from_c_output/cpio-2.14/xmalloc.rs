use std::alloc::{alloc, alloc_zeroed, realloc, Layout};
use std::mem;
use std::ptr::{copy_nonoverlapping, null_mut};
use std::process;

// Error handling function
fn xalloc_die() -> ! {
    eprintln!("Memory allocation failed");
    process::abort();
}

// Safe wrapper for non-null pointers
fn nonnull<T>(p: *mut T) -> *mut T {
    if p.is_null() {
        xalloc_die();
    }
    p
}

// Allocate S bytes of memory dynamically, with error checking
pub fn xmalloc(s: usize) -> *mut u8 {
    unsafe { nonnull(alloc(Layout::from_size_align(s, 1).unwrap())) }
}

// Change the size of an allocated block of memory P to S bytes, with error checking
pub fn xrealloc(p: *mut u8, s: usize) -> *mut u8 {
    if s == 0 {
        return p;
    }
    let new_ptr = unsafe { realloc(p, Layout::from_size_align(s, 1).unwrap(), s) };
    if new_ptr.is_null() && !p.is_null() {
        xalloc_die();
    }
    new_ptr
}

// Allocate an array of N objects, each with S bytes of memory, with error checking
pub fn xnmalloc(n: usize, s: usize) -> *mut u8 {
    xrealloc(null_mut(), n * s)
}

// Allocate zeroed memory for N elements of S bytes, with error checking
pub fn xcalloc(n: usize, s: usize) -> *mut u8 {
    unsafe { nonnull(alloc_zeroed(Layout::from_size_align(n * s, 1).unwrap())) }
}

// Allocate S bytes of zeroed memory dynamically, with error checking
pub fn xzalloc(s: usize) -> *mut u8 {
    xcalloc(s, 1)
}

// Clone an object P of size S, with error checking
pub fn xmemdup(p: *const u8, s: usize) -> *mut u8 {
    let new_ptr = xmalloc(s);
    unsafe {
        copy_nonoverlapping(p, new_ptr, s);
    }
    new_ptr
}

// Clone STRING
pub fn xstrdup(string: &str) -> *mut u8 {
    let len = string.len() + 1;
    let new_ptr = xmalloc(len);
    unsafe {
        copy_nonoverlapping(string.as_ptr(), new_ptr, string.len());
        *new_ptr.add(string.len()) = 0;
    }
    new_ptr
}

// Grow array with exponential growth strategy
pub fn x2nrealloc(p: *mut u8, pn: &mut usize, s: usize) -> *mut u8 {
    let mut n = *pn;

    if p.is_null() {
        if n == 0 {
            const DEFAULT_MXFAST: usize = 64 * mem::size_of::<usize>() / 4;
            n = DEFAULT_MXFAST / s;
            if n == 0 {
                n = 1;
            }
        }
    } else {
        // Set N = floor(1.5 * N) + 1
        n = n + (n >> 1) + 1;
        if n == 0 { // Overflow check
            xalloc_die();
        }
    }

    let new_ptr = xrealloc(p, n * s);
    *pn = n;
    new_ptr
}

// Clone an object P of size S, with error checking. Append a terminating NUL byte
pub fn ximemdup0(p: *const u8, s: usize) -> *mut u8 {
    let new_ptr = xmalloc(s + 1);
    unsafe {
        copy_nonoverlapping(p, new_ptr, s);
        *new_ptr.add(s) = 0;
    }
    new_ptr
}