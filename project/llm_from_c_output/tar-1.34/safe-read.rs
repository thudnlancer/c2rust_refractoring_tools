// An interface to read() that retries after interrupts.
// Copyright (C) 2002, 2006, 2009-2021 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io::{self, Read, Write};
use std::os::unix::io::AsRawFd;

pub const SAFE_READ_ERROR: usize = usize::MAX;

/// Read up to `count` bytes at `buf` from descriptor `fd`, retrying if interrupted.
/// Return the actual number of bytes read, zero for EOF, or SAFE_READ_ERROR upon error.
pub fn safe_read(fd: &impl AsRawFd, buf: &mut [u8], count: usize) -> Result<usize, io::Error> {
    let mut buf = &mut buf[..count];
    let fd = fd.as_raw_fd();

    loop {
        let result = unsafe {
            let res = libc::read(fd, buf.as_mut_ptr() as *mut libc::c_void, buf.len());
            if res >= 0 {
                Ok(res as usize)
            } else {
                Err(io::Error::last_os_error())
            }
        };

        match result {
            Ok(n) => return Ok(n),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
}

/// Write up to `count` bytes from `buf` to descriptor `fd`, retrying if interrupted.
/// Return the actual number of bytes written or SAFE_WRITE_ERROR upon error.
pub fn safe_write(fd: &impl AsRawFd, buf: &[u8], count: usize) -> Result<usize, io::Error> {
    let buf = &buf[..count];
    let fd = fd.as_raw_fd();

    loop {
        let result = unsafe {
            let res = libc::write(fd, buf.as_ptr() as *const libc::c_void, buf.len());
            if res >= 0 {
                Ok(res as usize)
            } else {
                Err(io::Error::last_os_error())
            }
        };

        match result {
            Ok(n) => return Ok(n),
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) if e.raw_os_error() == Some(libc::EINVAL) && buf.len() > SYS_BUFSIZE_MAX => {
                // Handle case where buffer size exceeds system limit
                let new_count = SYS_BUFSIZE_MAX;
                return safe_write(fd, &buf[..new_count], new_count);
            }
            Err(e) => return Err(e),
        }
    }
}

// System-specific buffer size limit
const SYS_BUFSIZE_MAX: usize = if cfg!(target_os = "linux") {
    0x7ffff000 // Linux default pipe buffer size
} else {
    8192 // Conservative default for other systems
};