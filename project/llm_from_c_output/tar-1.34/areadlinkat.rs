/* areadlinkat.rs -- readlinkat wrapper to return allocated link name
   Unlike xreadlinkat, only panic on failure to change directory.

   Copyright (C) 2001, 2003-2007, 2009-2021 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Jim Meyering <jim@meyering.net>,
   and Bruno Haible <bruno@clisp.org>,
   and Eric Blake <ebb9@byu.net>.  */

use std::os::unix::prelude::*;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[cfg(target_os = "linux")]
pub fn areadlinkat(fd: i32, filename: &Path) -> io::Result<PathBuf> {
    let mut buf = Vec::with_capacity(1024);
    
    loop {
        match fs::readlinkat(fd, filename, &mut buf) {
            Ok(_) => return Ok(PathBuf::from(buf)),
            Err(e) if e.raw_os_error() == Some(libc::ERANGE) => {
                let new_capacity = buf.capacity().checked_mul(2)
                    .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "link value too long"))?;
                buf.reserve(new_capacity);
            }
            Err(e) => return Err(e),
        }
    }
}

#[cfg(not(target_os = "linux"))]
pub fn areadlinkat(fd: i32, filename: &Path) -> io::Result<PathBuf> {
    // On non-Linux systems, change directory once and use regular readlink
    let original_dir = std::env::current_dir()?;
    
    // Change directory to the given fd
    let dir = unsafe { std::fs::File::from_raw_fd(fd) };
    std::env::set_current_dir(dir.path())?;
    
    let result = fs::read_link(filename);
    
    // Restore original directory
    std::env::set_current_dir(original_dir)?;
    
    result
}