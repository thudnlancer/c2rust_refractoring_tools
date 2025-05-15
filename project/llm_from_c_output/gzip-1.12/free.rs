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
use std::alloc::{dealloc, Layout};

/// A safe wrapper around dealloc that preserves errno
pub fn rpl_free(p: *mut u8, layout: Layout) {
    // Save the current errno value
    let err = io::Error::last_os_error();
    let errno = err.raw_os_error().unwrap_or(0);
    
    // Perform the deallocation
    if !p.is_null() {
        unsafe { dealloc(p, layout) };
    }
    
    // Restore the original errno value
    if errno != 0 {
        io::set_errno(errno);
    }
}