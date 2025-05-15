// Binary mode I/O.
// Copyright (C) 2001, 2003, 2005, 2008-2023 Free Software Foundation, Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![allow(unused)]

use std::fs::File;
use std::io;
use std::os::unix::io::AsRawFd;
use std::os::unix::prelude::FromRawFd;

#[cfg(any(target_os = "windows", target_os = "cygwin"))]
use libc::{O_BINARY, O_TEXT};

#[cfg(not(any(target_os = "windows", target_os = "cygwin")))]
const O_BINARY: i32 = 0;
#[cfg(not(any(target_os = "windows", target_os = "cygwin")))]
const O_TEXT: i32 = 0;

/// Set file descriptor's mode to either O_TEXT or O_BINARY.
/// Returns the old mode if successful, or an error on failure.
pub fn set_binary_mode(fd: i32, mode: i32) -> io::Result<i32> {
    #[cfg(any(target_os = "emx", target_os = "djgpp", target_os = "cygwin"))]
    {
        if is_terminal(fd)? {
            // If FD refers to a console, O_TEXT is the only reasonable mode
            return Ok(O_TEXT);
        }
        unsafe { Ok(libc::setmode(fd, mode)) }
    }

    #[cfg(not(any(target_os = "emx", target_os = "djgpp", target_os = "cygwin")))]
    {
        Ok(O_BINARY)
    }
}

/// Convenience macro to set binary mode
#[macro_export]
macro_rules! SET_BINARY {
    ($fd:expr) => {
        let _ = $crate::set_binary_mode($fd, O_BINARY);
    };
}

fn is_terminal(fd: i32) -> io::Result<bool> {
    unsafe {
        let file = File::from_raw_fd(fd);
        let is_tty = atty::is(atty::Stream::from_fd(fd)?;
        std::mem::forget(file); // Prevent closing the file descriptor
        Ok(is_tty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_binary_mode() {
        let file = File::open("/dev/null").unwrap();
        let fd = file.as_raw_fd();
        assert!(set_binary_mode(fd, O_BINARY).is_ok());
    }
}