//! Safe automatic memory allocation.
//! 
//! This module provides safe alternatives to alloca() for stack and heap allocations.

use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

/// The maximum alignment required for any fundamental type.
const SA_ALIGNMENT_MAX: usize = {
    let align_long = mem::align_of::<usize>();
    let align_double = mem::align_of::<f64>();
    let align_longlong = mem::align_of::<u64>();
    let align_longdouble = if cfg!(target_arch = "x86") { 4 } else { 16 };
    
    ((align_long - 1) | (align_double - 1) | (align_longlong - 1) | (align_longdouble - 1)) + 1
};

/// Safe version of alloca() that returns None for large allocations.
/// 
/// Only use for small allocations (< 4032 bytes) on the stack.
#[inline]
pub fn safe_alloca(size: usize) -> Option<*mut u8> {
    if size < 4032 {
        Some(unsafe { std::alloc::alloca(size) })
    } else {
        None
    }
}

/// Allocates memory on stack or heap depending on size.
/// 
/// For small allocations (< 4032 bytes), uses stack allocation.
/// For larger allocations, uses heap allocation.
/// Must be freed with freea().
pub fn malloca(size: usize) -> Option<*mut u8> {
    if size < 4032 - (2 * SA_ALIGNMENT_MAX - 1) {
        let mem = unsafe { std::alloc::alloca(size + 2 * SA_ALIGNMENT_MAX - 1) };
        let aligned_ptr = ((mem as usize + 2 * SA_ALIGNMENT_MAX - 1) & !(2 * SA_ALIGNMENT_MAX - 1)) as *mut u8;
        Some(aligned_ptr)
    } else {
        mmalloca(size)
    }
}

/// Heap allocation fallback for malloca().
pub fn mmalloca(size: usize) -> Option<*mut u8> {
    let alignment2_mask = 2 * SA_ALIGNMENT_MAX - 1;
    let plus = mem::size_of::<u8>() + alignment2_mask;
    
    let nplus = size.checked_add(plus)?;
    if nplus > isize::MAX as usize {
        return None;
    }

    let layout = Layout::from_size_align(nplus, 1).ok()?;
    let mem = unsafe { alloc(layout) };
    if mem.is_null() {
        return None;
    }

    let umemplus = mem as usize + mem::size_of::<u8>() + SA_ALIGNMENT_MAX - 1;
    let offset = ((umemplus & !alignment2_mask) + SA_ALIGNMENT_MAX - mem as usize);
    let vp = unsafe { mem.add(offset) };
    
    unsafe {
        ptr::write(vp.sub(1) as *mut u8, offset as u8);
    }
    
    Some(vp)
}

/// Safe array allocation version of malloca.
pub fn nmalloca(n: usize, size: usize) -> Option<*mut u8> {
    if n.checked_mul(size).is_none() {
        None
    } else {
        malloca(n * size)
    }
}

/// Frees memory allocated by malloca().
pub unsafe fn freea(p: *mut u8) {
    if p.is_null() {
        return;
    }

    if (p as usize) & (SA_ALIGNMENT_MAX - 1) != 0 {
        panic!("Invalid pointer passed to freea()");
    }

    if (p as usize) & SA_ALIGNMENT_MAX != 0 {
        let offset = ptr::read(p.sub(1)) as usize;
        let mem = p.sub(offset);
        let layout = Layout::from_size_align(offset + 2 * SA_ALIGNMENT_MAX - 1, 1)
            .expect("Invalid layout");
        dealloc(mem, layout);
    }
    // Stack allocations are automatically freed when the function returns
}