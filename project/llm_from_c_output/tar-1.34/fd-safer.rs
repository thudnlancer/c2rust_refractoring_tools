// Return a safer copy of a file descriptor.

// Copyright (C) 2005-2006, 2009-2021 Free Software Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Written by Paul Eggert.

use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::io::{self, Error};

// Return FD, unless FD would be a copy of standard input, output, or
// error; in that case, return a duplicate of FD, closing FD. On
// failure to duplicate, close FD, set errno, and return -1. Preserve
// errno if FD is negative, so that the caller can always inspect
// errno when the returned value is negative.
//
// This function is usefully wrapped around functions that return file
// descriptors, e.g., fd_safer (open ("file", O_RDONLY)).

pub fn fd_safer(fd: RawFd) -> Result<RawFd, Error> {
    if fd >= libc::STDIN_FILENO && fd <= libc::STDERR_FILENO {
        match unsafe { libc::dup(fd) } {
            -1 => {
                let e = io::Error::last_os_error();
                unsafe { libc::close(fd) };
                Err(e)
            }
            new_fd => {
                unsafe { libc::close(fd) };
                Ok(new_fd)
            }
        }
    } else {
        Ok(fd)
    }
}