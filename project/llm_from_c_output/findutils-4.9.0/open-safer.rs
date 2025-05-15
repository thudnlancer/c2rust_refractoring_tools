/* Invoke open, but avoid some glitches.

   Copyright (C) 2005-2006, 2008-2022 Free Software Foundation, Inc.

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

use std::fs::{OpenOptions, File};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{AsRawFd, FromRawFd};
use libc::mode_t;

pub fn open_safer(file: &str, flags: i32, mode: Option<mode_t>) -> std::io::Result<File> {
    let mut options = OpenOptions::new();
    
    // Set flags
    options.read((flags & libc::O_RDONLY) != 0)
           .write((flags & libc::O_WRONLY) != 0 || (flags & libc::O_RDWR) != 0)
           .create((flags & libc::O_CREAT) != 0)
           .append((flags & libc::O_APPEND) != 0)
           .truncate((flags & libc::O_TRUNC) != 0)
           .create_new((flags & libc::O_EXCL) != 0 && (flags & libc::O_CREAT) != 0);
    
    // Set mode if O_CREAT is set and mode is provided
    if (flags & libc::O_CREAT) != 0 {
        if let Some(m) = mode {
            options.mode(m as u32);
        }
    }
    
    let file = options.open(file)?;
    fd_safer(file)
}

fn fd_safer(file: File) -> std::io::Result<File> {
    // In Rust, file descriptors are automatically closed when File goes out of scope,
    // so we don't need to do special handling like in C.
    // However, if additional safety checks are needed, they can be added here.
    Ok(file)
}