// xalloc.rs -- memory allocation with out of memory checking

use std::alloc::{alloc, alloc_zeroed, realloc, Layout};
use std::ptr::{copy_nonoverlapping, null_mut};
use std::mem;

/// Error type for allocation failures
#[derive(Debug)]
pub struct AllocationError;

/// Allocates memory with error checking
pub fn xmalloc(size: usize) -> Result<*mut u8, AllocationError> {
    if size == 0 {
        return Ok(null_mut());
    }
    let layout = Layout::from_size_align(size, mem::align_of::<u8>()).map_err(|_| AllocationError)?;
    unsafe { alloc(layout).as_mut().ok_or(AllocationError) }
}

/// Reallocates memory with error checking
pub fn xrealloc(ptr: *mut u8, new_size: usize) -> Result<*mut u8, AllocationError> {
    if new_size == 0 {
        return Ok(null_mut());
    }
    let layout = Layout::from_size_align(new_size, mem::align_of::<u8>()).map_err(|_| AllocationError)?;
    unsafe { realloc(ptr, layout, new_size).as_mut().ok_or(AllocationError) }
}

/// Allocates zeroed memory with error checking
pub fn xzalloc(size: usize) -> Result<*mut u8, AllocationError> {
    if size == 0 {
        return Ok(null_mut());
    }
    let layout = Layout::from_size_align(size, mem::align_of::<u8>()).map_err(|_| AllocationError)?;
    unsafe { alloc_zeroed(layout).as_mut().ok_or(AllocationError) }
}

/// Allocates memory for an array with error checking
pub fn xnmalloc(n: usize, size: usize) -> Result<*mut u8, AllocationError> {
    xmalloc(n * size)
}

/// Reallocates memory for an array with error checking
pub fn xreallocarray(ptr: *mut u8, n: usize, size: usize) -> Result<*mut u8, AllocationError> {
    xrealloc(ptr, n * size)
}

/// Duplicates memory with error checking
pub fn xmemdup(src: *const u8, size: usize) -> Result<*mut u8, AllocationError> {
    let dest = xmalloc(size)?;
    unsafe {
        copy_nonoverlapping(src, dest, size);
    }
    Ok(dest)
}

/// Duplicates string with error checking
pub fn xstrdup(s: &str) -> Result<*mut u8, AllocationError> {
    let size = s.len() + 1;
    let dest = xmalloc(size)?;
    unsafe {
        copy_nonoverlapping(s.as_ptr(), dest, s.len());
        *dest.add(s.len()) = 0;
    }
    Ok(dest)
}

/// Grows an allocation exponentially
pub fn x2nrealloc(
    ptr: *mut u8,
    pn: &mut usize,
    size: usize,
) -> Result<*mut u8, AllocationError> {
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
        // Set N = floor(1.5 * N) + 1
        n = n + (n >> 1) + 1;
    }

    let new_ptr = xreallocarray(ptr, n, size)?;
    *pn = n;
    Ok(new_ptr)
}

/// Grows an allocation with more control over parameters
pub fn xpalloc(
    pa: *mut u8,
    pn: &mut usize,
    n_incr_min: usize,
    n_max: Option<usize>,
    size: usize,
) -> Result<*mut u8, AllocationError> {
    let n0 = *pn;
    const DEFAULT_MXFAST: usize = 64 * mem::size_of::<usize>() / 4;

    // Calculate new size (1.5x growth)
    let mut n = n0 + (n0 >> 1);
    if let Some(max) = n_max {
        if n > max {
            n = max;
        }
    }

    // Adjust for DEFAULT_MXFAST
    let nbytes = n.checked_mul(size).ok_or(AllocationError)?;
    let adjusted_nbytes = if nbytes < DEFAULT_MXFAST {
        DEFAULT_MXFAST
    } else {
        nbytes
    };
    let n = adjusted_nbytes / size;

    // Check minimum increment
    if n - n0 < n_incr_min {
        let new_n = n0 + n_incr_min;
        if n_max.map(|max| new_n > max).unwrap_or(false) {
            return Err(AllocationError);
        }
        n = new_n;
    }

    let new_pa = xrealloc(pa, n * size)?;
    *pn = n;
    Ok(new_pa)
}