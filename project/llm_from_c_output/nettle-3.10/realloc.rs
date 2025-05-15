// realloc.rs

// Copyright (C) 2002 Niels Möller
//
// This file is part of GNU Nettle.
//
// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:
//
//   * the GNU Lesser General Public License as published by the Free
//     Software Foundation; either version 3 of the License, or (at your
//     option) any later version.
//
// or
//
//   * the GNU General Public License as published by the Free
//     Software Foundation; either version 2 of the License, or (at your
//     option) any later version.
//
// or both in parallel, as here.
//
// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use std::alloc::{alloc, dealloc, realloc, Layout};
use std::io::{stderr, Write};
use std::process;

pub type NettleReallocFunc = fn(ctx: Option<&mut ()>, ptr: Option<Box<[u8]>>, length: usize) -> Option<Box<[u8]>>;

/// NOTE: Calling Rust's realloc with length == 0 is not required to
/// totally free the object, it is allowed to return a valid pointer.
pub fn nettle_realloc(_ctx: Option<&mut ()>, ptr: Option<Box<[u8]>>, length: usize) -> Option<Box<[u8]>> {
    if length > 0 {
        if let Some(mut boxed_slice) = ptr {
            unsafe {
                let layout = Layout::array::<u8>(boxed_slice.len()).unwrap();
                let ptr = realloc(
                    boxed_slice.as_mut_ptr() as *mut u8,
                    layout,
                    length
                );
                if ptr.is_null() {
                    return None;
                }
                Some(Box::from_raw(std::slice::from_raw_parts_mut(ptr, length)))
            }
        } else {
            unsafe {
                let layout = Layout::array::<u8>(length).unwrap();
                let ptr = alloc(layout);
                if ptr.is_null() {
                    return None;
                }
                Some(Box::from_raw(std::slice::from_raw_parts_mut(ptr, length)))
            }
        }
    } else {
        // Free the memory by letting the Box drop
        None
    }
}

pub fn nettle_xrealloc(_ctx: Option<&mut ()>, ptr: Option<Box<[u8]>>, length: usize) -> Option<Box<[u8]>> {
    if length > 0 {
        let result = nettle_realloc(_ctx, ptr, length);
        if result.is_none() {
            let _ = writeln!(stderr(), "Virtual memory exhausted.");
            process::abort();
        }
        result
    } else {
        // Free the memory by letting the Box drop
        None
    }
}