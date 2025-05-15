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

/// Report a save-cwd failure and exit
pub fn openat_save_fail(errnum: Error) -> ! {
    eprintln!("unable to record current working directory: {}", errnum);
    process::abort();
}

/// Exit with an error about failure to restore the working directory
/// during an openat emulation. The caller must ensure that fd 2 is
/// not a just-opened fd, even when openat_safer is not in use.
pub fn openat_restore_fail(errnum: Error) -> ! {
    eprintln!("failed to return to initial working directory: {}", errnum);
    process::abort();
}