// Invoke open, but avoid some glitches.

// Copyright (C) 2005-2006, 2008-2023 Free Software Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Written by Paul Eggert.

use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::FromRawFd;
use libc::{c_int, mode_t};

pub fn open_safer(file: &str, flags: c_int, mode: Option<mode_t>) -> Result<File, std::io::Error> {
    let mut options = OpenOptions::new();
    
    options.read((flags & libc::O_RDONLY) != 0)
           .write((flags & libc::O_WRONLY) != 0 || (flags & libc::O_RDWR) != 0)
           .create((flags & libc::O_CREAT) != 0)
           .append((flags & libc::O_APPEND) != 0)
           .truncate((flags & libc::O_TRUNC) != 0)
           .create_new((flags & libc::O_EXCL) != 0);
    
    if let Some(m) = mode {
        options.mode(m);
    }
    
    let file = options.open(file)?;
    Ok(file)
}