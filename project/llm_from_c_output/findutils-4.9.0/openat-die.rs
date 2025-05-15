/* Report a save- or restore-cwd failure in our openat replacement and then exit.

   Copyright (C) 2005-2006, 2008-2022 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::process;
use std::io::{self, Error};
use gettextrs::{gettext};

const EXIT_FAILURE: i32 = 1;

#[no_mangle]
pub fn openat_save_fail(errnum: i32) -> ! {
    #[cfg(not(feature = "gnulib_libposix"))]
    {
        eprintln!("{}: {}", gettext("unable to record current working directory"), Error::from_raw_os_error(errnum));
    }
    
    // Safety net in case exit_failure is 0
    process::abort();
}

#[no_mangle]
pub fn openat_restore_fail(errnum: i32) -> ! {
    #[cfg(not(feature = "gnulib_libposix"))]
    {
        eprintln!("{}: {}", gettext("failed to return to initial working directory"), Error::from_raw_os_error(errnum));
    }
    
    // Safety net in case exit_failure is 0
    process::abort();
}