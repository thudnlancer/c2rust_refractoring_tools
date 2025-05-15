// Adjust a file descriptor result so that it avoids clobbering
// STD{IN,OUT,ERR}_FILENO, with specific flags.
//
// Copyright (C) 2005-2006, 2009-2023 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Written by Paul Eggert and Eric Blake.

use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::io::{self, Error};

/// Return FD, unless FD would be a copy of standard input, output, or
/// error; in that case, return a duplicate of FD, closing FD. If FLAG
/// contains O_CLOEXEC, the returned FD will have close-on-exec
/// semantics. On failure to duplicate, close FD, set errno, and
/// return -1. Preserve errno if FD is negative, so that the caller
/// can always inspect errno when the returned value is negative.
///
/// This function is usefully wrapped around functions that return file
/// descriptors, e.g., fd_safer_flag (open ("file", O_RDONLY | flag), flag).
pub fn fd_safer_flag(fd: RawFd, flag: libc::c_int) -> Result<RawFd, Error> {
    if fd < 0 {
        return Err(Error::last_os_error());
    }

    if fd == libc::STDIN_FILENO || fd == libc::STDOUT_FILENO || fd == libc::STDERR_FILENO {
        match dup_safer_flag(fd, flag) {
            Ok(new_fd) => {
                unsafe { libc::close(fd) };
                Ok(new_fd)
            }
            Err(e) => {
                unsafe { libc::close(fd) };
                Err(e)
            }
        }
    } else {
        Ok(fd)
    }
}

fn dup_safer_flag(fd: RawFd, flag: libc::c_int) -> Result<RawFd, Error> {
    let new_fd = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0) };
    if new_fd == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(new_fd)
    }
}