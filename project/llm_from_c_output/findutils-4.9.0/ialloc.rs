/*
 * ialloc.rs -- memory allocation with idx_t rather than size_t
 *
 * Copyright 2021-2022 Free Software Foundation, Inc.
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

use std::alloc::{alloc, alloc_zeroed, dealloc, realloc, Layout};
use std::convert::TryFrom;
use std::num::NonZeroUsize;
use std::ptr::NonNull;

type Idx = isize; // Assuming idx_t maps to isize in Rust

#[derive(Debug)]
pub enum AllocError {
    OutOfMemory,
    InvalidSize,
}

fn alloc_nomem() -> Result<NonNull<u8>, AllocError> {
    Err(AllocError::OutOfMemory)
}

pub fn imalloc(s: Idx) -> Result<NonNull<u8>, AllocError> {
    let size = usize::try_from(s).map_err(|_| AllocError::InvalidSize)?;
    if size == 0 {
        return Ok(NonNull::dangling());
    }
    let layout = Layout::from_size_align(size, 1).map_err(|_| AllocError::InvalidSize)?;
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            alloc_nomem()
        } else {
            Ok(NonNull::new_unchecked(ptr))
        }
    }
}

pub fn irealloc(p: Option<NonNull<u8>>, s: Idx) -> Result<NonNull<u8>, AllocError> {
    let size = usize::try_from(s).map_err(|_| AllocError::InvalidSize)?;
    let size = size.max(1); // Work around zero size case
    let layout = Layout::from_size_align(size, 1).map_err(|_| AllocError::InvalidSize)?;
    
    unsafe {
        let ptr = match p {
            Some(ptr) => realloc(ptr.as_ptr(), layout, size),
            None => alloc(layout),
        };
        if ptr.is_null() {
            alloc_nomem()
        } else {
            Ok(NonNull::new_unchecked(ptr))
        }
    }
}

pub fn icalloc(n: Idx, s: Idx) -> Result<NonNull<u8>, AllocError> {
    let n = usize::try_from(n).unwrap_or(0);
    let s = usize::try_from(s).unwrap_or(0);
    
    if n == 0 || s == 0 {
        return Ok(NonNull::dangling());
    }
    
    let size = n.checked_mul(s).ok_or(AllocError::InvalidSize)?;
    let layout = Layout::from_size_align(size, 1).map_err(|_| AllocError::InvalidSize)?;
    
    unsafe {
        let ptr = alloc_zeroed(layout);
        if ptr.is_null() {
            alloc_nomem()
        } else {
            Ok(NonNull::new_unchecked(ptr))
        }
    }
}

pub fn ireallocarray(
    p: Option<NonNull<u8>>,
    n: Idx,
    s: Idx,
) -> Result<NonNull<u8>, AllocError> {
    let n = usize::try_from(n).unwrap_or(1);
    let s = usize::try_from(s).unwrap_or(1);
    let n = n.max(1);
    let s = s.max(1);
    
    let size = n.checked_mul(s).ok_or(AllocError::InvalidSize)?;
    let layout = Layout::from_size_align(size, 1).map_err(|_| AllocError::InvalidSize)?;
    
    unsafe {
        let ptr = match p {
            Some(ptr) => realloc(ptr.as_ptr(), layout, size),
            None => alloc(layout),
        };
        if ptr.is_null() {
            alloc_nomem()
        } else {
            Ok(NonNull::new_unchecked(ptr))
        }
    }
}

pub fn ifree(p: Option<NonNull<u8>>, s: Idx) {
    if let Some(ptr) = p {
        let size = usize::try_from(s).unwrap_or(0);
        if size > 0 {
            let layout = Layout::from_size_align(size, 1).unwrap();
            unsafe {
                dealloc(ptr.as_ptr(), layout);
            }
        }
    }
}