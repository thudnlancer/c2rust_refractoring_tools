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

use std::os::unix::io::RawFd;
use std::sync::{Arc, Mutex};
use std::collections::LinkedList;
use std::io::{Error, ErrorKind};

#[cfg(target_os = "linux")]
use libc::{DIR, close};

struct DirpFd {
    dirp: *mut DIR,
    fd: RawFd,
}

lazy_static::lazy_static! {
    static ref DIRP_FD_LIST: Arc<Mutex<LinkedList<DirpFd>>> = Arc::new(Mutex::new(LinkedList::new()));
}

/// Register fd associated with dirp to dirp_fd_list.
fn register_dirp_fd(fd: RawFd, dirp: *mut DIR) -> Result<(), Error> {
    let mut list = DIRP_FD_LIST.lock().unwrap();
    list.push_front(DirpFd { dirp, fd });
    Ok(())
}

/// Unregister fd from dirp_fd_list with closing it
fn unregister_dirp_fd(fd: RawFd) {
    let mut list = DIRP_FD_LIST.lock().unwrap();
    let mut split_list = list.split_off(0);
    
    while let Some(dirp_fd) = split_list.pop_front() {
        if dirp_fd.fd == fd {
            unsafe { close(fd) };
        } else {
            list.push_back(dirp_fd);
        }
    }
}

pub fn dirfd(dir_p: *mut DIR) -> Result<RawFd, Error> {
    #[cfg(feature = "gnulib_defined_dir")]
    {
        // Assuming dir_p has a fd_to_close field in GNULIB case
        unsafe {
            let fd = (*dir_p).fd_to_close;
            if fd == -1 {
                return Err(Error::new(ErrorKind::InvalidInput, "Invalid directory stream"));
            }
            Ok(fd)
        }
    }
    
    #[cfg(not(feature = "gnulib_defined_dir"))]
    {
        #[cfg(not(target_os = "linux"))]
        {
            // Default POSIX implementation
            unsafe {
                let fd = libc::dirfd(dir_p);
                if fd == -1 {
                    return Err(Error::last_os_error());
                }
                Ok(fd)
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            // KLIBC-specific implementation
            let list = DIRP_FD_LIST.lock().unwrap();
            for dirp_fd in list.iter() {
                if dirp_fd.dirp == dir_p {
                    return Ok(dirp_fd.fd);
                }
            }
            Err(Error::new(ErrorKind::InvalidInput, "Directory stream not found"))
        }
    }
}