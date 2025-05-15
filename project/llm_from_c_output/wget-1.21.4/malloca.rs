//! Safe automatic memory allocation.
//! 
//! This module provides safe alternatives to alloca for stack and heap allocations.

use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

/// The maximum alignment required for any type.
const MAX_ALIGNMENT: usize = {
    let align_long = mem::align_of::<usize>();
    let align_double = mem::align_of::<f64>();
    let align_longlong = mem::align_of::<u128>();
    let align_longdouble = mem::align_of::<f128>();
    
    ((align_long - 1) | (align_double - 1) | (align_longlong - 1) | (align_longdouble - 1)) + 1
};

/// The maximum size for stack allocation (4032 bytes - some padding)
const MAX_STACK_ALLOC_SIZE: usize = 4032 - (2 * MAX_ALIGNMENT - 1);

/// A wrapper for stack or heap allocated memory that must be freed.
pub struct Malloca {
    ptr: *mut u8,
    len: usize,
    on_stack: bool,
}

impl Malloca {
    /// Allocate memory on stack or heap.
    pub fn new(size: usize) -> Option<Self> {
        if size <= MAX_STACK_ALLOC_SIZE {
            // Try stack allocation
            let layout = Layout::from_size_align(size, MAX_ALIGNMENT).ok()?;
            let ptr = unsafe { 
                let ptr = alloca(layout.size());
                if ptr.is_null() {
                    return None;
                }
                ptr
            };
            Some(Self {
                ptr,
                len: size,
                on_stack: true,
            })
        } else {
            // Fall back to heap allocation
            let layout = Layout::from_size_align(size, MAX_ALIGNMENT).ok()?;
            let ptr = unsafe { alloc(layout) };
            if ptr.is_null() {
                return None;
            }
            Some(Self {
                ptr,
                len: size,
                on_stack: false,
            })
        }
    }

    /// Get a mutable pointer to the allocated memory.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr
    }

    /// Get the size of the allocation.
    pub fn len(&self) -> usize {
        self.len
    }
}

impl Drop for Malloca {
    fn drop(&mut self) {
        if !self.on_stack && !self.ptr.is_null() {
            let layout = Layout::from_size_align(self.len, MAX_ALIGNMENT)
                .expect("Invalid layout");
            unsafe {
                dealloc(self.ptr, layout);
            }
        }
    }
}

/// Safe stack allocation (returns None if size is too large)
pub fn safe_alloca(size: usize) -> Option<*mut u8> {
    if size < 4032 {
        let layout = Layout::from_size_align(size, MAX_ALIGNMENT).ok()?;
        Some(unsafe { alloca(layout.size()) })
    } else {
        None
    }
}

/// Free memory allocated by malloca
pub fn freea(malloca: Malloca) {
    drop(malloca); // Let Drop impl handle it
}

/// Overflow-safe array allocation
pub fn nmalloca(n: usize, size: usize) -> Option<Malloca> {
    n.checked_mul(size).and_then(|total| Malloca::new(total))
}

// Helper function to simulate alloca behavior
#[cfg(target_arch = "x86_64")]
unsafe fn alloca(size: usize) -> *mut u8 {
    let mut rsp: usize;
    asm!("mov {}, rsp", out(reg) rsp);
    rsp = (rsp - size) & !(MAX_ALIGNMENT - 1);
    rsp as *mut u8
}

#[cfg(not(target_arch = "x86_64"))]
unsafe fn alloca(_size: usize) -> *mut u8 {
    // Fallback implementation for non-x86_64 platforms
    ptr::null_mut()
}

// Dummy type for long double alignment
#[cfg(target_pointer_width = "64")]
type f128 = u128;

#[cfg(not(target_pointer_width = "64"))]
type f128 = u64;