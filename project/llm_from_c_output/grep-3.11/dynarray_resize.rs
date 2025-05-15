/* Increase the size of a dynamic array.
   Copyright (C) 2017-2023 Free Software Foundation, Inc.
   This file is part of the GNU C Library.

   The GNU C Library is free software; you can redistribute it and/or
   modify it under the terms of the GNU Lesser General Public
   License as published by the Free Software Foundation; either
   version 2.1 of the License, or (at your option) any later version.

   The GNU C Library is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public
   License along with the GNU C Library; if not, see
   <https://www.gnu.org/licenses/>.  */

use std::alloc::{self, Layout};
use std::mem;
use std::ptr;
use std::num::TryFromIntError;

struct DynarrayHeader {
    array: Option<Box<[u8]>>,
    allocated: usize,
    used: usize,
}

fn dynarray_resize(
    list: &mut DynarrayHeader,
    size: usize,
    scratch: Option<&[u8]>,
    element_size: usize,
) -> Result<(), TryFromIntError> {
    // The existing allocation provides sufficient room.
    if size <= list.allocated {
        list.used = size;
        return Ok(());
    }

    // Otherwise, use size as the new allocation size. The caller is
    // expected to provide the final size of the array, so there is no
    // over-allocation here.

    let new_size_bytes = size.checked_mul(element_size).ok_or_else(|| {
        TryFromIntError(())
    })?;

    let new_array = if list.array.is_none() && scratch.is_some() {
        // The previous array was not heap-allocated.
        let mut new_vec = Vec::with_capacity(new_size_bytes);
        if let Some(scratch_data) = scratch {
            new_vec.extend_from_slice(scratch_data);
        }
        Some(new_vec.into_boxed_slice())
    } else {
        let mut new_vec = Vec::with_capacity(new_size_bytes);
        if let Some(existing) = &list.array {
            new_vec.extend_from_slice(existing);
        }
        Some(new_vec.into_boxed_slice())
    };

    list.array = new_array;
    list.allocated = size;
    list.used = size;
    Ok(())
}