//! Safe automatic memory allocation.
//! 
//! This module provides safe alternatives to alloca/malloca functionality in Rust,
//! using Rust's ownership system to ensure memory safety.

use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

/// The desired alignment of memory allocations is the maximum alignment
/// among all elementary types.
const SA_ALIGNMENT_MAX: usize = {
    let align_long = mem::align_of::<usize>();
    let align_double = mem::align_of::<f64>();
    let align_longlong = mem::align_of::<u64>();
    let align_longdouble = mem::align_of::<f64>(); // Rust doesn't have long double
    
    ((align_long - 1) | (align_double - 1) | (align_longlong - 1) | (align_longdouble - 1)) + 1
};

/// A wrapper for stack or heap allocated memory that must be freed.
pub struct Malloca {
    ptr: *mut u8,
    len: usize,
    on_stack: bool,
}

impl Malloca {
    /// Allocate memory on stack or heap.
    pub fn new(n: usize) -> Option<Self> {
        if n < 4032 - (2 * SA_ALIGNMENT_MAX - 1) {
            // Try stack allocation
            unsafe {
                let layout = Layout::from_size_align(n, SA_ALIGNMENT_MAX).ok()?;
                let ptr = std::alloc::alloc(layout);
                if !ptr.is_null() {
                    Some(Self {
                        ptr,
                        len: n,
                        on_stack: true,
                    })
                } else {
                    None
                }
            }
        } else {
            // Fall back to heap allocation
            mmalloca(n).map(|ptr| Self {
                ptr,
                len: n,
                on_stack: false,
            })
        }
    }

    /// Get a mutable pointer to the allocated memory.
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr
    }

    /// Get the length of the allocated memory.
    pub fn len(&self) -> usize {
        self.len
    }
}

impl Drop for Malloca {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe {
                let layout = Layout::from_size_align(self.len, SA_ALIGNMENT_MAX)
                    .expect("Invalid layout");
                if self.on_stack {
                    // Stack allocation doesn't need explicit freeing in Rust
                    // as it's tied to the scope
                } else {
                    dealloc(self.ptr, layout);
                }
            }
        }
    }
}

/// Allocate memory on the heap with proper alignment.
fn mmalloca(n: usize) -> Option<*mut u8> {
    if n == 0 {
        return None;
    }

    let alignment2_mask = 2 * SA_ALIGNMENT_MAX - 1;
    let plus = mem::size_of::<u8>() + alignment2_mask;
    let nplus = n.checked_add(plus)?;

    unsafe {
        let layout = Layout::from_size_align(nplus, 1).ok()?;
        let mem = alloc(layout);
        if mem.is_null() {
            return None;
        }

        let umem = mem as usize;
        let umemplus = umem.checked_add(mem::size_of::<u8>() + SA_ALIGNMENT_MAX - 1)?;
        let offset = ((umemplus & !alignment2_mask) + SA_ALIGNMENT_MAX - umem);
        let vp = mem.add(offset);
        let p = vp as *mut u8;
        
        // Store the offset
        ptr::write((p as *mut u8).offset(-1) as *mut u8, offset as u8);
        
        Some(p)
    }
}

/// Safe wrapper for stack allocation.
pub fn safe_alloca(n: usize) -> Option<Malloca> {
    if n < 4032 {
        Malloca::new(n)
    } else {
        None
    }
}

/// Overflow-safe variant of malloca for arrays.
pub fn nmalloca(n: usize, s: usize) -> Option<Malloca> {
    if n.checked_mul(s).is_none() {
        None
    } else {
        Malloca::new(n * s)
    }
}