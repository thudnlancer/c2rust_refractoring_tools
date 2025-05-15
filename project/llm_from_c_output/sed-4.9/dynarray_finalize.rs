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

use std::alloc::{self, Layout};
use std::ptr;
use std::mem;

#[derive(Debug)]
pub struct DynarrayHeader {
    pub array: *mut u8,
    pub used: usize,
}

#[derive(Debug)]
pub struct DynarrayFinalizeResult {
    pub array: *mut u8,
    pub length: usize,
}

pub fn dynarray_error(list: &DynarrayHeader) -> bool {
    // Implementation of error checking would go here
    false
}

pub fn libc_dynarray_finalize(
    list: &mut DynarrayHeader,
    scratch: *mut u8,
    element_size: usize,
) -> Result<DynarrayFinalizeResult, ()> {
    if dynarray_error(list) {
        // The caller will report the deferred error.
        return Err(());
    }

    let used = list.used;

    // Empty list.
    if used == 0 {
        // An empty list could still be backed by a heap-allocated
        // array. Free it if necessary.
        if list.array != scratch && !list.array.is_null() {
            unsafe {
                let layout = Layout::array::<u8>(0).unwrap();
                alloc::dealloc(list.array, layout);
            }
        }
        return Ok(DynarrayFinalizeResult {
            array: ptr::null_mut(),
            length: 0,
        });
    }

    let allocation_size = used.checked_mul(element_size).ok_or(())?;
    let layout = Layout::array::<u8>(allocation_size).map_err(|_| ())?;
    let heap_array = unsafe { alloc::alloc(layout) };

    if !heap_array.is_null() {
        // The new array takes ownership of the data.
        if !list.array.is_null() {
            unsafe {
                ptr::copy_nonoverlapping(list.array, heap_array, allocation_size);
            }
        }
        if list.array != scratch && !list.array.is_null() {
            unsafe {
                let old_layout = Layout::array::<u8>(allocation_size).unwrap();
                alloc::dealloc(list.array, old_layout);
            }
        }
        Ok(DynarrayFinalizeResult {
            array: heap_array,
            length: used,
        })
    } else {
        // The caller will perform the freeing operation.
        Err(())
    }
}