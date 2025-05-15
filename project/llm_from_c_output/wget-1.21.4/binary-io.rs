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

use std::os::fd::{AsFd, BorrowedFd};
use std::io::{self, IsTerminal};

#[cfg(windows)]
use windows_sys::Win32::Storage::FileSystem::{
    FILE_TYPE_CHAR, FILE_TYPE_DISK, FILE_TYPE_PIPE, FILE_TYPE_REMOTE, FILE_TYPE_UNKNOWN,
    GetFileType,
};

/// Sets the binary mode for a file descriptor.
///
/// On Windows, this uses `_setmode`. On Unix-like systems, this is a no-op
/// since binary mode is the default.
pub fn set_binary_mode<Fd: AsFd>(fd: Fd, binary: bool) -> io::Result<()> {
    #[cfg(windows)]
    {
        use std::os::windows::io::AsRawHandle;
        use windows_sys::Win32::Storage::FileSystem::SetHandleInformation;
        use windows_sys::Win32::System::Console::{
            ENABLE_LINE_INPUT, ENABLE_PROCESSED_INPUT, ENABLE_VIRTUAL_TERMINAL_INPUT,
            ENABLE_VIRTUAL_TERMINAL_PROCESSING,
        };

        let handle = fd.as_fd().as_raw_handle();
        let mode = if binary {
            0
        } else {
            ENABLE_LINE_INPUT | ENABLE_PROCESSED_INPUT | ENABLE_VIRTUAL_TERMINAL_INPUT | ENABLE_VIRTUAL_TERMINAL_PROCESSING
        };

        if unsafe { SetHandleInformation(handle as _, mode, mode) } == 0 {
            return Err(io::Error::last_os_error());
        }
    }

    #[cfg(not(windows))]
    {
        // On Unix, binary mode is the default and only mode
        let _ = (fd, binary);
    }

    Ok(())
}

/// Checks if the file descriptor refers to a terminal device.
pub fn is_terminal<Fd: AsFd>(fd: Fd) -> bool {
    fd.as_fd().is_terminal()
}

/// Sets the file descriptor to binary mode if it's not a terminal.
pub fn set_binary_mode_if_not_tty<Fd: AsFd>(fd: Fd, binary: bool) -> io::Result<()> {
    if is_terminal(&fd) {
        Ok(())
    } else {
        set_binary_mode(fd, binary)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempfile;

    #[test]
    fn test_set_binary_mode() {
        let mut file = tempfile().unwrap();
        file.write_all(b"test").unwrap();
        assert!(set_binary_mode(&file, true).is_ok());
    }

    #[test]
    fn test_is_terminal() {
        assert!(!is_terminal(&std::io::stdout()));
    }
}