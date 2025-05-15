//! Safe automatic memory allocation.
//! 
//! This module provides safe alternatives to alloca() for stack and heap allocations.

use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

/// The desired alignment of memory allocations is the maximum alignment
/// among all elementary types.
const SA_ALIGNMENT_MAX: usize = {
    let align_long = mem::align_of::<usize>();
    let align_double = mem::align_of::<f64>();
    let align_longlong = mem::align_of::<u64>();
    let align_longdouble = mem::align_of::<f128>();
    
    ((align_long - 1) | (align_double - 1) | (align_longlong - 1) | (align_longdouble - 1)) + 1
};

/// Safe stack allocation when possible, otherwise returns None.
/// 
/// The OS usually guarantees only one guard page at the bottom of the stack,
/// and a page size can be as small as 4096 bytes. So we cannot safely
/// allocate anything larger than 4096 bytes.
pub fn safe_alloca(n: usize) -> Option<*mut u8> {
    if n < 4032 {
        let layout = Layout::from_size_align(n, 1).unwrap();
        unsafe { Some(alloc(layout)) }
    } else {
        None
    }
}

/// Allocate memory on stack or heap that must be freed with freea().
/// 
/// Returns None on allocation failure.
pub fn malloca(n: usize) -> Option<*mut u8> {
    if n < 4032 - (2 * SA_ALIGNMENT_MAX - 1) {
        let total_size = n + 2 * SA_ALIGNMENT_MAX - 1;
        let layout = Layout::from_size_align(total_size, 1).unwrap();
        
        unsafe {
            let mem = alloc(layout);
            if mem.is_null() {
                return None;
            }
            
            let aligned_ptr = ((mem as usize + 2 * SA_ALIGNMENT_MAX - 1) & !(2 * SA_ALIGNMENT_MAX - 1)) as *mut u8;
            Some(aligned_ptr)
        }
    } else {
        mmalloca(n)
    }
}

/// Overflow-safe variant of malloca for allocating arrays.
pub fn nmalloca(n: usize, size: usize) -> Option<*mut u8> {
    n.checked_mul(size).and_then(|total| malloca(total))
}

/// Heap allocation fallback for malloca.
/// 
/// Returns memory aligned to SA_ALIGNMENT_MAX.
pub fn mmalloca(n: usize) -> Option<*mut u8> {
    if n == 0 {
        return None;
    }

    let alignment2_mask = 2 * SA_ALIGNMENT_MAX - 1;
    let plus = mem::size_of::<u8>() + alignment2_mask;
    let nplus = n.checked_add(plus)?;

    let layout = Layout::from_size_align(nplus, 1).ok()?;
    
    unsafe {
        let mem = alloc(layout);
        if mem.is_null() {
            return None;
        }

        let umem = mem as usize;
        let umemplus = umem + mem::size_of::<u8>() + SA_ALIGNMENT_MAX - 1;
        let offset = ((umemplus & !alignment2_mask) + SA_ALIGNMENT_MAX - umem);
        
        let vp = mem.add(offset);
        ptr::write(vp.sub(1) as *mut u8, offset as u8);
        
        Some(vp)
    }
}

/// Free memory allocated by malloca().
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
        let layout = Layout::from_size_align(offset + 1, 1).unwrap();
        dealloc(mem, layout);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_alloca() {
        let small = safe_alloca(100);
        assert!(!small.unwrap().is_null());
        
        let large = safe_alloca(5000);
        assert!(large.is_none());
    }

    #[test]
    fn test_malloca() {
        let mem = malloca(100).unwrap();
        unsafe { freea(mem) };
        
        let large = malloca(5000);
        assert!(!large.unwrap().is_null());
    }
}

#[repr(align(16))]
struct f128(f64, f64);  // Simulate long double alignment