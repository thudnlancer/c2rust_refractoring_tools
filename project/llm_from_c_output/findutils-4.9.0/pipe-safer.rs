// Invoke pipe, but avoid some glitches.
// Copyright (C) 2005-2006, 2009-2022 Free Software Foundation, Inc.
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
use std::io::{self, Result};

/// Like pipe, but ensure that neither of the file descriptors is
/// STDIN_FILENO, STDOUT_FILENO, or STDERR_FILENO. Returns ENOSYS on
/// platforms that lack pipe.
pub fn pipe_safer() -> Result<(std::fs::File, std::fs::File)> {
    let (read, write) = os_pipe::pipe()?;
    
    let read_fd = read.as_raw_fd();
    let write_fd = write.as_raw_fd();
    
    let safer_read = fd_safer(read_fd).map_err(|e| {
        let _ = std::fs::File::from_raw_fd(write_fd);
        e
    })?;
    
    let safer_write = fd_safer(write_fd).map_err(|e| {
        let _ = std::fs::File::from_raw_fd(safer_read.as_raw_fd());
        e
    })?;
    
    Ok((safer_read, safer_write))
}

fn fd_safer(fd: RawFd) -> Result<std::fs::File> {
    use libc::{STDIN_FILENO, STDOUT_FILENO, STDERR_FILENO};
    
    match fd {
        STDIN_FILENO | STDOUT_FILENO | STDERR_FILENO => {
            let new_fd = unsafe { libc::dup(fd) };
            if new_fd < 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(unsafe { std::fs::File::from_raw_fd(new_fd) })
        }
        _ => Ok(unsafe { std::fs::File::from_raw_fd(fd) }),
    }
}