// buffer-init.rs

//   Copyright (C) 2002 Niels Möller
//
//   This file is part of GNU Nettle.
//
//   GNU Nettle is free software: you can redistribute it and/or
//   modify it under the terms of either:
//
//     * the GNU Lesser General Public License as published by the Free
//       Software Foundation; either version 3 of the License, or (at your
//       option) any later version.
//
//   or
//
//     * the GNU General Public License as published by the Free
//       Software Foundation; either version 2 of the License, or (at your
//       option) any later version.
//
//   or both in parallel, as here.
//
//   GNU Nettle is distributed in the hope that it will be useful,
//   but WITHOUT ANY WARRANTY; without even the implied warranty of
//   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
//   General Public License for more details.
//
//   You should have received copies of the GNU General Public License and
//   GNU Lesser General Public License along with this program.  If
//   not, see http://www.gnu.org/licenses/.

use std::alloc::{alloc, dealloc, realloc, Layout};
use std::ptr;

pub struct NettleBuffer {
    data: *mut u8,
    size: usize,
    alloc: usize,
}

impl NettleBuffer {
    pub fn init(&mut self) {
        self.init_realloc(None, |ptr, old_size, new_size| {
            if new_size == 0 {
                if !ptr.is_null() {
                    unsafe {
                        dealloc(ptr as *mut u8, Layout::from_size_align(old_size, 1).unwrap());
                    }
                }
                ptr::null_mut()
            } else if ptr.is_null() {
                unsafe {
                    alloc(Layout::from_size_align(new_size, 1).unwrap())
                }
            } else {
                unsafe {
                    realloc(ptr as *mut u8, Layout::from_size_align(old_size, 1).unwrap(), new_size)
                }
            }
        });
    }

    fn init_realloc<F>(&mut self, initial_data: Option<&[u8]>, realloc_fn: F)
    where
        F: Fn(*mut libc::c_void, usize, usize) -> *mut libc::c_void,
    {
        self.data = ptr::null_mut();
        self.size = 0;
        self.alloc = 0;

        if let Some(data) = initial_data {
            let new_size = data.len();
            let new_data = realloc_fn(ptr::null_mut(), 0, new_size);
            if !new_data.is_null() {
                unsafe {
                    ptr::copy_nonoverlapping(data.as_ptr(), new_data as *mut u8, new_size);
                }
                self.data = new_data as *mut u8;
                self.size = new_size;
                self.alloc = new_size;
            }
        }
    }
}