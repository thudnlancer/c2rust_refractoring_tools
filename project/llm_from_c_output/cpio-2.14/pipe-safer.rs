// Invoke pipe, but avoid some glitches.
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

// Written by Jim Meyering.

use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::io::{Result, Error, ErrorKind};

/// Like pipe, but ensure that neither of the file descriptors is
/// STDIN_FILENO, STDOUT_FILENO, or STDERR_FILENO. Fail with ENOSYS on
/// platforms that lack pipe.
pub fn pipe_safer() -> Result<(std::fs::File, std::fs::File)> {
    let (read, write) = pipe()?;
    
    let read = fd_safer(read)?;
    let write = match fd_safer(write) {
        Ok(file) => file,
        Err(e) => {
            let _ = std::fs::File::from_raw_fd(read.as_raw_fd());
            return Err(e);
        }
    };
    
    Ok((read, write))
}

fn pipe() -> Result<(RawFd, RawFd)> {
    let mut fds = [0; 2];
    if unsafe { libc::pipe(fds.as_mut_ptr()) } == -1 {
        return Err(Error::last_os_error());
    }
    Ok((fds[0], fds[1]))
}

fn fd_safer(fd: RawFd) -> Result<std::fs::File> {
    let file = unsafe { std::fs::File::from_raw_fd(fd) };
    
    match fd {
        0 | 1 | 2 => {
            let new_fd = unsafe { libc::dup(fd) };
            if new_fd == -1 {
                return Err(Error::last_os_error());
            }
            Ok(unsafe { std::fs::File::from_raw_fd(new_fd) })
        }
        _ => Ok(file),
    }
}