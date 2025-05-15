// ialloc.rs -- malloc with idx_t rather than size_t

// Copyright 2021-2022 Free Software Foundation, Inc.

// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.

// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::alloc::{alloc, alloc_zeroed, realloc, Layout};
use std::convert::TryFrom;
use std::num::NonZeroUsize;

type Idx = isize; // Assuming idx_t is equivalent to isize

fn alloc_nomem() -> Option<Box<[u8]>> {
    None
}

pub fn imalloc(s: Idx) -> Option<Box<[u8]>> {
    let size = usize::try_from(s).ok()?;
    let layout = Layout::array::<u8>(size).ok()?;
    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            None
        } else {
            Some(Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, size)))
        }
    }
}

pub fn irealloc(p: Option<Box<[u8]>>, s: Idx) -> Option<Box<[u8]>> {
    let size = usize::try_from(s).ok().map(|s| if s == 0 { 1 } else { s })?;
    let old_ptr = match p {
        Some(boxed) => boxed.as_ptr() as *mut u8,
        None => std::ptr::null_mut(),
    };
    let layout = Layout::array::<u8>(size).ok()?;
    unsafe {
        let ptr = realloc(old_ptr, layout, size);
        if ptr.is_null() {
            None
        } else {
            Some(Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, size)))
        }
    }
}

pub fn icalloc(n: Idx, s: Idx) -> Option<Box<[u8]>> {
    let n = usize::try_from(n).ok().filter(|&n| n <= usize::MAX)?;
    let s = usize::try_from(s).ok().filter(|&s| s <= usize::MAX)?;
    let size = n.checked_mul(s)?;
    let layout = Layout::array::<u8>(size).ok()?;
    unsafe {
        let ptr = alloc_zeroed(layout);
        if ptr.is_null() {
            None
        } else {
            Some(Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, size)))
        }
    }
}

pub fn ireallocarray(p: Option<Box<[u8]>>, n: Idx, s: Idx) -> Option<Box<[u8]>> {
    let n = if n == 0 { 1 } else { usize::try_from(n).ok()? };
    let s = if s == 0 { 1 } else { usize::try_from(s).ok()? };
    let size = n.checked_mul(s)?;
    let old_ptr = match p {
        Some(boxed) => boxed.as_ptr() as *mut u8,
        None => std::ptr::null_mut(),
    };
    let layout = Layout::array::<u8>(size).ok()?;
    unsafe {
        let ptr = realloc(old_ptr, layout, size);
        if ptr.is_null() {
            None
        } else {
            Some(Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, size)))
        }
    }
}