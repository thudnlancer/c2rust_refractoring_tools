/* DO NOT EDIT! GENERATED AUTOMATICALLY! */
/* Like <fcntl.h>, but with non-working flags defined to 0.

   Copyright (C) 2006-2023 Free Software Foundation, Inc.

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

/* written by Paul Eggert */

use libc::{c_int, mode_t};
use std::os::raw::{c_char, c_void};

pub const FD_CLOEXEC: c_int = 1;

pub const F_DUPFD: c_int = 1;
pub const F_GETFD: c_int = 2;
pub const F_DUPFD_CLOEXEC: c_int = 0x40000000;

pub const O_CLOEXEC: c_int = 0x40000000;
pub const O_DIRECT: c_int = 0;
pub const O_DIRECTORY: c_int = 0;
pub const O_DSYNC: c_int = 0;
pub const O_EXEC: c_int = 0;
pub const O_IGNORE_CTTY: c_int = 0;
pub const O_NDELAY: c_int = 0;
pub const O_NOATIME: c_int = 0;
pub const O_NONBLOCK: c_int = 0;
pub const O_NOCTTY: c_int = 0;
pub const O_NOFOLLOW: c_int = 0;
pub const O_NOLINK: c_int = 0;
pub const O_NOLINKS: c_int = 0;
pub const O_NOTRANS: c_int = 0;
pub const O_RSYNC: c_int = 0;
pub const O_SEARCH: c_int = 0;
pub const O_SYNC: c_int = 0;
pub const O_TTY_INIT: c_int = 0;
pub const O_BINARY: c_int = 0;
pub const O_TEXT: c_int = 0;

pub const AT_FDCWD: c_int = -3041965;
pub const AT_SYMLINK_NOFOLLOW: c_int = 4096;
pub const AT_REMOVEDIR: c_int = 1;
pub const AT_SYMLINK_FOLLOW: c_int = 2;
pub const AT_EACCESS: c_int = 4;
pub const AT_NO_AUTOMOUNT: c_int = 0;

extern "C" {
    pub fn creat(path: *const c_char, mode: mode_t) -> c_int;
    pub fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    pub fn open(path: *const c_char, oflag: c_int, ...) -> c_int;
    pub fn openat(fd: c_int, path: *const c_char, oflag: c_int, ...) -> c_int;
}

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    use libc::{HANDLE, INVALID_HANDLE_VALUE};
    use std::ptr;

    const DUPLICATE_SAME_ACCESS: u32 = 0x00000002;
    const HANDLE_FLAG_INHERIT: u32 = 0x00000001;

    extern "C" {
        fn GetCurrentProcess() -> HANDLE;
        fn DuplicateHandle(
            hSourceProcessHandle: HANDLE,
            hSourceHandle: HANDLE,
            hTargetProcessHandle: HANDLE,
            lpTargetHandle: *mut HANDLE,
            dwDesiredAccess: u32,
            bInheritHandle: i32,
            dwOptions: u32,
        ) -> i32;
        fn CloseHandle(hObject: HANDLE) -> i32;
        fn GetHandleInformation(hObject: HANDLE, lpdwFlags: *mut u32) -> i32;
    }

    pub unsafe fn dupfd(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int {
        let curr_process = GetCurrentProcess();
        let old_handle = libc::_get_osfhandle(oldfd);
        
        if old_handle == INVALID_HANDLE_VALUE {
            libc::errno = libc::EBADF;
            return -1;
        }

        let inherit = if (flags & O_CLOEXEC) != 0 { 0 } else { 1 };
        let mut new_handle = INVALID_HANDLE_VALUE;

        if DuplicateHandle(
            curr_process,
            old_handle,
            curr_process,
            &mut new_handle,
            0,
            inherit,
            DUPLICATE_SAME_ACCESS,
        ) == 0
        {
            -1
        } else {
            let duplicated_fd = libc::_open_osfhandle(new_handle as isize, flags);
            if duplicated_fd < 0 {
                CloseHandle(new_handle);
                -1
            } else {
                duplicated_fd
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
mod unix {
    use super::*;
    use libc::{dup, dup2, fcntl as libc_fcntl, F_DUPFD, F_GETFD, F_SETFD, FD_CLOEXEC};

    pub unsafe fn dupfd(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int {
        let fd = libc_fcntl(oldfd, F_DUPFD, newfd);
        if fd >= 0 && (flags & O_CLOEXEC) != 0 {
            let current_flags = libc_fcntl(fd, F_GETFD);
            if current_flags >= 0 {
                libc_fcntl(fd, F_SETFD, current_flags | FD_CLOEXEC);
            }
        }
        fd
    }
}

pub unsafe fn rpl_fcntl(fd: c_int, cmd: c_int, arg: c_int) -> c_int {
    match cmd {
        F_DUPFD => rpl_fcntl_dupfd(fd, arg),
        F_DUPFD_CLOEXEC => rpl_fcntl_dupfd_cloexec(fd, arg),
        _ => libc::fcntl(fd, cmd, arg),
    }
}

unsafe fn rpl_fcntl_dupfd(fd: c_int, target: c_int) -> c_int {
    #[cfg(target_os = "windows")]
    return windows::dupfd(fd, target, 0);
    #[cfg(not(target_os = "windows"))]
    return unix::dupfd(fd, target, 0);
}

unsafe fn rpl_fcntl_dupfd_cloexec(fd: c_int, target: c_int) -> c_int {
    #[cfg(target_os = "windows"))]
    return windows::dupfd(fd, target, O_CLOEXEC);
    #[cfg(not(target_os = "windows"))]
    return unix::dupfd(fd, target, O_CLOEXEC);
}