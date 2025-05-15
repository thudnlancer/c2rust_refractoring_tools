/* ioctl.rs --- wrappers for Windows ioctl function

   Copyright (C) 2008-2022 Free Software Foundation, Inc.

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

/* Written by Paolo Bonzini */

use std::os::windows::io::AsRawHandle;
use std::io::{Error, ErrorKind};
use std::os::raw::c_int;

#[cfg(windows)]
use winapi::um::handleapi::INVALID_HANDLE_VALUE;
#[cfg(windows)]
use winapi::um::processenv::GetStdHandle;
#[cfg(windows)]
use winapi::um::winbase::STD_INPUT_HANDLE;

#[cfg(not(windows))]
use libc::{ioctl, c_void};

#[cfg(not(windows))]
pub fn rpl_ioctl(fd: c_int, request: c_int, arg: *mut c_void) -> Result<c_int, Error> {
    let res = unsafe { ioctl(fd, request as u32, arg) };
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(res)
    }
}

#[cfg(windows)]
fn primary_ioctl(fd: c_int, request: c_int, arg: *mut c_void) -> Result<c_int, Error> {
    // We don't support FIONBIO on pipes here. If you want to make pipe
    // fds non-blocking, use the gnulib 'nonblocking' module, until
    // gnulib implements fcntl F_GETFL / F_SETFL with O_NONBLOCK.
    
    let handle = unsafe { GetStdHandle(STD_INPUT_HANDLE) };
    if handle != INVALID_HANDLE_VALUE {
        Err(Error::new(ErrorKind::Unsupported, "operation not supported"))
    } else {
        Err(Error::new(ErrorKind::InvalidInput, "bad file descriptor"))
    }
}

#[cfg(windows)]
pub fn ioctl(fd: c_int, request: c_int, arg: *mut c_void) -> Result<c_int, Error> {
    #[cfg(feature = "windows_sockets")]
    {
        // execute_all_ioctl_hooks would need to be implemented separately
        execute_all_ioctl_hooks(primary_ioctl, fd, request, arg)
    }
    #[cfg(not(feature = "windows_sockets"))]
    {
        primary_ioctl(fd, request, arg)
    }
}