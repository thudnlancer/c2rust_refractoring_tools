// Increase the size of a dynamic array.
// Copyright (C) 2017-2022 Free Software Foundation, Inc.
// This file is part of the GNU C Library.
//
// The GNU C Library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// The GNU C Library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with the GNU C Library; if not, see
// <https://www.gnu.org/licenses/>.

use std::mem;
use std::ptr;
use std::alloc::{self, Layout};
use std::num::NonZeroUsize;

struct DynarrayHeader {
    array: Option<*mut u8>,
    allocated: usize,
    used: usize,
}

fn dynarray_resize(
    list: &mut DynarrayHeader,
    size: usize,
    scratch: Option<*mut u8>,
    element_size: usize,
) -> Result<(), alloc::LayoutError> {
    // The existing allocation provides sufficient room.
    if size <= list.allocated {
        list.used = size;
        return Ok(());
    }

    // Otherwise, use size as the new allocation size. The caller is
    // expected to provide the final size of the array, so there is no
    // over-allocation here.

    let new_size_bytes = size.checked_mul(element_size).ok_or(alloc::LayoutError)?;
    let new_layout = Layout::from_size_align(new_size_bytes, mem::align_of::<u8>())?;

    let new_array = if list.array == scratch {
        // The previous array was not heap-allocated.
        let new_ptr = unsafe { alloc::alloc(new_layout) };
        if new_ptr.is_null() {
            return Err(alloc::LayoutError);
        }
        if let Some(old_ptr) = list.array {
            unsafe {
                ptr::copy_nonoverlapping(
                    old_ptr,
                    new_ptr,
                    list.used * element_size,
                );
            }
        }
        Some(new_ptr)
    } else {
        let old_ptr = list.array.unwrap();
        let new_ptr = unsafe { alloc::realloc(old_ptr, new_layout, new_size_bytes) };
        if new_ptr.is_null() {
            return Err(alloc::LayoutError);
        }
        Some(new_ptr)
    };

    list.array = new_array;
    list.allocated = size;
    list.used = size;
    Ok(())
}