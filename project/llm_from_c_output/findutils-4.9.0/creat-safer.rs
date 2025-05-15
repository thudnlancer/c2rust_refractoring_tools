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
use nix::unistd::dup2;
use nix::fcntl::{fcntl, FcntlArg, FdFlag};

fn fd_safer(fd: i32) -> i32 {
    if fd <= 2 {
        let new_fd = fcntl(fd, FcntlArg::F_DUPFD_CLOEXEC(3)).unwrap();
        unsafe {
            libc::close(fd);
        }
        new_fd
    } else {
        fd
    }
}

pub fn creat_safer(file: &str, mode: libc::mode_t) -> std::io::Result<File> {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .mode(mode)
        .open(file)?;
    
    let safer_fd = fd_safer(file.as_raw_fd());
    Ok(unsafe { File::from_raw_fd(safer_fd) })
}