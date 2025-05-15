/*
 * stack.rs -- Rust implementation of stack functions for use by extensions.
 */

/*
 * Copyright (C) 2012, 2013 the Free Software Foundation, Inc.
 *
 * This file is part of GAWK, the GNU implementation of the
 * AWK Programming Language.
 *
 * GAWK is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * GAWK is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA
 */

use std::vec;

const INITIAL_STACK: usize = 20;

pub struct Stack {
    data: Vec<*mut std::ffi::c_void>,
    index: isize,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            data: Vec::with_capacity(INITIAL_STACK),
            index: -1,
        }
    }

    /// stack_empty --- return true if stack is empty
    pub fn is_empty(&self) -> bool {
        self.index < 0
    }

    /// stack_top --- return top object on the stack
    pub fn top(&self) -> Option<*mut std::ffi::c_void> {
        if self.is_empty() {
            None
        } else {
            Some(self.data[self.index as usize])
        }
    }

    /// stack_pop --- pop top object and return it
    pub fn pop(&mut self) -> Option<*mut std::ffi::c_void> {
        if self.is_empty() {
            None
        } else {
            let val = self.data[self.index as usize];
            self.index -= 1;
            Some(val)
        }
    }

    /// stack_push --- push an object onto the stack
    pub fn push(&mut self, object: *mut std::ffi::c_void) -> Result<(), ()> {
        if self.index + 1 >= self.data.capacity() as isize {
            let new_capacity = self.data.capacity() * 2;
            if new_capacity < self.data.capacity() {
                return Err(());
            }
            self.data.reserve(new_capacity - self.data.capacity());
        }

        self.index += 1;
        if self.index as usize >= self.data.len() {
            self.data.push(object);
        } else {
            self.data[self.index as usize] = object;
        }
        Ok(())
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        // Clean up any remaining pointers if needed
        self.data.clear();
    }
}