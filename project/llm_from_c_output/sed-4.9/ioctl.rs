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

#[cfg(windows)]
use std::os::windows::io::AsRawHandle;
use std::io;
use std::os::fd::AsRawFd;

#[cfg(not(windows))]
pub fn rpl_ioctl(fd: i32, request: u32, buf: Option<&mut [u8]>) -> io::Result<()> {
    unsafe {
        let res = libc::ioctl(fd, request as libc::c_ulong, buf.map(|b| b.as_mut_ptr()).unwrap_or(std::ptr::null_mut()));
        if res == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

#[cfg(windows)]
pub fn ioctl(fd: i32, request: u32, buf: Option<&mut [u8]>) -> io::Result<()> {
    use std::os::windows::io::FromRawHandle;
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    use winapi::um::errhandlingapi::GetLastError;
    use std::ptr;

    unsafe {
        let handle = libc::get_osfhandle(fd);
        if handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::from_raw_os_error(winapi::shared::winerror::ERROR_INVALID_HANDLE as i32));
        }

        // We don't support FIONBIO on pipes here. If you want to make pipe
        // fds non-blocking, use the gnulib 'nonblocking' module, until
        // gnulib implements fcntl F_GETFL / F_SETFL with O_NONBLOCK.
        Err(io::Error::from_raw_os_error(winapi::shared::winerror::ERROR_NOT_SUPPORTED as i32))
    }
}

#[cfg(windows)]
fn primary_ioctl(fd: i32, request: u32, arg: Option<&mut [u8]>) -> io::Result<()> {
    use winapi::um::handleapi::INVALID_HANDLE_VALUE;
    
    unsafe {
        let handle = libc::get_osfhandle(fd);
        if handle == INVALID_HANDLE_VALUE {
            return Err(io::Error::from_raw_os_error(winapi::shared::winerror::ERROR_INVALID_HANDLE as i32));
        }
        Err(io::Error::from_raw_os_error(winapi::shared::winerror::ERROR_NOT_SUPPORTED as i32))
    }
}

#[cfg(windows)]
pub fn execute_all_ioctl_hooks(fd: i32, request: u32, arg: Option<&mut [u8]>) -> io::Result<()> {
    primary_ioctl(fd, request, arg)
}