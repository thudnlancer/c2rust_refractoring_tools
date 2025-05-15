use std::alloc::{alloc, alloc_zeroed, realloc, Layout};
use std::mem;
use std::ptr;
use std::ptr::NonNull;

/// Error handler for allocation failures
fn xalloc_die() -> ! {
    panic!("Memory allocation failed");
}

/// Safe wrapper for non-null pointer checking
fn nonnull<T>(ptr: *mut T) -> NonNull<T> {
    NonNull::new(ptr).unwrap_or_else(|| xalloc_die())
}

/// Allocate S bytes of memory dynamically, with error checking
pub fn xmalloc(s: usize) -> NonNull<u8> {
    unsafe {
        let layout = Layout::from_size_align_unchecked(s, mem::align_of::<u8>());
        nonnull(alloc(layout))
    }
}

/// Change the size of an allocated block of memory P to S bytes,
/// with error checking
pub fn xrealloc(p: Option<NonNull<u8>>, s: usize) -> NonNull<u8> {
    unsafe {
        let layout = Layout::from_size_align_unchecked(s, mem::align_of::<u8>());
        let ptr = p.map_or(ptr::null_mut(), |p| p.as_ptr());
        let new_ptr = realloc(ptr, layout, s);
        if new_ptr.is_null() && (p.is_none() || s > 0) {
            xalloc_die();
        }
        nonnull(new_ptr)
    }
}

/// Allocate an array of N objects each of S bytes, with error checking
pub fn xnmalloc(n: usize, s: usize) -> NonNull<u8> {
    xrealloc(None, n * s)
}

/// Allocate S bytes of zeroed memory dynamically, with error checking
pub fn xzalloc(s: usize) -> NonNull<u8> {
    unsafe {
        let layout = Layout::from_size_align_unchecked(s, mem::align_of::<u8>());
        nonnull(alloc_zeroed(layout))
    }
}

/// Allocate zeroed memory for N elements of S bytes, with error checking
pub fn xcalloc(n: usize, s: usize) -> NonNull<u8> {
    unsafe {
        let layout = Layout::from_size_align_unchecked(n * s, mem::align_of::<u8>());
        nonnull(alloc_zeroed(layout))
    }
}

/// Clone an object P of size S, with error checking
pub fn xmemdup(p: &[u8]) -> NonNull<u8> {
    let s = p.len();
    let new_ptr = xmalloc(s);
    unsafe {
        ptr::copy_nonoverlapping(p.as_ptr(), new_ptr.as_ptr(), s);
    }
    new_ptr
}

/// Clone STRING with error checking
pub fn xstrdup(string: &str) -> NonNull<u8> {
    let bytes = string.as_bytes();
    let len = bytes.len();
    let new_ptr = xmalloc(len + 1);
    unsafe {
        ptr::copy_nonoverlapping(bytes.as_ptr(), new_ptr.as_ptr(), len);
        new_ptr.as_ptr().add(len).write(0);
    }
    new_ptr
}

/// If P is null, allocate a block of at least *PN such objects;
/// otherwise, reallocate P so that it contains more than *PN objects
/// each of S bytes.
pub fn x2nrealloc(p: Option<NonNull<u8>>, pn: &mut usize, s: usize) -> NonNull<u8> {
    let mut n = *pn;

    if p.is_none() {
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
        if n < *pn {
            xalloc_die(); // Overflow
        }
    }

    let new_ptr = xrealloc(p, n * s);
    *pn = n;
    new_ptr
}

/// Grow PA, which points to an array of *PN items, and return the
/// location of the reallocated array, updating *PN to reflect its
/// new size.
pub fn xpalloc(
    pa: Option<NonNull<u8>>,
    pn: &mut usize,
    n_incr_min: usize,
    n_max: Option<usize>,
    s: usize,
) -> NonNull<u8> {
    let n0 = *pn;
    const DEFAULT_MXFAST: usize = 64 * mem::size_of::<usize>() / 4;

    // Calculate new size (approximately 1.5x)
    let mut n = n0 + (n0 >> 1);
    if n < n0 {
        n = usize::MAX; // Overflow
    }

    // Apply n_max constraint
    if let Some(max) = n_max {
        if n > max {
            n = max;
        }
    }

    // Adjust for DEFAULT_MXFAST if needed
    let nbytes = n.checked_mul(s).unwrap_or(usize::MAX);
    let adjusted_nbytes = if nbytes == usize::MAX {
        usize::MAX.min(DEFAULT_MXFAST)
    } else if nbytes < DEFAULT_MXFAST {
        DEFAULT_MXFAST
    } else {
        0
    };

    if adjusted_nbytes != 0 {
        n = adjusted_nbytes / s;
    }

    if pa.is_none() {
        *pn = 0;
    }

    // Ensure minimum increment
    if n - n0 < n_incr_min {
        n = n0 + n_incr_min;
        if n < n0 || (n_max.is_some() && n > n_max.unwrap()) || n.checked_mul(s).is_none() {
            xalloc_die();
        }
    }

    let new_ptr = xrealloc(pa, n * s);
    *pn = n;
    new_ptr
}