/* An interface to read() that retries after interrupts.
   Copyright (C) 2002, 2006, 2009-2023 Free Software Foundation, Inc.

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

/// Read up to `count` bytes into `buf` from descriptor `fd`, retrying if interrupted.
/// Returns the actual number of bytes read, zero for EOF, or SAFE_READ_ERROR upon error.
pub fn safe_read(fd: &impl AsRawFd, buf: &mut [u8], count: usize) -> usize {
    let mut remaining = count.min(buf.len());
    let mut total_read = 0;

    while remaining > 0 {
        let current_buf = &mut buf[total_read..total_read + remaining];
        match read_attempt(fd.as_raw_fd(), current_buf) {
            Ok(0) => break, // EOF
            Ok(n) => {
                total_read += n;
                remaining -= n;
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => return SAFE_READ_ERROR,
        }
    }

    total_read
}

fn read_attempt(fd: i32, buf: &mut [u8]) -> io::Result<usize> {
    let mut file = unsafe { std::fs::File::from_raw_fd(fd) };
    let result = file.read(buf);
    std::mem::forget(file); // Prevent closing the file descriptor
    result
}

/* An interface to read and write that retries after interrupts.

   Copyright (C) 1993-1994, 1998, 2002-2006, 2009-2023 Free Software
   Foundation, Inc.

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

pub const SAFE_WRITE_ERROR: usize = usize::MAX;

/// Write up to `count` bytes from `buf` to descriptor `fd`, retrying if interrupted.
/// Returns the actual number of bytes written or SAFE_WRITE_ERROR upon error.
pub fn safe_write(fd: &impl AsRawFd, buf: &[u8], count: usize) -> usize {
    let mut remaining = count.min(buf.len());
    let mut total_written = 0;

    while remaining > 0 {
        let current_buf = &buf[total_written..total_written + remaining];
        match write_attempt(fd.as_raw_fd(), current_buf) {
            Ok(0) => break, // Shouldn't happen for writes
            Ok(n) => {
                total_written += n;
                remaining -= n;
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(_) => return SAFE_WRITE_ERROR,
        }
    }

    total_written
}

fn write_attempt(fd: i32, buf: &[u8]) -> io::Result<usize> {
    let mut file = unsafe { std::fs::File::from_raw_fd(fd) };
    let result = file.write(buf);
    std::mem::forget(file); // Prevent closing the file descriptor
    result
}