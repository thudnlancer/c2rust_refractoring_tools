/*
 * ialloc.rs -- malloc with idx_t rather than size_t
 *
 * Copyright 2021-2023 Free Software Foundation, Inc.
 *
 * This file is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of the
 * License, or (at your option) any later version.
 *
 * This file is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::alloc::{alloc, alloc_zeroed, realloc, Layout};
use std::num::NonZeroUsize;
use std::ptr::NonNull;

type Idx = isize; // Assuming idx_t maps to isize

#[cold]
fn alloc_nomem() -> Result<NonNull<u8>, std::alloc::AllocError> {
    Err(std::alloc::AllocError)
}

/// imalloc (size) is like malloc (size).
/// It returns a non-NULL pointer to size bytes of memory.
/// Upon failure, it returns an error.
pub fn imalloc(s: Idx) -> Result<NonNull<u8>, std::alloc::AllocError> {
    if s <= 0 {
        return alloc_nomem();
    }
    let size = s as usize;
    let layout = Layout::from_size_align(size, std::mem::align_of::<u8>())?;
    unsafe { NonNull::new(alloc(layout)).ok_or(std::alloc::AllocError) }
}

/// irealloc (ptr, size) is like realloc (ptr, size).
/// It returns a non-NULL pointer to size bytes of memory.
/// Upon failure, it returns an error.
pub fn irealloc(
    ptr: NonNull<u8>,
    old_size: Idx,
    s: Idx,
) -> Result<NonNull<u8>, std::alloc::AllocError> {
    if s <= 0 {
        return alloc_nomem();
    }
    let size = s as usize;
    let old_size = old_size as usize;
    let layout = Layout::from_size_align(old_size, std::mem::align_of::<u8>())?;
    unsafe { NonNull::new(realloc(ptr.as_ptr(), layout, size)).ok_or(std::alloc::AllocError) }
}

/// icalloc (num, size) is like calloc (num, size).
/// It returns a non-NULL pointer to num * size bytes of memory.
/// Upon failure, it returns an error.
pub fn icalloc(n: Idx, s: Idx) -> Result<NonNull<u8>, std::alloc::AllocError> {
    if n <= 0 || s <= 0 {
        return alloc_nomem();
    }
    let total = match n.checked_mul(s) {
        Some(v) if v > 0 => v as usize,
        _ => return alloc_nomem(),
    };
    let layout = Layout::from_size_align(total, std::mem::align_of::<u8>())?;
    unsafe { NonNull::new(alloc_zeroed(layout)).ok_or(std::alloc::AllocError) }
}

/// ireallocarray (ptr, num, size) is like reallocarray (ptr, num, size).
/// It returns a non-NULL pointer to num * size bytes of memory.
/// Upon failure, it returns an error.
pub fn ireallocarray(
    ptr: NonNull<u8>,
    old_size: Idx,
    n: Idx,
    s: Idx,
) -> Result<NonNull<u8>, std::alloc::AllocError> {
    if n <= 0 || s <= 0 {
        return alloc_nomem();
    }
    let total = match n.checked_mul(s) {
        Some(v) if v > 0 => v as usize,
        _ => return alloc_nomem(),
    };
    let old_size = old_size as usize;
    let layout = Layout::from_size_align(old_size, std::mem::align_of::<u8>())?;
    unsafe { NonNull::new(realloc(ptr.as_ptr(), layout, total)).ok_or(std::alloc::AllocError) }
}

/*
 * malloc with idx_t rather than size_t
 *
 * Copyright 2021-2023 Free Software Foundation, Inc.
 *
 * This file is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of the
 * License, or (at your option) any later version.
 *
 * This file is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */