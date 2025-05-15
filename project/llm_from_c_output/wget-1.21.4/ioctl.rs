/* ioctl.rs --- wrappers for Windows ioctl function

   Copyright (C) 2008-2023 Free Software Foundation, Inc.

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

#[cfg(windows)]
use std::os::windows::io::AsRawHandle;
use std::io::{Error, ErrorKind};
use std::os::fd::AsRawFd;

#[cfg(not(windows))]
pub fn rpl_ioctl(fd: i32, request: u32, arg: Option<&mut [u8]>) -> Result<(), Error> {
    // Cast 'request' so that when the system's ioctl function takes a 64-bit
    // request argument, the value gets zero-extended, not sign-extended.
    let res = unsafe {
        libc::ioctl(fd, request as libc::c_ulong, arg.map_or(std::ptr::null_mut(), |a| a.as_mut_ptr()))
    };
    
    if res == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

#[cfg(windows)]
pub fn ioctl(fd: i32, request: u32, arg: Option<&mut [u8]>) -> Result<(), Error> {
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::ioapiset::DeviceIoControl;
    use std::os::windows::io::FromRawHandle;

    let handle = unsafe { std::os::windows::io::RawHandle::from_raw_handle(fd as _) };
    
    if handle == INVALID_HANDLE_VALUE {
        return Err(Error::new(ErrorKind::InvalidInput, "Invalid file descriptor"));
    }

    // We don't support FIONBIO on pipes here. If you want to make pipe
    // fds non-blocking, use the gnulib 'nonblocking' module, until
    // gnulib implements fcntl F_GETFL / F_SETFL with O_NONBLOCK.
    Err(Error::new(ErrorKind::Unsupported, "Operation not supported"))
}

#[cfg(windows)]
pub fn execute_all_ioctl_hooks(
    primary: fn(i32, u32, Option<&mut [u8]>) -> Result<(), Error>,
    fd: i32,
    request: u32,
    arg: Option<&mut [u8]>,
) -> Result<(), Error> {
    primary(fd, request, arg)
}

#[cfg(windows)]
pub fn primary_ioctl(fd: i32, request: u32, arg: Option<&mut [u8]>) -> Result<(), Error> {
    ioctl(fd, request, arg)
}