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

use std::io::{self, IsTerminal};
use std::os::fd::AsRawFd;

#[cfg(any(target_os = "windows", target_os = "cygwin", target_os = "emscripten"))]
use std::os::windows::io::AsRawHandle;
#[cfg(any(target_os = "windows", target_os = "cygwin", target_os = "emscripten"))]
use winapi::um::ioapiset::GetFileType;
#[cfg(any(target_os = "windows", target_os = "cygwin", target_os = "emscripten"))]
use winapi::um::winbase::FILE_TYPE_CHAR;

/// Set the binary mode for a file descriptor.
///
/// On Unix-like systems, this is a no-op since binary mode is the default.
/// On Windows, this sets the mode to either binary or text.
///
/// # Arguments
/// * `fd` - A raw file descriptor
/// * `mode` - The mode to set (O_BINARY or O_TEXT on Windows)
///
/// # Returns
/// The previous mode on success, or an error on failure.
#[cfg(not(any(target_os = "windows", target_os = "cygwin", target_os = "emscripten")))]
pub fn set_binary_mode(_fd: i32, _mode: i32) -> io::Result<i32> {
    Ok(libc::O_BINARY)
}

#[cfg(any(target_os = "windows", target_os = "cygwin", target_os = "emscripten"))]
pub fn set_binary_mode(fd: i32, mode: i32) -> io::Result<i32> {
    unsafe {
        let handle = libc::get_osfhandle(fd);
        if handle == -1 {
            return Err(io::Error::last_os_error());
        }

        let file_type = GetFileType(handle as _);
        if file_type == FILE_TYPE_CHAR {
            // If FD refers to a console, silently ignore the request
            return Ok(libc::O_TEXT);
        }

        let old_mode = libc::_setmode(fd, mode);
        if old_mode == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(old_mode)
        }
    }
}

/// Convenience function to set a file descriptor to binary mode.
///
/// # Arguments
/// * `fd` - A raw file descriptor
///
/// # Returns
/// `Ok(())` on success, or an error on failure.
pub fn set_binary(fd: i32) -> io::Result<()> {
    set_binary_mode(fd, libc::O_BINARY).map(|_| ())
}

/// Check if a file descriptor refers to a terminal.
///
/// # Arguments
/// * `fd` - A raw file descriptor
///
/// # Returns
/// `true` if the descriptor refers to a terminal, `false` otherwise.
pub fn is_terminal(fd: i32) -> bool {
    unsafe {
        let file = std::fs::File::from_raw_fd(fd);
        let result = file.is_terminal();
        std::mem::forget(file); // Prevent closing the file descriptor
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_set_binary_mode() {
        let mut tempfile = NamedTempFile::new().unwrap();
        writeln!(tempfile, "test").unwrap();
        let fd = tempfile.as_raw_fd();

        let result = set_binary_mode(fd, libc::O_BINARY);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_binary() {
        let mut tempfile = NamedTempFile::new().unwrap();
        writeln!(tempfile, "test").unwrap();
        let fd = tempfile.as_raw_fd();

        assert!(set_binary(fd).is_ok());
    }

    #[test]
    fn test_is_terminal() {
        let stdin_fd = 0; // STDIN_FILENO
        assert_eq!(is_terminal(stdin_fd), unsafe {
            libc::isatty(stdin_fd) != 0
        });
    }
}