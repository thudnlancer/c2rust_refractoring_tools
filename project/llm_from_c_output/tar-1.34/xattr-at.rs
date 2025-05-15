//! Extended attribute operations relative to a directory file descriptor.
//!
//! This module provides Rust equivalents of the C functions for operating with
//! extended file attributes relative to a directory file descriptor.

use std::os::unix::prelude::*;
use std::path::Path;
use std::io::{self, Error, ErrorKind};
use libc::{c_int, c_char, size_t, ssize_t};

const ENOATTR: i32 = libc::ENODATA; // No such attribute

/// Sets the value of an extended attribute relative to a directory file descriptor.
///
/// Equivalent to `setxattrat` in C.
pub fn setxattrat(
    dir_fd: RawFd,
    path: &Path,
    name: &str,
    value: &[u8],
    flags: c_int,
) -> io::Result<()> {
    let c_path = path_to_cstring(path)?;
    let c_name = str_to_cstring(name)?;
    
    unsafe {
        let res = libc::setxattr(
            c_path.as_ptr(),
            c_name.as_ptr(),
            value.as_ptr() as *const c_char,
            value.len() as size_t,
            flags,
        );
        
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

/// Sets the value of an extended attribute on a symlink relative to a directory fd.
///
/// Equivalent to `lsetxattrat` in C.
pub fn lsetxattrat(
    dir_fd: RawFd,
    path: &Path,
    name: &str,
    value: &[u8],
    flags: c_int,
) -> io::Result<()> {
    let c_path = path_to_cstring(path)?;
    let c_name = str_to_cstring(name)?;
    
    unsafe {
        let res = libc::lsetxattr(
            c_path.as_ptr(),
            c_name.as_ptr(),
            value.as_ptr() as *const c_char,
            value.len() as size_t,
            flags,
        );
        
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

/// Gets the value of an extended attribute relative to a directory file descriptor.
///
/// Equivalent to `getxattrat` in C.
pub fn getxattrat(
    dir_fd: RawFd,
    path: &Path,
    name: &str,
    value: &mut [u8],
) -> io::Result<ssize_t> {
    let c_path = path_to_cstring(path)?;
    let c_name = str_to_cstring(name)?;
    
    unsafe {
        let res = libc::getxattr(
            c_path.as_ptr(),
            c_name.as_ptr(),
            value.as_mut_ptr() as *mut c_char,
            value.len() as size_t,
        );
        
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Gets the value of an extended attribute on a symlink relative to a directory fd.
///
/// Equivalent to `lgetxattrat` in C.
pub fn lgetxattrat(
    dir_fd: RawFd,
    path: &Path,
    name: &str,
    value: &mut [u8],
) -> io::Result<ssize_t> {
    let c_path = path_to_cstring(path)?;
    let c_name = str_to_cstring(name)?;
    
    unsafe {
        let res = libc::lgetxattr(
            c_path.as_ptr(),
            c_name.as_ptr(),
            value.as_mut_ptr() as *mut c_char,
            value.len() as size_t,
        );
        
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Lists extended attribute names relative to a directory file descriptor.
///
/// Equivalent to `listxattrat` in C.
pub fn listxattrat(
    dir_fd: RawFd,
    path: &Path,
    list: &mut [u8],
) -> io::Result<ssize_t> {
    let c_path = path_to_cstring(path)?;
    
    unsafe {
        let res = libc::listxattr(
            c_path.as_ptr(),
            list.as_mut_ptr() as *mut c_char,
            list.len() as size_t,
        );
        
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Lists extended attribute names on a symlink relative to a directory fd.
///
/// Equivalent to `llistxattrat` in C.
pub fn llistxattrat(
    dir_fd: RawFd,
    path: &Path,
    list: &mut [u8],
) -> io::Result<ssize_t> {
    let c_path = path_to_cstring(path)?;
    
    unsafe {
        let res = libc::llistxattr(
            c_path.as_ptr(),
            list.as_mut_ptr() as *mut c_char,
            list.len() as size_t,
        );
        
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

// Helper functions

fn path_to_cstring(path: &Path) -> io::Result<std::ffi::CString> {
    std::ffi::CString::new(path.as_os_str().as_bytes())
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "path contains null byte"))
}

fn str_to_cstring(s: &str) -> io::Result<std::ffi::CString> {
    std::ffi::CString::new(s)
        .map_err(|_| Error::new(ErrorKind::InvalidInput, "string contains null byte"))
}