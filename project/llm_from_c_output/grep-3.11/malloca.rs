//! Safe automatic memory allocation in Rust

use std::alloc::{alloc, dealloc, Layout};
use std::mem;
use std::ptr;

/// The maximum alignment required for any fundamental type
const MAX_ALIGNMENT: usize = {
    let align_long = mem::align_of::<usize>();
    let align_double = mem::align_of::<f64>();
    let align_longlong = mem::align_of::<u64>();
    let align_longdouble = mem::align_of::<f128>();
    
    ((align_long - 1) | (align_double - 1) | (align_longlong - 1) | (align_longdouble - 1)) + 1
};

/// A wrapper for stack or heap allocated memory that must be freed
pub enum Malloca {
    Stack(*mut u8),
    Heap(*mut u8),
    None,
}

impl Malloca {
    /// Allocate memory on stack or heap
    pub fn new(size: usize) -> Option<Self> {
        if size < 4032 - (2 * MAX_ALIGNMENT - 1) {
            // Try stack allocation
            let layout = Layout::from_size_align(size, MAX_ALIGNMENT).ok()?;
            let ptr = unsafe { alloc(layout) };
            if !ptr.is_null() {
                Some(Self::Stack(ptr))
            } else {
                None
            }
        } else {
            // Fall back to heap allocation
            Self::mmalloca(size)
        }
    }

    /// Allocate memory on heap with proper alignment
    fn mmalloca(size: usize) -> Option<Self> {
        let layout = Layout::from_size_align(size, MAX_ALIGNMENT).ok()?;
        let ptr = unsafe { alloc(layout) };
        if !ptr.is_null() {
            Some(Self::Heap(ptr))
        } else {
            None
        }
    }

    /// Allocate array with overflow check
    pub fn array(count: usize, element_size: usize) -> Option<Self> {
        count.checked_mul(element_size).and_then(|size| Self::new(size))
    }

    /// Get raw pointer to allocated memory
    pub fn as_ptr(&self) -> *mut u8 {
        match self {
            Self::Stack(ptr) | Self::Heap(ptr) => *ptr,
            Self::None => ptr::null_mut(),
        }
    }
}

impl Drop for Malloca {
    fn drop(&mut self) {
        if let Self::Heap(ptr) = self {
            if !ptr.is_null() {
                let layout = Layout::from_size_align(0, MAX_ALIGNMENT).unwrap();
                unsafe { dealloc(*ptr, layout) };
            }
        }
        // Stack allocations are automatically freed when going out of scope
    }
}

/// 128-bit floating point type for alignment calculation
#[cfg(target_arch = "x86_64")]
#[repr(C)]
#[derive(Copy, Clone)]
struct f128(f64, f64);

#[cfg(not(target_arch = "x86_64"))]
#[repr(C)]
#[derive(Copy, Clone)]
struct f128(u64, u64);