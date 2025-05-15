/* Report a save- or restore-cwd failure in our openat replacement and then exit.

   Copyright (C) 2005-2006, 2008-2023 Free Software Foundation, Inc.

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
use gettextrs::gettext;

static EXIT_FAILURE: i32 = 1;

#[no_mangle]
pub extern "C" fn openat_save_fail(errnum: i32) -> ! {
    // In Rust, we'll use eprintln! for error reporting to stderr
    let err = Error::from_raw_os_error(errnum);
    eprintln!("{}: {}", gettext("unable to record current working directory"), err);
    
    // Exit with failure status
    process::exit(EXIT_FAILURE);
}

/* Exit with an error about failure to restore the working directory
   during an openat emulation.  The caller must ensure that fd 2 is
   not a just-opened fd, even when openat_safer is not in use.  */

#[no_mangle]
pub extern "C" fn openat_restore_fail(errnum: i32) -> ! {
    let err = Error::from_raw_os_error(errnum);
    eprintln!("{}: {}", gettext("failed to return to initial working directory"), err);
    
    // Exit with failure status
    process::exit(EXIT_FAILURE);
}