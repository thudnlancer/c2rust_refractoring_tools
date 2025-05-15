/* dirfd.rs -- return the file descriptor associated with an open DIR*

   Copyright (C) 2001, 2006, 2008-2023 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Jim Meyering. */

use std::collections::HashMap;
use std::os::unix::io::RawFd;
use std::sync::{Arc, Mutex};
use libc::{DIR, EINVAL, ENOTSUP};
use std::io;

#[cfg(GNULIB_defined_DIR)]
use dirent_private::DirExt;

lazy_static::lazy_static! {
    static ref DIRP_FD_MAP: Arc<Mutex<HashMap<*mut DIR, RawFd>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[cfg(__KLIBC__)]
pub fn register_dirp_fd(fd: RawFd, dirp: *mut DIR) -> io::Result<()> {
    let mut map = DIRP_FD_MAP.lock().unwrap();
    map.insert(dirp, fd);
    Ok(())
}

#[cfg(__KLIBC__)]
pub fn unregister_dirp_fd(fd: RawFd) -> io::Result<()> {
    let mut map = DIRP_FD_MAP.lock().unwrap();
    if let Some((&dirp, _)) = map.iter().find(|(_, &v)| v == fd) {
        map.remove(&dirp);
        unsafe { libc::close(fd) };
    }
    Ok(())
}

pub fn dirfd(dir_p: *mut DIR) -> io::Result<RawFd> {
    #[cfg(GNULIB_defined_DIR)]
    {
        let fd = unsafe { (*dir_p).fd_to_close };
        if fd == -1 {
            Err(io::Error::from_raw_os_error(EINVAL))
        } else {
            Ok(fd)
        }
    }

    #[cfg(not(GNULIB_defined_DIR))]
    {
        let fd = unsafe { DIR_TO_FD(dir_p) };
        if fd == -1 {
            #[cfg(not(__KLIBC__))]
            {
                Err(io::Error::from_raw_os_error(ENOTSUP))
            }
            #[cfg(__KLIBC__)]
            {
                let map = DIRP_FD_MAP.lock().unwrap();
                match map.get(&dir_p) {
                    Some(&fd) => Ok(fd),
                    None => Err(io::Error::from_raw_os_error(EINVAL)),
                }
            }
        } else {
            Ok(fd)
        }
    }
}