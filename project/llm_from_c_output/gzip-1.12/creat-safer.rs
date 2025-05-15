// Invoke creat, but avoid some glitches.

// Copyright (C) 2005-2006, 2009-2022 Free Software Foundation, Inc.

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

// Written by Jim Meyering.

use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::path::Path;

fn creat_safer(file: &Path, mode: u32) -> std::io::Result<File> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(mode)
        .open(file)?;
    
    fd_safer(file)
}

fn fd_safer(file: File) -> std::io::Result<File> {
    use nix::unistd::dup2;
    use std::os::unix::io::RawFd;
    
    let fd = file.as_raw_fd();
    if fd <= 2 {
        let new_fd = dup2(fd, 3)?;
        unsafe { Ok(File::from_raw_fd(new_fd)) }
    } else {
        Ok(file)
    }
}