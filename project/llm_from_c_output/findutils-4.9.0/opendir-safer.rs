/* Invoke opendir, but avoid some glitches.

   Copyright (C) 2009-2022 Free Software Foundation, Inc.

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

/* Written by Eric Blake.  */

use std::fs::{DirBuilder, File, OpenOptions};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::path::Path;
use std::io;
use libc::{STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO};
use nix::fcntl::{fcntl, FcntlArg, FdFlag};
use nix::dir::{Dir, DirStream};
use nix::errno::Errno;

/// Like opendir, but do not clobber stdin, stdout, or stderr.
pub fn opendir_safer<P: AsRef<Path>>(name: P) -> io::Result<Dir> {
    let dp = Dir::open(name.as_ref())?;
    let fd = dp.as_raw_fd();

    if fd >= STDIN_FILENO && fd <= STDERR_FILENO {
        // If fdopendir is native (as on Linux), then it is safe to
        // assume dirfd(fdopendir(n))==n.  If we are using the
        // gnulib module fdopendir, then this guarantee is not met,
        // but fdopendir recursively calls opendir_safer up to 3
        // times to at least get a safe fd.  If fdopendir is not
        // present but dirfd is accurate (as on cygwin 1.5.x), then
        // we recurse up to 3 times ourselves.  Finally, if dirfd
        // always fails (as on mingw), then we are already safe.
        
        let new_fd = fcntl(fd, FcntlArg::F_DUPFD_CLOEXEC(STDERR_FILENO + 1))
            .map_err(|e| io::Error::from_raw_os_error(e as i32))?;
        
        let new_dp = match Dir::from_fd(new_fd) {
            Ok(dir) => dir,
            Err(e) => {
                let _ = nix::unistd::close(new_fd);
                return Err(io::Error::from_raw_os_error(e as i32));
            }
        };
        
        Ok(new_dp)
    } else {
        Ok(dp)
    }
}