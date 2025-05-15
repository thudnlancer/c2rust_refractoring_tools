// Binary mode I/O.
// Copyright (C) 2001, 2003, 2005, 2008-2022 Free Software Foundation, Inc.
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

use std::os::raw::c_int;
use std::io::{self, IsTerminal};

#[cfg(windows)]
use windows_sys::Win32::Storage::FileSystem::{O_BINARY, O_TEXT};

#[cfg(not(windows))]
const O_BINARY: c_int = 0;
#[cfg(not(windows))]
const O_TEXT: c_int = 0;

/// Set file descriptor's mode to either O_TEXT or O_BINARY.
/// Returns the old mode if successful, or an error on failure.
pub fn set_binary_mode(fd: c_int, mode: c_int) -> io::Result<c_int> {
    #[cfg(any(target_os = "emx", target_os = "diego", target_os = "cygwin"))]
    {
        if is_terminal(fd) {
            Ok(O_TEXT)
        } else {
            unsafe { set_mode(fd, mode) }
        }
    }
    
    #[cfg(not(any(target_os = "emx", target_os = "diego", target_os = "cygwin")))]
    {
        if is_terminal(fd) {
            Ok(O_TEXT)
        } else {
            Ok(O_BINARY)
        }
    }
}

/// Helper function to check if file descriptor is a terminal
fn is_terminal(fd: c_int) -> bool {
    use std::os::fd::FromRawFd;
    unsafe { std::fs::File::from_raw_fd(fd).is_terminal() }
}

/// Wrapper for the platform-specific setmode function
#[cfg(any(target_os = "emx", target_os = "diego", target_os = "cygwin"))]
unsafe fn set_mode(fd: c_int, mode: c_int) -> io::Result<c_int> {
    let result = libc::setmode(fd, mode);
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(result)
    }
}

/// Macro to set binary mode (obsolescent)
#[macro_export]
macro_rules! SET_BINARY {
    ($fd:expr) => {
        let _ = $crate::set_binary_mode($fd, O_BINARY);
    };
}