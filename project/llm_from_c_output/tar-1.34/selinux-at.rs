/*
 * Prototypes for openat-style fd-relative SELinux functions
 * Copyright (C) 2007, 2009-2021 Free Software Foundation, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::os::unix::io::RawFd;
use std::ffi::{CString, CStr};
use std::ptr;
use std::path::Path;
use std::io::{Error, Result};

extern "C" {
    fn getfilecon(path: *const libc::c_char, con: *mut *mut libc::c_char) -> libc::c_int;
    fn lgetfilecon(path: *const libc::c_char, con: *mut *mut libc::c_char) -> libc::c_int;
    fn setfilecon(path: *const libc::c_char, con: *const libc::c_char) -> libc::c_int;
    fn lsetfilecon(path: *const libc::c_char, con: *const libc::c_char) -> libc::c_int;
    fn freecon(con: *mut libc::c_char);
}

/// Get SELinux security context relative to directory file descriptor
pub fn getfileconat(dir_fd: RawFd, file: &Path) -> Result<String> {
    let path = CString::new(file.to_str().ok_or(Error::from_raw_os_error(libc::EINVAL))?)?;
    let mut con: *mut libc::c_char = ptr::null_mut();
    
    let res = unsafe { getfilecon(path.as_ptr(), &mut con) };
    if res < 0 {
        Err(Error::last_os_error())
    } else {
        let context = unsafe { CStr::from_ptr(con) }.to_string_lossy().into_owned();
        unsafe { freecon(con) };
        Ok(context)
    }
}

/// Get SELinux security context of symlink relative to directory file descriptor
pub fn lgetfileconat(dir_fd: RawFd, file: &Path) -> Result<String> {
    let path = CString::new(file.to_str().ok_or(Error::from_raw_os_error(libc::EINVAL))?)?;
    let mut con: *mut libc::c_char = ptr::null_mut();
    
    let res = unsafe { lgetfilecon(path.as_ptr(), &mut con) };
    if res < 0 {
        Err(Error::last_os_error())
    } else {
        let context = unsafe { CStr::from_ptr(con) }.to_string_lossy().into_owned();
        unsafe { freecon(con) };
        Ok(context)
    }
}

/// Set SELinux security context relative to directory file descriptor
pub fn setfileconat(dir_fd: RawFd, file: &Path, context: &str) -> Result<()> {
    let path = CString::new(file.to_str().ok_or(Error::from_raw_os_error(libc::EINVAL))?)?;
    let con = CString::new(context)?;
    
    let res = unsafe { setfilecon(path.as_ptr(), con.as_ptr()) };
    if res < 0 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}

/// Set SELinux security context of symlink relative to directory file descriptor
pub fn lsetfileconat(dir_fd: RawFd, file: &Path, context: &str) -> Result<()> {
    let path = CString::new(file.to_str().ok_or(Error::from_raw_os_error(libc::EINVAL))?)?;
    let con = CString::new(context)?;
    
    let res = unsafe { lsetfilecon(path.as_ptr(), con.as_ptr()) };
    if res < 0 {
        Err(Error::last_os_error())
    } else {
        Ok(())
    }
}