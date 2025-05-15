/* Make free() preserve errno.

   Copyright (C) 2003, 2006, 2009-2021 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* written by Paul Eggert */

use std::io;
use std::ptr;
use std::alloc::{dealloc, Layout};

pub fn rpl_free(p: *mut u8, layout: Layout) {
    // This Rust version doesn't need the GCC workaround since Rust's compiler
    // doesn't have the same optimization issue. We'll preserve the errno
    // behavior using std::io::Error.
    
    let saved_errno = io::Error::last_os_error();
    
    unsafe {
        dealloc(p, layout);
    }
    
    if saved_errno.raw_os_error().is_some() {
        io::set_last_os_error(saved_errno);
    }
}