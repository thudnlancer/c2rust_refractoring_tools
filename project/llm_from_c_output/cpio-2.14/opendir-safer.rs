/* Invoke opendir, but avoid some glitches.

   Copyright (C) 2009-2023 Free Software Foundation, Inc.

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

use std::fs::{File, OpenOptions};
use std::os::unix::io::{AsRawFd, FromRawFd, IntoRawFd};
use std::path::Path;
use std::io;
use libc::{STDERR_FILENO, dirfd, opendir, closedir, fdopendir, fcntl, F_DUPFD_CLOEXEC};
use nix::fcntl;

pub fn opendir_safer<P: AsRef<Path>>(name: P) -> io::Result<*mut libc::DIR> {
    unsafe {
        let dp = opendir(name.as_ref().as_ptr() as *const _);
        
        if !dp.is_null() {
            let fd = dirfd(dp);
            
            if fd >= 0 && fd <= STDERR_FILENO {
                let newdp;
                let e;
                
                #[cfg(any(HAVE_FDOPENDIR, GNULIB_FDOPENDIR))]
                {
                    let f = fcntl(fd, F_DUPFD_CLOEXEC, STDERR_FILENO + 1);
                    if f < 0 {
                        e = io::Error::last_os_error();
                        newdp = std::ptr::null_mut();
                    } else {
                        newdp = fdopendir(f);
                        e = io::Error::last_os_error();
                        if newdp.is_null() {
                            libc::close(f);
                        }
                    }
                }
                
                #[cfg(not(any(HAVE_FDOPENDIR, GNULIB_FDOPENDIR)))]
                {
                    newdp = opendir_safer(name)?;
                    e = io::Error::last_os_error();
                }
                
                closedir(dp);
                if newdp.is_null() {
                    return Err(e);
                }
                return Ok(newdp);
            }
        }
        
        if dp.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(dp)
        }
    }
}