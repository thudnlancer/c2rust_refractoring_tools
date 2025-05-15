/* This Rust code provides a safe wrapper around file descriptor operations,
   similar to the functionality in the original C fcntl.h implementation. */

use libc::{c_int, c_void, dup, dup2, fcntl as libc_fcntl, F_DUPFD, F_DUPFD_CLOEXEC, F_GETFD, F_SETFD, FD_CLOEXEC};
use std::os::unix::io::RawFd;
use std::io::{Error, Result};

// Constants matching the original C defines
pub const O_CLOEXEC: c_int = 0o2000000;
pub const F_DUPFD_CLOEXEC: c_int = F_DUPFD | O_CLOEXEC;

/// Safe wrapper for fcntl operations
pub fn fcntl(fd: RawFd, cmd: c_int, arg: c_int) -> Result<RawFd> {
    unsafe {
        let res = libc_fcntl(fd, cmd, arg);
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Implementation of F_DUPFD functionality
pub fn dupfd(fd: RawFd, target: RawFd) -> Result<RawFd> {
    unsafe {
        let res = libc_fcntl(fd, F_DUPFD, target);
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Implementation of F_DUPFD_CLOEXEC functionality
pub fn dupfd_cloexec(fd: RawFd, target: RawFd) -> Result<RawFd> {
    // Try native F_DUPFD_CLOEXEC first
    let res = unsafe { libc_fcntl(fd, F_DUPFD_CLOEXEC, target) };
    
    if res != -1 {
        return Ok(res);
    }
    
    // Fallback to manual implementation if native fails
    let new_fd = dupfd(fd, target)?;
    
    // Set close-on-exec flag
    let flags = unsafe { libc_fcntl(new_fd, F_GETFD, 0) };
    if flags == -1 {
        let e = Error::last_os_error();
        let _ = unsafe { libc::close(new_fd) };
        return Err(e);
    }
    
    if unsafe { libc_fcntl(new_fd, F_SETFD, flags | FD_CLOEXEC) } == -1 {
        let e = Error::last_os_error();
        let _ = unsafe { libc::close(new_fd) };
        return Err(e);
    }
    
    Ok(new_fd)
}

/// Safe wrapper for dup2
pub fn dup2(oldfd: RawFd, newfd: RawFd) -> Result<RawFd> {
    unsafe {
        let res = dup2(oldfd, newfd);
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Safe wrapper for dup
pub fn dup(fd: RawFd) -> Result<RawFd> {
    unsafe {
        let res = dup(fd);
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Get file descriptor flags
pub fn getfd(fd: RawFd) -> Result<c_int> {
    unsafe {
        let res = libc_fcntl(fd, F_GETFD);
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Set file descriptor flags
pub fn setfd(fd: RawFd, flags: c_int) -> Result<()> {
    unsafe {
        if libc_fcntl(fd, F_SETFD, flags) == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

/// Set close-on-exec flag for a file descriptor
pub fn set_cloexec(fd: RawFd) -> Result<()> {
    let flags = getfd(fd)?;
    setfd(fd, flags | FD_CLOEXEC)
}