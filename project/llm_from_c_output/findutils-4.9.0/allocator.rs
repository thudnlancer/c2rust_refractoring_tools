//! Memory allocators such as Box and Vec.
//!
//! This module provides an interface similar to the C allocator interface,
//! but implemented using Rust's safe memory management constructs.

use std::alloc::{alloc, realloc, dealloc, Layout};
use std::ptr::NonNull;
use std::num::NonZeroUsize;

/// An object describing a memory allocator family.
pub struct Allocator {
    /// Call `allocate` to allocate memory. On failure should return `None`.
    /// When given a zero size it may return `None` even if successful.
    pub allocate: fn(size: usize) -> Option<NonNull<u8>>,
    
    /// Call `reallocate` to reallocate memory. On failure should return `None`.
    /// When given a zero size it may return `None` even if successful.
    pub reallocate: fn(ptr: NonNull<u8>, new_size: usize) -> Option<NonNull<u8>>,
    
    /// Call `free` to free memory.
    pub free: fn(ptr: NonNull<u8>),
    
    /// If non-None, call `die` if allocation fails.
    /// The size argument should be `SIZE_MAX` if size_t overflow was detected.
    pub die: Option<fn(size: usize)>,
}

/// Safe wrapper around std alloc functions
fn std_allocate(size: usize) -> Option<NonNull<u8>> {
    if size == 0 {
        return None;
    }
    let layout = Layout::from_size_align(size, 1).ok()?;
    // SAFETY: size is non-zero and layout is valid
    unsafe { NonNull::new(alloc(layout).cast()) }
}

fn std_reallocate(ptr: NonNull<u8>, new_size: usize) -> Option<NonNull<u8>> {
    let old_layout = Layout::from_size_align(1, 1).unwrap(); // dummy layout
    // SAFETY: ptr came from allocator, new_size may be 0
    unsafe { NonNull::new(realloc(ptr.as_ptr().cast(), old_layout, new_size).cast()) }
}

fn std_free(ptr: NonNull<u8>) {
    let layout = Layout::from_size_align(1, 1).unwrap(); // dummy layout
    // SAFETY: ptr came from allocator
    unsafe { dealloc(ptr.as_ptr().cast(), layout); }
}

/// An allocator using the stdlib functions and no die function.
pub const STDLIB_ALLOCATOR: Allocator = Allocator {
    allocate: std_allocate,
    reallocate: std_reallocate,
    free: std_free,
    die: None,
};