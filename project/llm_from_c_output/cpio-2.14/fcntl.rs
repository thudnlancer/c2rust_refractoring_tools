/* This is a Rust translation of the C fcntl.h functionality.
   It provides file descriptor control operations in a safe Rust interface. */

use libc::{c_int, c_void, mode_t};
use std::os::unix::io::RawFd;

// Constants from fcntl.h
pub const FD_CLOEXEC: c_int = 1;

pub const F_DUPFD: c_int = 0;
pub const F_GETFD: c_int = 1;
pub const F_SETFD: c_int = 2;
pub const F_GETFL: c_int = 3;
pub const F_SETFL: c_int = 4;
pub const F_DUPFD_CLOEXEC: c_int = 0x40000000;

pub const O_CLOEXEC: c_int = 0x40000000;
pub const O_RDONLY: c_int = 0;
pub const O_WRONLY: c_int = 1;
pub const O_RDWR: c_int = 2;
pub const O_ACCMODE: c_int = O_RDONLY | O_WRONLY | O_RDWR;
pub const O_APPEND: c_int = 0x0008;
pub const O_CREAT: c_int = 0x0200;
pub const O_EXCL: c_int = 0x0800;
pub const O_TRUNC: c_int = 0x0400;
pub const O_NONBLOCK: c_int = 0x4000;
pub const O_NOCTTY: c_int = 0x8000;
pub const O_SYNC: c_int = 0x2000;
pub const O_DSYNC: c_int = 0x10000;
pub const O_RSYNC: c_int = 0x20000;
pub const O_NOFOLLOW: c_int = 0x100000;
pub const O_DIRECTORY: c_int = 0x200000;
pub const O_EXEC: c_int = O_RDONLY;
pub const O_SEARCH: c_int = O_RDONLY;
pub const O_PATH: c_int = 0x1000000;
pub const O_TMPFILE: c_int = 0x2000000;

#[cfg(target_os = "linux")]
pub const O_DIRECT: c_int = 0x40000;
#[cfg(not(target_os = "linux"))]
pub const O_DIRECT: c_int = 0;

pub const AT_FDCWD: c_int = -100;
pub const AT_SYMLINK_NOFOLLOW: c_int = 0x100;
pub const AT_REMOVEDIR: c_int = 0x200;
pub const AT_SYMLINK_FOLLOW: c_int = 0x400;
pub const AT_EACCESS: c_int = 0x200;
pub const AT_NO_AUTOMOUNT: c_int = 0x800;

/// Safe wrapper around fcntl F_DUPFD operation
pub fn dupfd(fd: RawFd, target: c_int) -> Result<RawFd, std::io::Error> {
    unsafe {
        let res = libc::fcntl(fd, F_DUPFD, target);
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Safe wrapper around fcntl F_DUPFD_CLOEXEC operation
pub fn dupfd_cloexec(fd: RawFd, target: c_int) -> Result<RawFd, std::io::Error> {
    unsafe {
        let res = libc::fcntl(fd, F_DUPFD_CLOEXEC, target);
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Safe wrapper around fcntl F_GETFD operation
pub fn getfd(fd: RawFd) -> Result<c_int, std::io::Error> {
    unsafe {
        let res = libc::fcntl(fd, F_GETFD);
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Safe wrapper around fcntl F_SETFD operation
pub fn setfd(fd: RawFd, flags: c_int) -> Result<(), std::io::Error> {
    unsafe {
        let res = libc::fcntl(fd, F_SETFD, flags);
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

/// Safe wrapper around fcntl F_GETFL operation
pub fn getfl(fd: RawFd) -> Result<c_int, std::io::Error> {
    unsafe {
        let res = libc::fcntl(fd, F_GETFL);
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Safe wrapper around fcntl F_SETFL operation
pub fn setfl(fd: RawFd, flags: c_int) -> Result<(), std::io::Error> {
    unsafe {
        let res = libc::fcntl(fd, F_SETFL, flags);
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

/// Safe wrapper around open function
pub fn open(path: &std::path::Path, flags: c_int, mode: mode_t) -> Result<RawFd, std::io::Error> {
    let c_path = std::ffi::CString::new(path.to_str().unwrap()).unwrap();
    unsafe {
        let res = libc::open(c_path.as_ptr(), flags, mode);
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Safe wrapper around creat function
pub fn creat(path: &std::path::Path, mode: mode_t) -> Result<RawFd, std::io::Error> {
    let c_path = std::ffi::CString::new(path.to_str().unwrap()).unwrap();
    unsafe {
        let res = libc::creat(c_path.as_ptr(), mode);
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Safe wrapper around openat function
pub fn openat(dirfd: RawFd, path: &std::path::Path, flags: c_int, mode: mode_t) -> Result<RawFd, std::io::Error> {
    let c_path = std::ffi::CString::new(path.to_str().unwrap()).unwrap();
    unsafe {
        let res = libc::openat(dirfd, c_path.as_ptr(), flags, mode);
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Sets the close-on-exec flag for a file descriptor
pub fn set_cloexec(fd: RawFd) -> Result<(), std::io::Error> {
    let flags = getfd(fd)?;
    setfd(fd, flags | FD_CLOEXEC)
}

/// Clears the close-on-exec flag for a file descriptor
pub fn clear_cloexec(fd: RawFd) -> Result<(), std::io::Error> {
    let flags = getfd(fd)?;
    setfd(fd, flags & !FD_CLOEXEC)
}