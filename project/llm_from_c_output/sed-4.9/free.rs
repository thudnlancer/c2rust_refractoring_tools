/* Make free() preserve errno.

   Copyright (C) 2003, 2006, 2009-2022 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* written by Paul Eggert */

use std::io;
use std::ptr;

// Since Rust's memory management is handled by ownership system,
// we don't need to implement a direct equivalent of free().
// Instead, we provide a function that drops a Box while preserving errno.

pub fn rpl_free<T>(p: Box<T>) {
    // Save the current errno
    let err = io::Error::last_os_error();
    
    // Explicitly drop the Box to free the memory
    drop(p);
    
    // Restore the original errno
    if let Some(errno) = err.raw_os_error() {
        io::Error::set_last_os_error(errno);
    }
}

// For raw pointers (unsafe context), provide a separate function
#[cfg(feature = "unsafe")]
pub unsafe fn rpl_free_raw<T>(p: *mut T) {
    let err = io::Error::last_os_error();
    
    // Convert to Box and drop it
    if !p.is_null() {
        Box::from_raw(p);
    }
    
    if let Some(errno) = err.raw_os_error() {
        io::Error::set_last_os_error(errno);
    }
}