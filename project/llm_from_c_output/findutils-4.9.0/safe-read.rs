/* An interface to read() that retries after interrupts.
   Copyright (C) 2002, 2006, 2009-2022 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;

pub const SAFE_READ_ERROR: usize = usize::MAX;

/// Read up to `count` bytes at `buf` from descriptor `fd`, retrying if interrupted.
/// Returns the actual number of bytes read, zero for EOF, or SAFE_READ_ERROR upon error.
pub fn safe_read(fd: &impl AsRawFd, buf: &mut [u8], count: usize) -> usize {
    let mut remaining = count.min(buf.len());
    let mut total_read = 0;
    let fd = fd.as_raw_fd();

    while remaining > 0 {
        let buf_slice = &mut buf[total_read..total_read + remaining];
        match unsafe { libc::read(fd, buf_slice.as_mut_ptr() as *mut libc::c_void, remaining) } {
            -1 => {
                let err = io::Error::last_os_error();
                if err.kind() == io::ErrorKind::Interrupted {
                    continue;
                } else if err.raw_os_error() == Some(libc::EINVAL) && remaining > SYS_BUFSIZE_MAX {
                    remaining = SYS_BUFSIZE_MAX;
                    continue;
                } else {
                    return SAFE_READ_ERROR;
                }
            }
            0 => break, // EOF
            n => {
                let bytes_read = n as usize;
                total_read += bytes_read;
                remaining -= bytes_read;
            }
        }
    }

    total_read
}

const SYS_BUFSIZE_MAX: usize = if cfg!(target_os = "linux") {
    0x7ffff000
} else {
    // Conservative default for other platforms
    8192
};

/// Write up to `count` bytes from `buf` to descriptor `fd`, retrying if interrupted.
/// Returns the actual number of bytes written or SAFE_WRITE_ERROR upon error.
pub fn safe_write(fd: &impl AsRawFd, buf: &[u8], count: usize) -> usize {
    let mut remaining = count.min(buf.len());
    let mut total_written = 0;
    let fd = fd.as_raw_fd();

    while remaining > 0 {
        let buf_slice = &buf[total_written..total_written + remaining];
        match unsafe { libc::write(fd, buf_slice.as_ptr() as *const libc::c_void, remaining) } {
            -1 => {
                let err = io::Error::last_os_error();
                if err.kind() == io::ErrorKind::Interrupted {
                    continue;
                } else if err.raw_os_error() == Some(libc::EINVAL) && remaining > SYS_BUFSIZE_MAX {
                    remaining = SYS_BUFSIZE_MAX;
                    continue;
                } else {
                    return SAFE_READ_ERROR;
                }
            }
            n => {
                let bytes_written = n as usize;
                total_written += bytes_written;
                remaining -= bytes_written;
            }
        }
    }

    total_written
}