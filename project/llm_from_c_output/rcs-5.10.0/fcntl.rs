/* This Rust code provides a safe wrapper around the fcntl system call functionality,
   with similar features to the original C code but using Rust's safety features. */

use libc::{c_int, c_void, dup, dup2, fcntl as libc_fcntl, F_DUPFD, F_DUPFD_CLOEXEC, F_GETFD, F_SETFD, FD_CLOEXEC};
use std::os::unix::io::RawFd;
use std::io::{Error, Result};

const GNULIB_defined_F_DUPFD_CLOEXEC: bool = false;

/// Safe wrapper for fcntl F_DUPFD operation
pub fn fcntl_dupfd(fd: RawFd, target: RawFd) -> Result<RawFd> {
    unsafe {
        let res = libc_fcntl(fd, F_DUPFD, target);
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Safe wrapper for fcntl F_DUPFD_CLOEXEC operation
pub fn fcntl_dupfd_cloexec(fd: RawFd, target: RawFd) -> Result<RawFd> {
    if !GNULIB_defined_F_DUPFD_CLOEXEC {
        unsafe {
            let res = libc_fcntl(fd, F_DUPFD_CLOEXEC, target);
            if res != -1 {
                return Ok(res);
            }
            if Error::last_os_error().raw_os_error() != Some(libc::EINVAL) {
                return Err(Error::last_os_error());
            }
        }
    }

    let new_fd = fcntl_dupfd(fd, target)?;
    
    unsafe {
        let flags = libc_fcntl(new_fd, F_GETFD);
        if flags == -1 {
            let e = Error::last_os_error();
            let _ = libc::close(new_fd);
            return Err(e);
        }
        
        if libc_fcntl(new_fd, F_SETFD, flags | FD_CLOEXEC) == -1 {
            let e = Error::last_os_error();
            let _ = libc::close(new_fd);
            return Err(e);
        }
    }
    
    Ok(new_fd)
}

/// Safe wrapper for fcntl F_GETFD operation
pub fn fcntl_getfd(fd: RawFd) -> Result<c_int> {
    unsafe {
        let res = libc_fcntl(fd, F_GETFD);
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Safe wrapper for fcntl F_SETFD operation
pub fn fcntl_setfd(fd: RawFd, flags: c_int) -> Result<()> {
    unsafe {
        if libc_fcntl(fd, F_SETFD, flags) == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

/// Comprehensive fcntl wrapper that handles multiple operations
pub fn fcntl(fd: RawFd, cmd: c_int, arg: c_int) -> Result<c_int> {
    match cmd {
        F_DUPFD => fcntl_dupfd(fd, arg),
        F_DUPFD_CLOEXEC => fcntl_dupfd_cloexec(fd, arg),
        F_GETFD => fcntl_getfd(fd),
        _ => {
            unsafe {
                let res = libc_fcntl(fd, cmd, arg);
                if res == -1 {
                    Err(Error::last_os_error())
                } else {
                    Ok(res)
                }
            }
        }
    }
}

/// Helper function to duplicate a file descriptor with specific flags
fn dupfd(oldfd: RawFd, newfd: RawFd, flags: c_int) -> Result<RawFd> {
    let mut current = newfd;
    loop {
        unsafe {
            let res = dup2(oldfd, current);
            if res != -1 {
                if flags & libc::O_CLOEXEC != 0 {
                    let _ = fcntl_setfd(res, FD_CLOEXEC);
                }
                return Ok(res);
            }
            
            if Error::last_os_error().raw_os_error() != Some(libc::EMFILE) {
                return Err(Error::last_os_error());
            }
            
            current += 1;
            if current >= libc::getdtablesize() {
                return Err(Error::from_raw_os_error(libc::EMFILE));
            }
        }
    }
}