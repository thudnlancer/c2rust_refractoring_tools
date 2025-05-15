// Invoke pipe2, but avoid some glitches.
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

// Written by Eric Blake.

use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::io::{Result, Error, ErrorKind};

/// Like pipe2, but ensure that neither of the file descriptors is
/// STDIN_FILENO, STDOUT_FILENO, or STDERR_FILENO.
pub fn pipe2_safer(flags: libc::c_int) -> Result<(RawFd, RawFd)> {
    let mut fds = [0 as RawFd; 2];
    
    // Create the pipe
    if unsafe { libc::pipe2(fds.as_mut_ptr(), flags) } == 0 {
        for i in 0..2 {
            match fd_safer_flag(fds[i], flags) {
                Ok(new_fd) => fds[i] = new_fd,
                Err(e) => {
                    unsafe { libc::close(fds[1 - i]) };
                    return Err(e);
                }
            }
        }
        Ok((fds[0], fds[1]))
    } else {
        Err(Error::last_os_error())
    }
}

fn fd_safer_flag(fd: RawFd, flags: libc::c_int) -> Result<RawFd> {
    if fd == libc::STDIN_FILENO || fd == libc::STDOUT_FILENO || fd == libc::STDERR_FILENO {
        let new_fd = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 3) };
        if new_fd == -1 {
            return Err(Error::last_os_error());
        }
        unsafe { libc::close(fd) };
        Ok(new_fd)
    } else {
        Ok(fd)
    }
}