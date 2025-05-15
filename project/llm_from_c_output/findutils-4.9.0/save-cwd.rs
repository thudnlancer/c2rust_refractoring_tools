/* Save and restore current working directory.

   Copyright (C) 1995, 1997-1998, 2003, 2009-2022 Free Software Foundation,
   Inc.

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

/* Written by Jim Meyering.  */

use std::fs;
use std::os::unix::prelude::*;
use std::path::{Path, PathBuf};
use std::io;

pub struct SavedCwd {
    desc: Option<fs::File>,
    name: Option<PathBuf>,
}

impl SavedCwd {
    pub fn new() -> Self {
        SavedCwd {
            desc: None,
            name: None,
        }
    }

    /// Save the current working directory.
    /// Returns Ok(()) on success, Err(io::Error) on failure.
    pub fn save(&mut self) -> io::Result<()> {
        self.name = None;

        match fs::File::open(".") {
            Ok(file) => {
                self.desc = Some(file);
                Ok(())
            }
            Err(_) => {
                let current_dir = std::env::current_dir()?;
                self.name = Some(current_dir);
                Ok(())
            }
        }
    }

    /// Restore the saved working directory.
    /// Returns Ok(()) on success, Err(io::Error) on failure.
    pub fn restore(&self) -> io::Result<()> {
        if let Some(ref file) = self.desc {
            let fd = file.as_raw_fd();
            unsafe {
                if libc::fchdir(fd) == 0 {
                    Ok(())
                } else {
                    Err(io::Error::last_os_error())
                }
            }
        } else if let Some(ref path) = self.name {
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
        // The File descriptor will be automatically closed when dropped
        // The PathBuf will be automatically deallocated when dropped
    }
}