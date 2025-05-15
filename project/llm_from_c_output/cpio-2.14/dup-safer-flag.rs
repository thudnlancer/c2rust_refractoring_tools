// Duplicate a file descriptor result, avoiding clobbering
// STD{IN,OUT,ERR}_FILENO, with specific flags.
//
// Copyright (C) 2001, 2004-2006, 2009-2023 Free Software Foundation, Inc.
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

// Written by Paul Eggert and Eric Blake.

use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::os::unix::prelude::OwnedFd;
use std::fs::File;

const STDERR_FILENO: RawFd = 2;

/// Like dup, but do not return STDIN_FILENO, STDOUT_FILENO, or
/// STDERR_FILENO. If FLAG contains O_CLOEXEC, behave like
/// fcntl(F_DUPFD_CLOEXEC) rather than fcntl(F_DUPFD).
pub fn dup_safer_flag(fd: &File, flag: i32) -> std::io::Result<OwnedFd> {
    let min_fd = STDERR_FILENO + 1;
    
    if flag & libc::O_CLOEXEC != 0 {
        let new_fd = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_DUPFD_CLOEXEC, min_fd) };
        if new_fd == -1 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(unsafe { OwnedFd::from_raw_fd(new_fd) })
    } else {
        let new_fd = unsafe { libc::fcntl(fd.as_raw_fd(), libc::F_DUPFD, min_fd) };
        if new_fd == -1 {
            return Err(std::io::Error::last_os_error());
        }
        Ok(unsafe { OwnedFd::from_raw_fd(new_fd) })
    }
}