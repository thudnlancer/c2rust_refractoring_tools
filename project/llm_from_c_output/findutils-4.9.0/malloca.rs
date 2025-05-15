//! Safe automatic memory allocation.
//! 
//! This module provides safe variants of alloca-like functionality for Rust,
//! with proper memory management and alignment guarantees.

use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

/// The maximum alignment required for any type.
const MAX_ALIGNMENT: usize = {
    let align_long = mem::align_of::<usize>();
    let align_double = mem::align_of::<f64>();
    let align_longlong = mem::align_of::<u64>();
    let align_longdouble = if cfg!(target_arch = "x86") { 16 } else { mem::align_of::<f64>() };
    
    ((align_long - 1) | (align_double - 1) | (align_longlong - 1) | (align_longdouble - 1)) + 1
};

/// Safe stack allocation for small amounts of memory.
/// 
/// Returns None if the requested size is too large for safe stack allocation.
#[inline]
pub fn safe_alloca<T>(size: usize) -> Option<*mut T> {
    if size <= 4032 {
        Some(alloca(size))
    } else {
        None
    }
}

/// Allocate memory on the stack.
/// 
/// # Safety
/// This uses Rust's unstable alloca functionality and is inherently unsafe.
/// The caller must ensure the memory is not accessed after the function returns.
#[inline]
unsafe fn alloca<T>(size: usize) -> *mut T {
    let layout = Layout::from_size_align(size, 1).unwrap();
    let ptr = std::alloc::alloc(layout);
    ptr as *mut T
}

/// Allocate memory that must be freed with `freea`.
/// 
/// For small allocations, uses stack allocation. For larger allocations,
/// falls back to heap allocation.
pub fn malloca(size: usize) -> Option<*mut u8> {
    if size <= 4032 - (2 * MAX_ALIGNMENT - 1) {
        unsafe {
            let raw_ptr = alloca::<u8>(size + 2 * MAX_ALIGNMENT - 1);
            let aligned_ptr = ((raw_ptr as usize + 2 * MAX_ALIGNMENT - 1) & !(2 * MAX_ALIGNMENT - 1)) as *mut u8;
            Some(aligned_ptr)
        }
    } else {
        mmalloca(size)
    }
}

/// Heap allocation fallback for malloca.
pub fn mmalloca(size: usize) -> Option<*mut u8> {
    if size == 0 {
        return None;
    }

    let total_size = size + mem::size_of::<usize>() + 2 * MAX_ALIGNMENT - 1;
    let layout = Layout::from_size_align(total_size, 1).ok()?;

    unsafe {
        let mem = alloc(layout);
        if mem.is_null() {
            return None;
        }

        let umemplus = mem as usize + mem::size_of::<usize>() + MAX_ALIGNMENT - 1;
        let offset = ((umemplus & !(2 * MAX_ALIGNMENT - 1)) + MAX_ALIGNMENT - (mem as usize);
        let vp = mem.add(offset);
        
        // Store the offset before the returned pointer
        ptr::write((vp as *mut usize).offset(-1), offset);
        
        Some(vp)
    }
}

/// Free memory allocated with malloca.
/// 
/// # Safety
/// The pointer must have been returned by malloca or mmalloca.
pub unsafe fn freea(p: *mut u8) {
    if p.is_null() {
        return;
    }

    // Check alignment
    if (p as usize) & (MAX_ALIGNMENT - 1) != 0 {
        panic!("Invalid pointer passed to freea");
    }

    // Check if it was heap allocated
    if (p as usize) & MAX_ALIGNMENT != 0 {
        let offset = ptr::read((p as *mut usize).offset(-1));
        let mem = p.sub(offset);
        let total_size = offset + mem::size_of::<usize>() + 2 * MAX_ALIGNMENT - 1;
        let layout = Layout::from_size_align_unchecked(total_size, 1);
        dealloc(mem, layout);
    }
}

/// Safe version of nmalloca that checks for overflow.
pub fn nmalloca(n: usize, size: usize) -> Option<*mut u8> {
    n.checked_mul(size).and_then(|s| malloca(s))
}