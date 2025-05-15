/* Save and restore current working directory.

   Copyright (C) 1995, 1997-1998, 2003, 2009-2021 Free Software Foundation,
   Inc.

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

/* Written by Jim Meyering.  */

use std::fs;
use std::os::unix::io::{AsRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::io;

#[derive(Debug)]
pub struct SavedCwd {
    desc: Option<fs::File>,
    name: Option<PathBuf>,
}

impl SavedCwd {
    /// Save the current working directory.
    /// Returns Ok(SavedCwd) on success, Err on failure.
    pub fn save() -> io::Result<Self> {
        let desc = match fs::File::open(".") {
            Ok(file) => Some(file),
            Err(_) => None,
        };

        if desc.is_some() {
            Ok(Self {
                desc,
                name: None,
            })
        } else {
            let current_dir = std::env::current_dir()?;
            Ok(Self {
                desc: None,
                name: Some(current_dir),
            })
        }
    }

    /// Restore to the saved working directory.
    /// Returns Ok(()) on success, Err on failure.
    pub fn restore(&self) -> io::Result<()> {
        if let Some(file) = &self.desc {
            let fd = file.as_raw_fd();
            unsafe { libc::fchdir(fd) };
            Ok(())
        } else if let Some(path) = &self.name {
            std::env::set_current_dir(path)
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No saved working directory",
            ))
        }
    }
}

impl Drop for SavedCwd {
    fn drop(&mut self) {
        if let Some(file) = self.desc.take() {
            drop(file); // Closing the file descriptor
        }
        self.name.take(); // Freeing the path
    }
}