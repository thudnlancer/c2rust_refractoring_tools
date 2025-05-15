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
use std::convert::TryFrom;
use std::io::Error;
use std::ptr::NonNull;

type Idx = isize; // Assuming idx_t maps to isize

#[cold]
fn alloc_nomem() -> Result<NonNull<u8>, Error> {
    Err(Error::from_raw_os_error(libc::ENOMEM))
}

/// imalloc(size) is like malloc(size).
/// It returns a non-NULL pointer to size bytes of memory.
/// Upon failure, it returns an error.
pub fn imalloc(size: Idx) -> Result<NonNull<u8>, Error> {
    match usize::try_from(size) {
        Ok(size) if size > 0 => {
            let layout = Layout::from_size_align(size, std::mem::align_of::<u8>())?;
            unsafe { NonNull::new(alloc(layout)).ok_or_else(alloc_nomem) }
        }
        _ => alloc_nomem(),
    }
}

/// irealloc(ptr, size) is like realloc(ptr, size).
/// It returns a non-NULL pointer to size bytes of memory.
/// Upon failure, it returns an error.
pub fn irealloc(ptr: Option<NonNull<u8>>, size: Idx) -> Result<NonNull<u8>, Error> {
    match usize::try_from(size) {
        Ok(size) if size > 0 => {
            let layout = Layout::from_size_align(size, std::mem::align_of::<u8>())?;
            unsafe {
                NonNull::new(realloc(
                    ptr.map(|p| p.as_ptr()).unwrap_or(std::ptr::null_mut()),
                    layout,
                    size,
                ))
                .ok_or_else(alloc_nomem)
            }
        }
        _ => alloc_nomem(),
    }
}

/// icalloc(num, size) is like calloc(num, size).
/// It returns a non-NULL pointer to num * size bytes of memory.
/// Upon failure, it returns an error.
pub fn icalloc(num: Idx, size: Idx) -> Result<NonNull<u8>, Error> {
    let (num, size) = match (usize::try_from(num), usize::try_from(size)) {
        (Ok(n), Ok(s)) => (n, s),
        _ => return alloc_nomem(),
    };

    if num == 0 || size == 0 {
        return alloc_nomem();
    }

    let total_size = num.checked_mul(size).ok_or_else(alloc_nomem)?;
    let layout = Layout::from_size_align(total_size, std::mem::align_of::<u8>())?;
    unsafe { NonNull::new(alloc_zeroed(layout)).ok_or_else(alloc_nomem) }
}

/// ireallocarray(ptr, num, size) is like reallocarray(ptr, num, size).
/// It returns a non-NULL pointer to num * size bytes of memory.
/// Upon failure, it returns an error.
pub fn ireallocarray(
    ptr: Option<NonNull<u8>>,
    num: Idx,
    size: Idx,
) -> Result<NonNull<u8>, Error> {
    let (num, size) = match (usize::try_from(num), usize::try_from(size)) {
        (Ok(n), Ok(s)) => (n, s),
        _ => return alloc_nomem(),
    };

    if num == 0 || size == 0 {
        return alloc_nomem();
    }

    let total_size = num.checked_mul(size).ok_or_else(alloc_nomem)?;
    let layout = Layout::from_size_align(total_size, std::mem::align_of::<u8>())?;
    unsafe {
        NonNull::new(realloc(
            ptr.map(|p| p.as_ptr()).unwrap_or(std::ptr::null_mut()),
            layout,
            total_size,
        ))
        .ok_or_else(alloc_nomem)
    }
}