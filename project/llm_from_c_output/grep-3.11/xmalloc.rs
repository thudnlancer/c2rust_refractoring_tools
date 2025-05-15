use std::alloc::{alloc, alloc_zeroed, realloc, Layout};
use std::mem;
use std::ptr::{copy_nonoverlapping, null_mut};
use std::process;

/// Helper function to check for null pointer and abort if null
fn nonnull<T>(ptr: *mut T) -> *mut T {
    if ptr.is_null() {
        xalloc_die();
    }
    ptr
}

/// Abort the program on allocation failure
fn xalloc_die() -> ! {
    eprintln!("Memory allocation failed");
    process::abort();
}

/// Allocate `size` bytes of memory dynamically, with error checking
pub fn xmalloc(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, mem::align_of::<u8>()).unwrap();
    nonnull(unsafe { alloc(layout) })
}

/// Change the size of an allocated block of memory `ptr` to `size` bytes,
/// with error checking
pub fn xrealloc(ptr: *mut u8, size: usize) -> *mut u8 {
    if size == 0 {
        return ptr;
    }
    
    let layout = Layout::from_size_align(size, mem::align_of::<u8>()).unwrap();
    let new_ptr = unsafe { realloc(ptr, layout, size) };
    
    if new_ptr.is_null() && !ptr.is_null() {
        xalloc_die();
    }
    new_ptr
}

/// Allocate an array of `n` objects, each with `size` bytes of memory,
/// with error checking
pub fn xnmalloc(n: usize, size: usize) -> *mut u8 {
    xrealloc(null_mut(), n * size)
}

/// Allocate zeroed memory for `n` elements of `size` bytes, with error checking
pub fn xcalloc(n: usize, size: usize) -> *mut u8 {
    let total_size = n * size;
    let layout = Layout::from_size_align(total_size, mem::align_of::<u8>()).unwrap();
    nonnull(unsafe { alloc_zeroed(layout) })
}

/// Allocate `size` bytes of zeroed memory dynamically, with error checking
pub fn xzalloc(size: usize) -> *mut u8 {
    xcalloc(1, size)
}

/// Clone an object `src` of size `size`, with error checking
pub fn xmemdup(src: *const u8, size: usize) -> *mut u8 {
    let dest = xmalloc(size);
    unsafe {
        copy_nonoverlapping(src, dest, size);
    }
    dest
}

/// Clone a null-terminated string, with error checking
pub fn xstrdup(string: &str) -> *mut u8 {
    let len = string.len();
    let ptr = xmalloc(len + 1);
    unsafe {
        copy_nonoverlapping(string.as_ptr(), ptr, len);
        *ptr.add(len) = 0;
    }
    ptr
}

/// Clone an object `src` of size `size`, with error checking, and append a null byte
pub fn ximemdup0(src: *const u8, size: usize) -> *mut u8 {
    let dest = xmalloc(size + 1);
    unsafe {
        copy_nonoverlapping(src, dest, size);
        *dest.add(size) = 0;
    }
    dest
}

/// Reallocate memory with exponential growth strategy
pub fn x2nrealloc(ptr: *mut u8, pn: &mut usize, size: usize) -> *mut u8 {
    let mut n = *pn;
    
    if ptr.is_null() {
        if n == 0 {
            const DEFAULT_MXFAST: usize = 64 * mem::size_of::<usize>() / 4;
            n = DEFAULT_MXFAST / size;
            if n == 0 {
                n = 1;
            }
        }
    } else {
        // Set n = floor(1.5 * n) + 1
        n = n + (n >> 1) + 1;
    }
    
    let new_ptr = xrealloc(ptr, n * size);
    *pn = n;
    new_ptr
}

/// Grow an array with additional constraints
pub fn xpalloc(
    pa: *mut u8,
    pn: &mut usize,
    n_incr_min: usize,
    n_max: isize,
    size: usize,
) -> *mut u8 {
    let mut n0 = *pn;
    const DEFAULT_MXFAST: usize = 64 * mem::size_of::<usize>() / 4;
    
    let mut n = n0 + (n0 >> 1);
    if n_max >= 0 && n > n_max as usize {
        n = n_max as usize;
    }
    
    let mut nbytes = n * size;
    if nbytes < DEFAULT_MXFAST {
        nbytes = DEFAULT_MXFAST;
        n = nbytes / size;
    }
    
    if pa.is_null() {
        *pn = 0;
    }
    
    if n - n0 < n_incr_min {
        n = n0 + n_incr_min;
        if n_max >= 0 && n > n_max as usize {
            xalloc_die();
        }
        nbytes = n * size;
    }
    
    let new_pa = xrealloc(pa, nbytes);
    *pn = n;
    new_pa
}