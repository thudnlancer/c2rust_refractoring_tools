// Variable-sized buffer with on-stack default allocation.
// Copyright (C) 2015-2023 Free Software Foundation, Inc.
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

use std::alloc::{self, Layout};
use std::ptr;

pub struct ScratchBuffer {
    data: *mut u8,
    length: usize,
}

impl ScratchBuffer {
    pub fn new() -> Self {
        Self {
            data: ptr::null_mut(),
            length: 0,
        }
    }

    pub fn free(&mut self) {
        if !self.data.is_null() {
            unsafe {
                alloc::dealloc(
                    self.data,
                    Layout::from_size_align(self.length, 1).unwrap()
                );
            }
            self.data = ptr::null_mut();
            self.length = 0;
        }
    }

    pub fn grow(&mut self) -> Result<(), std::io::Error> {
        let new_length = self.length.checked_mul(2)
            .ok_or_else(|| std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                "Allocation size overflow"
            ))?;

        // Discard old buffer
        self.free();

        let layout = Layout::from_size_align(new_length, 1)
            .map_err(|_| std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Invalid layout"
            ))?;

        let new_ptr = unsafe { alloc::alloc(layout) };
        if new_ptr.is_null() {
            // Buffer must remain valid to free
            *self = Self::new();
            return Err(std::io::Error::new(
                std::io::ErrorKind::OutOfMemory,
                "Allocation failed"
            ));
        }

        // Install new heap-based buffer
        self.data = new_ptr;
        self.length = new_length;
        Ok(())
    }
}

impl Drop for ScratchBuffer {
    fn drop(&mut self) {
        self.free();
    }
}