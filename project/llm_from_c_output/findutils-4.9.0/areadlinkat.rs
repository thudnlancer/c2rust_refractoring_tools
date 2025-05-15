/* areadlinkat.rs -- readlinkat wrapper to return allocated link name
   Unlike xreadlinkat, only panic on failure to change directory.

   Copyright (C) 2001, 2003-2007, 2009-2022 Free Software Foundation, Inc.

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

/* Written by Jim Meyering <jim@meyering.net>,
   and Bruno Haible <bruno@clisp.org>,
   and Eric Blake <ebb9@byu.net>.  */

use std::os::unix::fs::readlinkat;
use std::path::{Path, PathBuf};
use std::io;

/// Call readlinkat to get the symbolic link value of `filename` relative to `fd`.
/// Return a `PathBuf` containing that path.
/// If readlinkat fails, return the error (although failure to change directory will panic).
/// If the link value is too long, return an error with kind `io::ErrorKind::InvalidInput`.
pub fn areadlinkat(fd: i32, filename: &Path) -> io::Result<PathBuf> {
    readlinkat(Some(fd), filename)
}

#[cfg(not(HAVE_READLINKAT))]
/// Fallback implementation when readlinkat is not available
pub fn areadlinkat(fd: i32, filename: &Path) -> io::Result<PathBuf> {
    use std::env::set_current_dir;
    use std::fs::read_link;
    use std::os::unix::io::AsRawFd;
    use std::path::PathBuf;

    let original_dir = std::fs::File::open(".").map_err(|e| {
        eprintln!("Failed to open current directory: {}", e);
        std::process::exit(1);
    })?;
    
    let res = (|| {
        let dir = std::fs::File::open(format!("/proc/self/fd/{}", fd.as_raw_fd()))?;
        set_current_dir(dir)?;
        read_link(filename)
    })();

    // Restore original directory
    if let Err(e) = set_current_dir(original_dir) {
        eprintln!("Failed to restore directory: {}", e);
        std::process::exit(1);
    }

    res
}