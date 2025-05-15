/* Open a directory relative to another directory.

   Copyright (C) 2018-2023 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published
   by the Free Software Foundation, either version 3 of the License,
   or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::fs::{DirBuilder, File, OpenOptions};
use std::os::unix::fs::{DirBuilderExt, OpenOptionsExt};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
use std::path::Path;
use std::io;
use libc::{O_RDONLY, O_CLOEXEC, O_DIRECTORY, O_NOCTTY, O_NONBLOCK};

/* Relative to DIR_FD, open the directory DIR, passing EXTRA_FLAGS to
   the underlying openat call. On success, store into *PNEW_FD the
   underlying file descriptor of the newly opened directory and return
   the directory stream. On failure, return None and set errno.

   On success, *PNEW_FD is at least 3, so this is a "safer" function. */

pub fn opendirat(
    dir_fd: i32,
    dir: &Path,
    extra_flags: i32,
    pnew_fd: &mut i32,
) -> io::Result<std::fs::Dir> {
    let open_flags = O_RDONLY | O_CLOEXEC | O_DIRECTORY | O_NOCTTY | O_NONBLOCK | extra_flags;

    let file = OpenOptions::new()
        .read(true)
        .custom_flags(open_flags)
        .open_at(&File::from_raw_fd(dir_fd), dir)?;

    let new_fd = file.as_raw_fd();
    match std::fs::Dir::from_fd(file) {
        Ok(dirp) => {
            *pnew_fd = new_fd;
            Ok(dirp)
        }
        Err(e) => {
            unsafe { libc::close(new_fd) };
            Err(e)
        }
    }
}

trait OpenOptionsExt {
    fn open_at(&self, dir: &File, path: &Path) -> io::Result<File>;
}

impl OpenOptionsExt for OpenOptions {
    fn open_at(&self, dir: &File, path: &Path) -> io::Result<File> {
        use std::os::unix::ffi::OsStrExt;
        let path_c = std::ffi::CString::new(path.as_os_str().as_bytes())?;
        let fd = unsafe { libc::openat(dir.as_raw_fd(), path_c.as_ptr(), self.custom_flags(), 0) };
        if fd < 0 {
            return Err(io::Error::last_os_error());
        }
        Ok(unsafe { File::from_raw_fd(fd) })
    }
}