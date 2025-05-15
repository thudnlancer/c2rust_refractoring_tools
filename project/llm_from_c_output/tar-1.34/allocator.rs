//! Memory allocators such as malloc+free.
//!
//! This module provides an abstraction over memory allocation strategies.
//!
//! Original C code copyright:
//! Copyright (C) 2011-2021 Free Software Foundation, Inc.
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation; either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.
//!
//! Written by Paul Eggert.

use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr::NonNull;
use std::num::NonZeroUsize;

/// An object describing a memory allocator family.
pub struct Allocator {
    /// Call allocate to allocate memory. On failure should return None.
    /// When given a zero size it may return None even if successful.
    pub allocate: fn(size: NonZeroUsize) -> Option<NonNull<u8>>,

    /// If Some, call reallocate to reallocate memory.
    /// On failure should return None.
    /// When given a zero size it may return None even if successful.
    pub reallocate: Option<fn(ptr: NonNull<u8>, new_size: NonZeroUsize) -> Option<NonNull<u8>>>,

    /// Call free to free memory.
    pub free: fn(ptr: NonNull<u8>),

    /// If Some, call die(size) if allocation fails.
    /// die should not return. SIZE should equal usize::MAX if overflow was detected.
    pub die: Option<fn(size: usize)>,
}

/// An allocator using the stdlib functions and a null DIE function.
pub const STDLIB_ALLOCATOR: Allocator = Allocator {
    allocate: |size| {
        let layout = Layout::from_size_align(size.get(), std::mem::align_of::<u8>()).ok()?;
        // SAFETY: Layout is valid (checked above) and size is non-zero
        unsafe { NonNull::new(alloc(layout)) }
    },
    reallocate: Some(|ptr, new_size| {
        let old_layout = Layout::from_size_align(1, std::mem::align_of::<u8>()).unwrap();
        let new_layout = Layout::from_size_align(new_size.get(), std::mem::align_of::<u8>()).ok()?;
        // SAFETY: ptr was allocated with the global allocator, old_layout matches,
        // and new_layout is valid (checked above)
        unsafe { NonNull::new(realloc(ptr.as_ptr(), old_layout, new_layout.size())) }
    }),
    free: |ptr| {
        let layout = Layout::from_size_align(1, std::mem::align_of::<u8>()).unwrap();
        // SAFETY: ptr was allocated with the global allocator and layout matches
        unsafe { dealloc(ptr.as_ptr(), layout) }
    },
    die: None,
};