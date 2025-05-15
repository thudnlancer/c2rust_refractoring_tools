// Invoke openat, but avoid some glitches.

// Copyright (C) 2005-2006, 2008-2021 Free Software Foundation, Inc.

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// Written by Paul Eggert for open, ported by Eric Blake for openat.

use std::fs::{File, OpenOptions};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::path::Path;

pub fn openat_safer(fd: RawFd, file: &Path, flags: i32, mode: Option<u32>) -> std::io::Result<File> {
    let mut options = OpenOptions::new();
    
    if flags & libc::O_CREAT != 0 {
        options.create(true);
        if let Some(m) = mode {
            options.mode(m);
        }
    }
    
    if flags & libc::O_WRONLY != 0 {
        options.write(true);
    } else if flags & libc::O_RDWR != 0 {
        options.read(true).write(true);
    } else {
        options.read(true);
    }
    
    if flags & libc::O_APPEND != 0 {
        options.append(true);
    }
    
    if flags & libc::O_TRUNC != 0 {
        options.truncate(true);
    }
    
    if flags & libc::O_EXCL != 0 {
        options.create_new(true);
    }
    
    let file = unsafe {
        let c_file = std::ffi::CString::new(file.as_os_str().to_str().unwrap())?;
        let raw_fd = libc::openat(fd, c_file.as_ptr(), flags, mode.unwrap_or(0));
        if raw_fd == -1 {
            return Err(std::io::Error::last_os_error());
        }
        File::from_raw_fd(raw_fd)
    };
    
    fd_safer(file)
}

fn fd_safer(file: File) -> std::io::Result<File> {
    let fd = file.into_raw_fd();
    if fd <= 2 {
        unsafe {
            let new_fd = libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 3);
            if new_fd == -1 {
                return Err(std::io::Error::last_os_error());
            }
            libc::close(fd);
            Ok(File::from_raw_fd(new_fd))
        }
    } else {
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}