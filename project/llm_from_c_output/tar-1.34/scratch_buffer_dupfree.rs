// Variable-sized buffer with on-stack default allocation.
// Copyright (C) 2020-2021 Free Software Foundation, Inc.
// This file is part of the GNU C Library.
//
// The GNU C Library is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation; either
// version 3 of the License, or (at your option) any later version.
//
// The GNU C Library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public
// License along with the GNU C Library; if not, see
// <https://www.gnu.org/licenses/>.

use std::alloc::{alloc, realloc, Layout};
use std::ptr::{self, NonNull};
use std::mem;

struct ScratchBuffer {
    data: *mut u8,
    space: [u8; 1024], // Assuming __space.__c is a fixed-size array
}

fn scratch_buffer_dupfree(buffer: &mut ScratchBuffer, size: usize) -> Option<NonNull<u8>> {
    let data = buffer.data;
    let space_ptr = buffer.space.as_ptr() as *mut u8;

    if data == space_ptr {
        // Allocate new memory and copy data
        let layout = Layout::from_size_align(size, mem::align_of::<u8>()).ok()?;
        let new_ptr = unsafe { alloc(layout) };
        if new_ptr.is_null() {
            return None;
        }
        unsafe {
            ptr::copy_nonoverlapping(data, new_ptr, size);
        }
        NonNull::new(new_ptr)
    } else {
        // Reallocate existing memory
        let layout = Layout::from_size_align(size, mem::align_of::<u8>()).ok()?;
        let new_ptr = unsafe { realloc(data, layout, size) };
        NonNull::new(new_ptr).or_else(|| NonNull::new(data))
    }
}