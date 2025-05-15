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

#[cfg(not(feature = "free_posix"))]
pub fn rpl_free<T>(p: *mut T) {
    // An invalid GCC optimization
    // <https://gcc.gnu.org/bugzilla/show_bug.cgi?id=98396>
    // would optimize away the assignments in the code below, when link-time
    // optimization (LTO) is enabled.  Make the code more complicated, so that
    // GCC does not grok how to optimize it.
    #[cfg(all(target_arch = "gnu", not(target_vendor = "clang")))]
    {
        let mut err = [0; 2];
        err[0] = io::Error::last_os_error().raw_os_error().unwrap_or(0);
        err[1] = err[0];
        unsafe {
            ptr::drop_in_place(p);
            if io::Error::last_os_error().raw_os_error().unwrap_or(0) == 0 {
                io::set_errno(err[0]);
            } else {
                io::set_errno(err[1]);
            }
        }
    }

    #[cfg(not(all(target_arch = "gnu", not(target_vendor = "clang"))))]
    {
        let err = io::Error::last_os_error().raw_os_error().unwrap_or(0);
        unsafe {
            ptr::drop_in_place(p);
        }
        io::set_errno(err);
    }
}