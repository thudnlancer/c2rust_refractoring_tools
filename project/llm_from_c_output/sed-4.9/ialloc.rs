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
use std::convert::TryInto;
use std::mem;
use std::ptr;

type Idx = isize;

fn alloc_nomem() -> Option<ptr::NonNull<u8>> {
    None
}

pub fn imalloc(s: Idx) -> Option<ptr::NonNull<u8>> {
    if s <= 0 {
        return alloc_nomem();
    }

    let size = match s.try_into() {
        Ok(size) => size,
        Err(_) => return alloc_nomem(),
    };

    let layout = match Layout::from_size_align(size, mem::align_of::<u8>()) {
        Ok(layout) => layout,
        Err(_) => return alloc_nomem(),
    };

    unsafe {
        let ptr = alloc(layout);
        if ptr.is_null() {
            alloc_nomem()
        } else {
            Some(ptr::NonNull::new_unchecked(ptr))
        }
    }
}

pub fn irealloc(p: Option<ptr::NonNull<u8>>, s: Idx) -> Option<ptr::NonNull<u8>> {
    let s = if s == 0 { 1 } else { s };
    
    if s <= 0 {
        return alloc_nomem();
    }

    let size = match s.try_into() {
        Ok(size) => size,
        Err(_) => return alloc_nomem(),
    };

    let layout = match Layout::from_size_align(size, mem::align_of::<u8>()) {
        Ok(layout) => layout,
        Err(_) => return alloc_nomem(),
    };

    let old_ptr = match p {
        Some(ptr) => ptr.as_ptr(),
        None => ptr::null_mut(),
    };

    unsafe {
        let new_ptr = realloc(old_ptr, layout, size);
        if new_ptr.is_null() {
            alloc_nomem()
        } else {
            Some(ptr::NonNull::new_unchecked(new_ptr))
        }
    }
}

pub fn icalloc(n: Idx, s: Idx) -> Option<ptr::NonNull<u8>> {
    let (n, s) = if n < 0 || s < 0 {
        return alloc_nomem();
    } else {
        (n as usize, s as usize)
    };

    let total_size = match n.checked_mul(s) {
        Some(size) => size,
        None => return alloc_nomem(),
    };

    let layout = match Layout::from_size_align(total_size, mem::align_of::<u8>()) {
        Ok(layout) => layout,
        Err(_) => return alloc_nomem(),
    };

    unsafe {
        let ptr = alloc_zeroed(layout);
        if ptr.is_null() {
            alloc_nomem()
        } else {
            Some(ptr::NonNull::new_unchecked(ptr))
        }
    }
}

pub fn ireallocarray(p: Option<ptr::NonNull<u8>>, n: Idx, s: Idx) -> Option<ptr::NonNull<u8>> {
    let (n, s) = if n == 0 || s == 0 {
        (1, 1)
    } else if n < 0 || s < 0 {
        return alloc_nomem();
    } else {
        (n as usize, s as usize)
    };

    let total_size = match n.checked_mul(s) {
        Some(size) => size,
        None => return alloc_nomem(),
    };

    let layout = match Layout::from_size_align(total_size, mem::align_of::<u8>()) {
        Ok(layout) => layout,
        Err(_) => return alloc_nomem(),
    };

    let old_ptr = match p {
        Some(ptr) => ptr.as_ptr(),
        None => ptr::null_mut(),
    };

    unsafe {
        let new_ptr = realloc(old_ptr, layout, total_size);
        if new_ptr.is_null() {
            alloc_nomem()
        } else {
            Some(ptr::NonNull::new_unchecked(new_ptr))
        }
    }
}