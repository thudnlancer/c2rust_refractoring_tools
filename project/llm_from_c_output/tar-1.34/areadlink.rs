/* Read symbolic links without size limitation.

   Copyright (C) 2001, 2003-2004, 2007, 2009-2021 Free Software Foundation,
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

/* Written by Jim Meyering <jim@meyering.net>  */

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn areadlink(filename: &Path) -> io::Result<PathBuf> {
    fs::read_link(filename)
}

#[cfg(feature = "areadlinkat")]
pub fn areadlinkat(fd: i32, filename: &Path) -> io::Result<PathBuf> {
    use std::os::unix::fs::FileTypeExt;
    
    let file = unsafe { std::fs::File::from_raw_fd(fd) };
    let metadata = file.metadata()?;
    if !metadata.file_type().is_symlink() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "not a symlink"));
    }
    fs::read_link(filename)
}

#[cfg(feature = "areadlinkat_with_size")]
pub fn areadlinkat_with_size(fd: i32, filename: &Path, _size_hint: usize) -> io::Result<PathBuf> {
    areadlinkat(fd, filename)
}

/* areadlink.c -- readlink wrapper to return the link name in malloc'd storage
   Unlike xreadlink and xreadlink_with_size, don't ever call exit.

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

/* Written by Jim Meyering <jim@meyering.net>
   and Bruno Haible <bruno@clisp.org>.  */

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_areadlink() {
        let dir = tempdir().unwrap();
        let link_path = dir.path().join("link");
        let target_path = dir.path().join("target");
        fs::write(&target_path, "").unwrap();
        std::os::unix::fs::symlink(&target_path, &link_path).unwrap();
        
        let result = areadlink(&link_path).unwrap();
        assert_eq!(result, target_path);
    }
}