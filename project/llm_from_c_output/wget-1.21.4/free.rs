/* Make free() preserve errno.

   Copyright (C) 2003, 2006, 2009-2023 Free Software Foundation, Inc.

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

/// A replacement for free() that preserves errno.
/// Only needed if HAVE_FREE_POSIX is not defined.
#[cfg(not(feature = "have_free_posix"))]
pub fn rpl_free<T>(p: *mut T) {
    // Save the current errno
    let saved_errno = io::Error::last_os_error();
    
    // Perform the free operation
    unsafe {
        let _ = Box::from_raw(p);
    }
    
    // Restore errno if it was changed
    let current_errno = io::Error::last_os_error();
    if current_errno.raw_os_error() != Some(0) {
        // If errno was set by free, leave it as is
        return;
    }
    
    // Restore the original errno
    if let Some(errno) = saved_errno.raw_os_error() {
        unsafe {
            *libc::__errno_location() = errno;
        }
    }
}