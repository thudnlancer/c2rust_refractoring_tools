/* Report a save- or restore-cwd failure in our openat replacement and then exit.

   Copyright (C) 2005-2006, 2008-2021 Free Software Foundation, Inc.

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

use std::process;
use std::io::{Error, stderr};
use std::fmt;

const EXIT_FAILURE: i32 = 1;

#[derive(Debug)]
enum OpenatError {
    SaveFailure(Error),
    RestoreFailure(Error),
}

impl fmt::Display for OpenatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpenatError::SaveFailure(e) => write!(f, "unable to record current working directory: {}", e),
            OpenatError::RestoreFailure(e) => write!(f, "failed to return to initial working directory: {}", e),
        }
    }
}

impl std::error::Error for OpenatError {}

#[cold]
#[inline(never)]
pub fn openat_save_fail(errnum: Error) -> ! {
    eprintln!("{}", OpenatError::SaveFailure(errnum));
    process::abort();
}

#[cold]
#[inline(never)]
pub fn openat_restore_fail(errnum: Error) -> ! {
    eprintln!("{}", OpenatError::RestoreFailure(errnum));
    process::abort();
}