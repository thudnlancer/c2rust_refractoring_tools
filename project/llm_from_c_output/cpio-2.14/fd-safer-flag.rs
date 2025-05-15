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

use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::os::unix::io::IntoRawFd;
use std::fs::File;
use std::io;

/// Return FD, unless FD would be a copy of standard input, output, or
/// error; in that case, return a duplicate of FD, closing FD. If FLAG
/// contains O_CLOEXEC, the returned FD will have close-on-exec
/// semantics. On failure to duplicate, close FD, set errno, and
/// return -1. Preserve errno if FD is negative, so that the caller
/// can always inspect errno when the returned value is negative.
///
/// This function is usefully wrapped around functions that return file
/// descriptors, e.g., fd_safer_flag (open ("file", O_RDONLY | flag), flag).
pub fn fd_safer_flag(fd: RawFd, flag: libc::c_int) -> io::Result<RawFd> {
    if fd >= libc::STDIN_FILENO && fd <= libc::STDERR_FILENO {
        let file = unsafe { File::from_raw_fd(fd) };
        let new_fd = dup_safer_flag(file, flag)?;
        Ok(new_fd)
    } else {
        Ok(fd)
    }
}

fn dup_safer_flag(file: File, flag: libc::c_int) -> io::Result<RawFd> {
    let new_file = file.try_clone()?;
    let new_fd = new_file.into_raw_fd();
    
    if flag & libc::O_CLOEXEC != 0 {
        unsafe {
            libc::fcntl(new_fd, libc::F_SETFD, libc::FD_CLOEXEC);
        }
    }
    
    Ok(new_fd)
}