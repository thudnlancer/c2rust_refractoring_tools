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
use std::io;
use std::os::raw::c_int;

#[cfg(not(windows))]
pub fn rpl_ioctl(fd: c_int, request: c_int, arg: *mut std::ffi::c_void) -> io::Result<c_int> {
    // Cast 'request' so that when the system's ioctl function takes a 64-bit
    // request argument, the value gets zero-extended, not sign-extended.
    let request = request as u32;
    
    unsafe {
        match libc::ioctl(fd, request, arg) {
            -1 => Err(io::Error::last_os_error()),
            res => Ok(res),
        }
    }
}

#[cfg(windows)]
pub fn ioctl(fd: c_int, request: c_int, arg: *mut std::ffi::c_void) -> io::Result<c_int> {
    use std::os::windows::io::FromRawHandle;
    
    let handle = unsafe { std::os::windows::io::RawHandle::from_raw_handle(fd as _) };
    if handle == std::ptr::null_mut() {
        return Err(io::Error::from_raw_os_error(libc::EBADF));
    }
    
    // We don't support FIONBIO on pipes here. If you want to make pipe
    // fds non-blocking, use the gnulib 'nonblocking' module, until
    // gnulib implements fcntl F_GETFL / F_SETFL with O_NONBLOCK.
    Err(io::Error::from_raw_os_error(libc::ENOSYS))
}

#[cfg(windows)]
pub fn execute_all_ioctl_hooks(
    primary: fn(c_int, c_int, *mut std::ffi::c_void) -> io::Result<c_int>,
    fd: c_int,
    request: c_int,
    arg: *mut std::ffi::c_void,
) -> io::Result<c_int> {
    primary(fd, request, arg)
}