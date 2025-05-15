/* Set file access and modification times.

   Copyright (C) 2009-2021 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by the
   Free Software Foundation; either version 3 of the License, or any
   later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Eric Blake.  */

/* derived from a function in utimens.c */

use std::fs::{File, OpenOptions};
use std::os::unix::fs::{OpenOptionsExt, utimesat, futimens};
use std::path::Path;
use std::time::SystemTime;
use std::os::unix::io::AsRawFd;
use std::io;

/// Set the access and modification timestamps of FD (a.k.a. FILE) to be
/// TIMESPEC[0] and TIMESPEC[1], respectively; relative to directory DIR.
/// FD must be either negative -- in which case it is ignored --
/// or a file descriptor that is open on FILE.
/// If FD is nonnegative, then FILE can be NULL, which means
/// use just futimes (or equivalent) instead of utimes (or equivalent),
/// and fail if on an old system without futimes (or equivalent).
/// If TIMESPEC is null, set the timestamps to the current time.
/// ATFLAG is passed to utimensat if FD is negative or futimens was
/// unsupported, which can allow operation on FILE as a symlink.
/// Return 0 on success, -1 (setting errno) on failure.
pub fn fdutimensat(
    fd: Option<&File>,
    dir: Option<&File>,
    file: Option<&Path>,
    ts: Option<[std::time::SystemTime; 2]>,
    atflag: i32,
) -> io::Result<()> {
    let ts = ts.unwrap_or_else(|| {
        let now = SystemTime::now();
        [now, now]
    });

    if let Some(fd) = fd {
        match futimens(fd, &ts) {
            Ok(_) => return Ok(()),
            Err(e) if e.raw_os_error() == Some(libc::ENOSYS) => (),
            Err(e) => return Err(e),
        }
    }

    if let Some(file) = file {
        let dir_fd = dir.map(|f| f.as_raw_fd()).unwrap_or(libc::AT_FDCWD);
        utimesat(dir_fd, file, &ts, atflag)?;
        Ok(())
    } else {
        Err(io::Error::from_raw_os_error(libc::EBADF))
    }
}