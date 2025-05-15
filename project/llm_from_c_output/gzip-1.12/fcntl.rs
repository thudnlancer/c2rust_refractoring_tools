/* Like <fcntl.h>, but with non-working flags defined to 0.

   Copyright (C) 2006-2022 Free Software Foundation, Inc.

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

use libc::{c_int, mode_t};
use std::os::raw::{c_char, c_void};

// Fix up the FD_* macros
pub const FD_CLOEXEC: c_int = 1;

// Fix up the supported F_* macros
pub const F_DUPFD_CLOEXEC: c_int = 0x40000000;
pub const F_DUPFD: c_int = 1;
pub const F_GETFD: c_int = 2;

// Fix up the O_* macros
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

// Fix up the AT_* macros
pub const AT_FDCWD: c_int = -3041965;
pub const AT_SYMLINK_NOFOLLOW: c_int = 4096;
pub const AT_REMOVEDIR: c_int = 1;
pub const AT_SYMLINK_FOLLOW: c_int = 2;
pub const AT_EACCESS: c_int = 4;
pub const AT_NO_AUTOMOUNT: c_int = 0;

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    use std::os::windows::io::{AsRawHandle, FromRawHandle, IntoRawHandle};
    use winapi::um::handleapi::{DuplicateHandle, INVALID_HANDLE_VALUE};
    use winapi::um::winbase::DUPLICATE_SAME_ACCESS;
    use winapi::um::winnt::HANDLE_FLAG_INHERIT;

    pub fn dupfd(oldfd: c_int, newfd: c_int, flags: c_int) -> c_int {
        unsafe {
            let curr_process = winapi::um::processthreadsapi::GetCurrentProcess();
            let old_handle = libc::get_osfhandle(oldfd);
            
            if old_handle == INVALID_HANDLE_VALUE {
                errno::set_errno(errno::Errno(libc::EBADF));
                return -1;
            }

            let inherit = if flags & O_CLOEXEC != 0 {
                0
            } else {
                HANDLE_FLAG_INHERIT
            };

            let mut new_handle = INVALID_HANDLE_VALUE;
            if DuplicateHandle(
                curr_process,
                old_handle,
                curr_process,
                &mut new_handle,
                0,
                inherit as i32,
                DUPLICATE_SAME_ACCESS,
            ) == 0
            {
                let err = std::io::Error::last_os_error();
                errno::set_errno(errno::Errno(err.raw_os_error().unwrap_or(libc::EINVAL)));
                return -1;
            }

            let duplicated_fd = libc::open_osfhandle(new_handle as isize, flags);
            if duplicated_fd < 0 {
                winapi::um::handleapi::CloseHandle(new_handle);
                return -1;
            }

            if newfd <= duplicated_fd {
                return duplicated_fd;
            }

            // Need to close the fd and try again
            libc::close(duplicated_fd);
            dupfd(oldfd, newfd, flags)
        }
    }
}

pub fn fcntl(fd: c_int, cmd: c_int, arg: c_int) -> c_int {
    unsafe { libc::fcntl(fd, cmd, arg) }
}

pub fn open(path: *const c_char, flags: c_int, mode: mode_t) -> c_int {
    unsafe { libc::open(path, flags, mode) }
}

pub fn openat(dirfd: c_int, path: *const c_char, flags: c_int, mode: mode_t) -> c_int {
    unsafe { libc::openat(dirfd, path, flags, mode) }
}

pub fn creat(path: *const c_char, mode: mode_t) -> c_int {
    unsafe { libc::creat(path, mode) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;
    use tempfile::NamedTempFile;

    #[test]
    fn test_open() {
        let tempfile = NamedTempFile::new().unwrap();
        let path = CString::new(tempfile.path().to_str().unwrap()).unwrap();
        let fd = open(path.as_ptr(), O_RDONLY, 0);
        assert!(fd >= 0);
        unsafe { libc::close(fd) };
    }
}