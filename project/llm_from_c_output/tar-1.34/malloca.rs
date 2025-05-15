//! Safe automatic memory allocation.

#![no_std]

use core::alloc::{GlobalAlloc, Layout};
use core::mem;
use core::ptr;

/// The desired alignment of memory allocations is the maximum alignment
/// among all elementary types.
const SA_ALIGNMENT_MAX: usize = {
    let align_long = mem::align_of::<usize>();
    let align_double = mem::align_of::<f64>();
    let align_longlong = mem::align_of::<u64>();
    let align_longdouble = mem::align_of::<u128>(); // Approximation for long double
    
    ((align_long - 1) | (align_double - 1) | (align_longlong - 1) | (align_longdouble - 1)) + 1
};

/// Safe variant of alloca(N). Allocates N bytes of memory on the stack.
/// Returns None if allocation fails or if N is too large.
#[inline]
pub fn safe_alloca<N: Into<usize>>(size: N) -> Option<*mut u8> {
    let size = size.into();
    if size < 4032 {
        // In Rust, we can't safely use alloca directly, so we return None
        // In practice, you'd want to use stackalloc crate or similar
        None
    } else {
        None
    }
}

/// Safe variant of alloca that must be freed with freea().
/// Returns None on failure.
pub fn malloca(size: usize) -> Option<*mut u8> {
    if size < 4032 - (2 * SA_ALIGNMENT_MAX - 1) {
        // Stack allocation path - not safely possible in Rust without unsafe
        None
    } else {
        mmalloca(size)
    }
}

/// Heap allocation fallback for malloca.
pub fn mmalloca(size: usize) -> Option<*mut u8> {
    let nplus = size.checked_add(mem::size_of::<u8>() + 2 * SA_ALIGNMENT_MAX - 1)?;
    
    let layout = Layout::from_size_align(nplus, 1).ok()?;
    let mem = unsafe { alloc(layout) };
    if mem.is_null() {
        return None;
    }

    let p = ((mem as usize + mem::size_of::<u8>() + SA_ALIGNMENT_MAX - 1) 
            & !(2 * SA_ALIGNMENT_MAX - 1)) + SA_ALIGNMENT_MAX;
    let p = p as *mut u8;
    
    unsafe {
        ptr::write(p.sub(1) as *mut u8, (p as usize - mem as usize) as u8);
    }
    
    Some(p)
}

/// Free memory allocated with malloca.
pub unsafe fn freea(p: *mut u8) {
    if p.is_null() {
        return;
    }
    
    if (p as usize) & (SA_ALIGNMENT_MAX - 1) != 0 {
        // Invalid pointer - not aligned properly
        core::intrinsics::abort();
    }
    
    if (p as usize) & SA_ALIGNMENT_MAX != 0 {
        // Heap allocated
        let offset = ptr::read(p.sub(1));
        let mem = p.sub(offset as usize);
        let layout = Layout::from_size_align(offset as usize + mem::size_of::<u8>() + 2 * SA_ALIGNMENT_MAX - 1, 1)
            .expect("Invalid layout");
        dealloc(mem, layout);
    }
    // Stack allocated pointers are not freed in Rust version
}

/// Overflow-safe variant of malloca for array allocation.
pub fn nmalloca(n: usize, size: usize) -> Option<*mut u8> {
    if n.checked_mul(size).is_none() {
        None
    } else {
        malloca(n * size)
    }
}

// Dummy allocator functions - should be replaced with actual allocator
unsafe fn alloc(layout: Layout) -> *mut u8 {
    // In practice, use GlobalAlloc or another allocator
    core::ptr::null_mut()
}

unsafe fn dealloc(ptr: *mut u8, _layout: Layout) {
    // In practice, use GlobalAlloc or another allocator
}