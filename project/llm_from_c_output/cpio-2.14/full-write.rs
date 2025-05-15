// An interface to write() that writes all it is asked to write.

// Copyright (C) 2002-2003, 2009-2023 Free Software Foundation, Inc.

// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.

// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::io::{self, Write, Read};
use std::os::unix::io::AsRawFd;

/// Write COUNT bytes at BUF to descriptor FD, retrying if interrupted
/// or if partial writes occur. Return the number of bytes successfully
/// written, setting errno if that is less than COUNT.
pub fn full_write(fd: i32, buf: &[u8]) -> io::Result<usize> {
    let mut total = 0;
    let mut ptr = buf;
    
    while !ptr.is_empty() {
        match unsafe { libc::write(fd, ptr.as_ptr() as *const _, ptr.len()) } {
            -1 => {
                let err = io::Error::last_os_error();
                if err.kind() != io::ErrorKind::Interrupted {
                    return Err(err);
                }
            }
            0 => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Zero byte transfer",
                ));
            }
            n => {
                let n = n as usize;
                total += n;
                ptr = &ptr[n..];
            }
        }
    }
    
    Ok(total)
}

/// Read COUNT bytes from descriptor FD into BUF, retrying if interrupted
/// or if partial reads occur. Return the number of bytes successfully
/// read, setting errno to 0 if EOF is encountered.
pub fn full_read(fd: i32, buf: &mut [u8]) -> io::Result<usize> {
    let mut total = 0;
    let mut ptr = buf;
    
    while !ptr.is_empty() {
        match unsafe { libc::read(fd, ptr.as_mut_ptr() as *mut _, ptr.len()) } {
            -1 => {
                let err = io::Error::last_os_error();
                if err.kind() != io::ErrorKind::Interrupted {
                    return Err(err);
                }
            }
            0 => {
                // EOF
                return Ok(total);
            }
            n => {
                let n = n as usize;
                total += n;
                ptr = &mut ptr[n..];
            }
        }
    }
    
    Ok(total)
}

// Safe wrapper using Rust's stdlib
pub fn full_write_std<W: Write>(mut writer: W, buf: &[u8]) -> io::Result<usize> {
    let mut total = 0;
    let mut ptr = buf;
    
    while !ptr.is_empty() {
        match writer.write(ptr) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "Zero byte transfer",
                ));
            }
            Ok(n) => {
                total += n;
                ptr = &ptr[n..];
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    
    Ok(total)
}

pub fn full_read_std<R: Read>(mut reader: R, buf: &mut [u8]) -> io::Result<usize> {
    let mut total = 0;
    let mut ptr = buf;
    
    while !ptr.is_empty() {
        match reader.read(ptr) {
            Ok(0) => return Ok(total), // EOF
            Ok(n) => {
                total += n;
                ptr = &mut ptr[n..];
            }
            Err(e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        }
    }
    
    Ok(total)
}