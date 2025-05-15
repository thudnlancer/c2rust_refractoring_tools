/* This is a Rust translation of the provided C fcntl.h implementation.
   It maintains the same functionality while following Rust safety practices. */

use libc::{c_int, c_void, mode_t};
use std::os::unix::io::RawFd;

// Constants from fcntl.h
pub const FD_CLOEXEC: c_int = 1;
pub const F_DUPFD: c_int = 1;
pub const F_GETFD: c_int = 2;
pub const F_DUPFD_CLOEXEC: c_int = 0x40000000;

// O_* flags
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

// AT_* constants
pub const AT_FDCWD: c_int = -3041965;
pub const AT_SYMLINK_NOFOLLOW: c_int = 4096;
pub const AT_REMOVEDIR: c_int = 1;
pub const AT_SYMLINK_FOLLOW: c_int = 2;
pub const AT_EACCESS: c_int = 4;

/// Wrapper for fcntl system call
pub fn fcntl(fd: RawFd, cmd: c_int, arg: c_int) -> Result<RawFd, std::io::Error> {
    unsafe {
        let res = libc::fcntl(fd, cmd, arg);
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Implementation of F_DUPFD functionality
pub fn dupfd(oldfd: RawFd, target: RawFd) -> Result<RawFd, std::io::Error> {
    fcntl(oldfd, F_DUPFD, target)
}

/// Implementation of F_DUPFD_CLOEXEC functionality
pub fn dupfd_cloexec(oldfd: RawFd, target: RawFd) -> Result<RawFd, std::io::Error> {
    let newfd = dupfd(oldfd, target)?;
    
    // Set FD_CLOEXEC on the new file descriptor
    match fcntl(newfd, F_GETFD, 0) {
        Ok(flags) => {
            fcntl(newfd, F_SETFD, flags | FD_CLOEXEC)?;
            Ok(newfd)
        }
        Err(e) => {
            let _ = libc::close(newfd); // Ignore errors during cleanup
            Err(e)
        }
    }
}

/// Wrapper for open system call
pub fn open(path: &str, flags: c_int, mode: mode_t) -> Result<RawFd, std::io::Error> {
    let c_path = std::ffi::CString::new(path).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid path"))?;
    
    unsafe {
        let res = libc::open(c_path.as_ptr(), flags, mode);
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Wrapper for openat system call
pub fn openat(dirfd: RawFd, path: &str, flags: c_int, mode: mode_t) -> Result<RawFd, std::io::Error> {
    let c_path = std::ffi::CString::new(path).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid path"))?;
    
    unsafe {
        let res = libc::openat(dirfd, c_path.as_ptr(), flags, mode);
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Wrapper for creat system call
pub fn creat(path: &str, mode: mode_t) -> Result<RawFd, std::io::Error> {
    let c_path = std::ffi::CString::new(path).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid path"))?;
    
    unsafe {
        let res = libc::creat(c_path.as_ptr(), mode);
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

// Additional utility functions can be added here as needed