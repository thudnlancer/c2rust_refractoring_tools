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
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{AsRawFd, RawFd};
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

        let file = fs::OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_CLOEXEC)
            .open(".")?;

        self.desc = Some(file);
        Ok(())
    }

    /// Restore the saved working directory.
    /// Returns Ok(()) on success, Err(io::Error) on failure.
    pub fn restore(&self) -> io::Result<()> {
        if let Some(ref file) = self.desc {
            let fd = file.as_raw_fd();
            unsafe {
                if libc::fchdir(fd) != 0 {
                    return Err(io::Error::last_os_error());
                }
            }
            Ok(())
        } else if let Some(ref path) = self.name {
            fs::set_current_dir(path)
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
        // The File and PathBuf will be automatically dropped
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn test_save_restore_cwd() {
        let original_dir = env::current_dir().unwrap();
        let mut cwd = SavedCwd::new();

        assert!(cwd.save().is_ok());

        let temp_dir = tempfile::tempdir().unwrap();
        assert!(env::set_current_dir(&temp_dir).is_ok());

        assert!(cwd.restore().is_ok());
        assert_eq!(env::current_dir().unwrap(), original_dir);
    }
}