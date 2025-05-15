/* Invoke dup, but avoid some glitches.

   Copyright (C) 2001, 2004-2006, 2009-2022 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Paul Eggert.  */

use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::fs::File;
use std::io;

const STDIN_FILENO: RawFd = 0;
const STDOUT_FILENO: RawFd = 1;
const STDERR_FILENO: RawFd = 2;

/* Like dup, but do not return STDIN_FILENO, STDOUT_FILENO, or
   STDERR_FILENO.  */

pub fn dup_safer(fd: RawFd) -> io::Result<File> {
    let min_fd = STDERR_FILENO + 1;
    unsafe {
        let new_fd = libc::fcntl(fd, libc::F_DUPFD, min_fd);
        if new_fd == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(File::from_raw_fd(new_fd))
        }
    }
}