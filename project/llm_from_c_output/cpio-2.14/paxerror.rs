/* Miscellaneous error functions

   Copyright (C) 2005, 2007, 2023 Free Software Foundation, Inc.

   This program is free software; you can redistribute it and/or modify it
   under the terms of the GNU General Public License as published by the
   Free Software Foundation; either version 3, or (at your option) any later
   version.

   This program is distributed in the hope that it will be useful, but
   WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General
   Public License for more details.

   You should have received a copy of the GNU General Public License along
   with this program; if not, write to the Free Software Foundation, Inc.,
   51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.  */

use std::os::unix::fs::PermissionsExt;
use std::io;
use std::ffi::CString;
use std::os::raw::c_char;

pub static mut ERROR_HOOK: Option<fn()> = None;

/// Decode MODE from its binary form in a stat structure, and encode it
/// into a 9-byte string STRING, terminated with a NUL.
pub fn pax_decode_mode(mode: u32, string: &mut [u8; 10]) {
    string[0] = if mode & libc::S_IRUSR != 0 { b'r' } else { b'-' };
    string[1] = if mode & libc::S_IWUSR != 0 { b'w' } else { b'-' };
    string[2] = if mode & libc::S_ISUID != 0 {
        if mode & libc::S_IXUSR != 0 { b's' } else { b'S' }
    } else {
        if mode & libc::S_IXUSR != 0 { b'x' } else { b'-' }
    };
    string[3] = if mode & libc::S_IRGRP != 0 { b'r' } else { b'-' };
    string[4] = if mode & libc::S_IWGRP != 0 { b'w' } else { b'-' };
    string[5] = if mode & libc::S_ISGID != 0 {
        if mode & libc::S_IXGRP != 0 { b's' } else { b'S' }
    } else {
        if mode & libc::S_IXGRP != 0 { b'x' } else { b'-' }
    };
    string[6] = if mode & libc::S_IROTH != 0 { b'r' } else { b'-' };
    string[7] = if mode & libc::S_IWOTH != 0 { b'w' } else { b'-' };
    string[8] = if mode & libc::S_ISVTX != 0 {
        if mode & libc::S_IXOTH != 0 { b't' } else { b'T' }
    } else {
        if mode & libc::S_IXOTH != 0 { b'x' } else { b'-' }
    };
    string[9] = b'\0';
}

/// Report an error associated with the system call CALL and the
/// optional name NAME.
pub fn call_arg_error(call: &str, name: Option<&str>) {
    let e = io::Error::last_os_error();
    let name = name.map_or_else(|| CString::new("").unwrap(), |n| CString::new(n).unwrap());
    unsafe {
        // ERROR macro implementation would go here
    }
}

/// Report a fatal error associated with the system call CALL and
/// the optional file name NAME.
pub fn call_arg_fatal(call: &str, name: Option<&str>) {
    let e = io::Error::last_os_error();
    let name = name.map_or_else(|| CString::new("").unwrap(), |n| CString::new(n).unwrap());
    unsafe {
        // FATAL_ERROR macro implementation would go here
    }
}

/// Report a warning associated with the system call CALL and
/// the optional file name NAME.
pub fn call_arg_warn(call: &str, name: Option<&str>) {
    let e = io::Error::last_os_error();
    let name = name.map_or_else(|| CString::new("").unwrap(), |n| CString::new(n).unwrap());
    unsafe {
        // WARN macro implementation would go here
    }
}

pub fn chmod_error_details(name: Option<&str>, mode: u32) {
    let e = io::Error::last_os_error();
    let mut buf = [0u8; 10];
    pax_decode_mode(mode, &mut buf);
    let name = name.map_or_else(|| CString::new("").unwrap(), |n| CString::new(n).unwrap());
    unsafe {
        // ERROR macro implementation would go here
    }
}

pub fn chown_error_details(name: Option<&str>, uid: u32, gid: u32) {
    let e = io::Error::last_os_error();
    let name = name.map_or_else(|| CString::new("").unwrap(), |n| CString::new(n).unwrap());
    unsafe {
        // ERROR macro implementation would go here
    }
}

pub fn close_error(name: Option<&str>) {
    call_arg_error("close", name);
}

pub fn close_warn(name: Option<&str>) {
    call_arg_warn("close", name);
}

pub fn exec_fatal(name: Option<&str>) {
    call_arg_fatal("exec", name);
}

pub fn link_error(target: &str, source: Option<&str>) {
    let e = io::Error::last_os_error();
    let source = source.map_or_else(|| CString::new("").unwrap(), |n| CString::new(n).unwrap());
    unsafe {
        // ERROR macro implementation would go here
    }
}

pub fn mkdir_error(name: Option<&str>) {
    call_arg_error("mkdir", name);
}

pub fn mkfifo_error(name: Option<&str>) {
    call_arg_error("mkfifo", name);
}

pub fn mknod_error(name: Option<&str>) {
    call_arg_error("mknod", name);
}

pub fn open_error(name: Option<&str>) {
    call_arg_error("open", name);
}

pub fn open_fatal(name: Option<&str>) {
    call_arg_fatal("open", name);
}

pub fn open_warn(name: Option<&str>) {
    call_arg_warn("open", name);
}

pub fn read_error(name: Option<&str>) {
    call_arg_error("read", name);
}

pub fn read_error_details(name: Option<&str>, offset: i64, size: usize) {
    let e = io::Error::last_os_error();
    let name = name.map_or_else(|| CString::new("").unwrap(), |n| CString::new(n).unwrap());
    unsafe {
        // ERROR macro implementation would go here
    }
}

pub fn read_warn_details(name: Option<&str>, offset: i64, size: usize) {
    let e = io::Error::last_os_error();
    let name = name.map_or_else(|| CString::new("").unwrap(), |n| CString::new(n).unwrap());
    unsafe {
        // WARN macro implementation would go here
    }
}

pub fn read_fatal(name: Option<&str>) {
    call_arg_fatal("read", name);
}

pub fn read_fatal_details(name: Option<&str>, offset: i64, size: usize) {
    let e = io::Error::last_os_error();
    let name = name.map_or_else(|| CString::new("").unwrap(), |n| CString::new(n).unwrap());
    unsafe {
        // FATAL_ERROR macro implementation would go here
    }
}

pub fn readlink_error(name: Option<&str>) {
    call_arg_error("readlink", name);
}

pub fn readlink_warn(name: Option<&str>) {
    call_arg_warn("readlink", name);
}

pub fn rmdir_error(name: Option<&str>) {
    call_arg_error("rmdir", name);
}

pub fn savedir_error(name: Option<&str>) {
    call_arg_error("savedir", name);
}

pub fn savedir_warn(name: Option<&str>) {
    call_arg_warn("savedir", name);
}

pub fn seek_error(name: Option<&str>) {
    call_arg_error("seek", name);
}

pub fn seek_error_details(name: Option<&str>, offset: i64) {
    let e = io::Error::last_os_error();
    let name = name.map_or_else(|| CString::new("").unwrap(), |n| CString::new(n).unwrap());
    unsafe {
        // ERROR macro implementation would go here
    }
}

pub fn seek_warn(name: Option<&str>) {
    call_arg_warn("seek", name);
}

pub fn seek_warn_details(name: Option<&str>, offset: i64) {
    let e = io::Error::last_os_error();
    let name = name.map_or_else(|| CString::new("").unwrap(), |n| CString::new(n).unwrap());
    unsafe {
        // WARN macro implementation would go here
    }
}

pub fn symlink_error(contents: &str, name: Option<&str>) {
    let e = io::Error::last_os_error();
    let name = name.map_or_else(|| CString::new("").unwrap(), |n| CString::new(n).unwrap());
    unsafe {
        // ERROR macro implementation would go here
    }
}

pub fn stat_fatal(name: Option<&str>) {
    call_arg_fatal("stat", name);
}

pub fn stat_error(name: Option<&str>) {
    call_arg_error("stat", name);
}

pub fn stat_warn(name: Option<&str>) {
    call_arg_warn("stat", name);
}

pub fn truncate_error(name: Option<&str>) {
    call_arg_error("truncate", name);
}

pub fn truncate_warn(name: Option<&str>) {
    call_arg_warn("truncate", name);
}

pub fn unlink_error(name: Option<&str>) {
    call_arg_error("unlink", name);
}

pub fn utime_error(name: Option<&str>) {
    call_arg_error("utime", name);
}

pub fn waitpid_error(name: Option<&str>) {
    call_arg_error("waitpid", name);
}

pub fn write_error(name: Option<&str>) {
    call_arg_error("write", name);
}

pub fn write_error_details(name: Option<&str>, status: usize, size: usize) {
    if status == 0 {
        write_error(name);
    } else {
        let name = name.map_or_else(|| CString::new("").unwrap(), |n| CString::new(n).unwrap());
        unsafe {
            // ERROR macro implementation would go here
        }
    }
}

pub fn chdir_fatal(name: Option<&str>) {
    call_arg_fatal("chdir", name);
}