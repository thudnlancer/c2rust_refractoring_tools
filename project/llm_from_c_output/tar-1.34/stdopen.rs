//! Ensure that the three standard file descriptors are in use
//!
//! Copyright (C) 2005-2021 Free Software Foundation, Inc.
//!
//! This program is free software: you can redistribute it and/or modify
//! it under the terms of the GNU General Public License as published by
//! the Free Software Foundation; either version 3 of the License, or
//! (at your option) any later version.
//!
//! This program is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU General Public License for more details.
//!
//! You should have received a copy of the GNU General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};

const STDIN_FILENO: RawFd = 0;
const STDOUT_FILENO: RawFd = 1;
const STDERR_FILENO: RawFd = 2;

/// Try to ensure that all of the standard file numbers (0, 1, 2) are in use.
/// Without this, each application would have to guard every call to open, dup,
/// fopen, etc. with tests to ensure they don't use one of the special file numbers
/// when opening a file.
///
/// Returns Ok(()) if successful, or an io::Error if at least one of the file
/// descriptors is initially closed and could not be opened.
pub fn stdopen() -> io::Result<()> {
    for fd in STDIN_FILENO..=STDERR_FILENO {
        if let Err(e) = unsafe { libc::fcntl(fd, libc::F_GETFD) } {
            if e != libc::EBADF {
                return Err(io::Error::from_raw_os_error(e));
            }

            // Open /dev/null with the contrary mode so that the typical
            // read (stdin) or write (stdout, stderr) operation will fail.
            // With descriptor 0, we can do even better on systems that
            // have /dev/full, by opening that write-only instead of
            // /dev/null. The only drawback is that a write-provoked
            // failure comes with a misleading errno value, ENOSPC.
            let mode = if fd == STDIN_FILENO {
                libc::O_WRONLY
            } else {
                libc::O_RDONLY
            };

            let file = if fd == STDIN_FILENO {
                OpenOptions::new()
                    .write(true)
                    .open("/dev/full")
                    .or_else(|_| OpenOptions::new().write(true).open("/dev/null"))
            } else {
                OpenOptions::new().read(true).open("/dev/null")
            }?;

            let new_fd = file.as_raw_fd();
            if STDERR_FILENO < new_fd {
                // 0, 1, and 2 are already open somehow.
                // Our is not to reason why.
                return Ok(());
            }

            // If the new fd doesn't match the target fd, we need to dup/close
            if new_fd != fd {
                unsafe {
                    if libc::dup2(new_fd, fd) == -1 {
                        return Err(io::Error::last_os_error());
                    }
                    libc::close(new_fd);
                }
            }
        }
    }

    Ok(())
}