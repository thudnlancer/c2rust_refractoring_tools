/* Save and restore current working directory.

   Copyright (C) 1995, 1997-1998, 2003, 2009-2023 Free Software Foundation,
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
use std::os::unix::fs::DirBuilderExt;
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd, RawFd};
use std::path::{Path, PathBuf};
use std::io;

#[derive(Debug)]
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
                self.name = Some(std::env::current_dir()?);
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
                let file = fs::File::from_raw_fd(fd);
                let res = nix::unistd::fchdir(fd);
                let _ = file.into_raw_fd(); // Prevent closing the file
                res.map_err(|e| io::Error::from_raw_os_error(e as i32))?;
            }
            Ok(())
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
        if let Some(file) = self.desc.take() {
            let _ = file.into_raw_fd(); // File will be closed when dropped
        }
        self.name = None;
    }
}