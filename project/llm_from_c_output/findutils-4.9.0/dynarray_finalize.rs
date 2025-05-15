/* Copy the dynamically-allocated area to an explicitly-sized heap allocation.
   Copyright (C) 2017-2022 Free Software Foundation, Inc.
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

use std::alloc::{alloc, dealloc, Layout};
use std::ptr::{self, NonNull};
use std::mem;

#[derive(Debug)]
pub struct DynarrayHeader {
    array: Option<NonNull<u8>>,
    used: usize,
    // Other fields as needed
}

#[derive(Debug)]
pub struct DynarrayFinalizeResult<T> {
    pub array: Option<Box<[T]>>,
    pub length: usize,
}

pub fn dynarray_error(list: &DynarrayHeader) -> bool {
    // Implement error checking logic
    false
}

pub fn libc_dynarray_finalize<T>(
    list: &mut DynarrayHeader,
    scratch: Option<NonNull<u8>>,
    element_size: usize,
) -> Result<DynarrayFinalizeResult<T>, ()> {
    if dynarray_error(list) {
        // The caller will report the deferred error.
        return Err(());
    }

    let used = list.used;

    // Empty list.
    if used == 0 {
        // An empty list could still be backed by a heap-allocated array.
        // Free it if necessary.
        if let Some(array) = list.array {
            if scratch.map_or(true, |s| array != s) {
                unsafe {
                    let layout = Layout::array::<u8>(used * element_size).unwrap();
                    dealloc(array.as_ptr(), layout);
                }
            }
        }
        return Ok(DynarrayFinalizeResult {
            array: None,
            length: 0,
        });
    }

    let allocation_size = used * element_size;
    let layout = Layout::array::<u8>(allocation_size).unwrap();

    let heap_array = unsafe { alloc(layout) };
    if !heap_array.is_null() {
        // The new array takes ownership of the data.
        if let Some(array) = list.array {
            unsafe {
                ptr::copy_nonoverlapping(array.as_ptr(), heap_array, allocation_size);
            }
        }

        if let Some(array) = list.array {
            if scratch.map_or(true, |s| array != s) {
                unsafe {
                    dealloc(array.as_ptr(), layout);
                list.array = None;
                }
            }
        }

        // Convert the raw pointer to a Box<[T]>
        let boxed_slice = unsafe {
            let slice_ptr = ptr::slice_from_raw_parts_mut(heap_array as *mut T, used);
            Box::from_raw(slice_ptr)
        };

        Ok(DynarrayFinalizeResult {
            array: Some(boxed_slice),
            length: used,
        })
    } else {
        // The caller will perform the freeing operation.
        Err(())
    }
}