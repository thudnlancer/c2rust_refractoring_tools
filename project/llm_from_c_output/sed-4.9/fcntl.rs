/* This Rust code is a translation of the C fcntl.h functionality, providing
   file descriptor control operations while following Rust's safety practices. */

use libc::{c_int, c_void, dup2, fcntl as libc_fcntl, F_DUPFD, F_GETFD, F_SETFD, FD_CLOEXEC};
use std::os::unix::io::RawFd;
use std::io::{Error, Result};

// Constants from fcntl.h
pub const O_CLOEXEC: c_int = 0x40000000;
pub const F_DUPFD_CLOEXEC: c_int = 0x40000000;

/// Safe wrapper around fcntl F_DUPFD operation
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

/// Safe wrapper around fcntl F_DUPFD_CLOEXEC operation
pub fn dupfd_cloexec(fd: RawFd, target: RawFd) -> Result<RawFd> {
    // First try F_DUPFD_CLOEXEC if available
    unsafe {
        let res = libc_fcntl(fd, F_DUPFD_CLOEXEC, target);
        if res != -1 {
            return Ok(res);
        }
    }

    // Fallback to F_DUPFD + setting FD_CLOEXEC
    let new_fd = dupfd(fd, target)?;
    
    match set_cloexec(new_fd, true) {
        Ok(_) => Ok(new_fd),
        Err(e) => {
            let _ = close_fd(new_fd);
            Err(e)
        }
    }
}

/// Set or clear FD_CLOEXEC flag on a file descriptor
pub fn set_cloexec(fd: RawFd, enable: bool) -> Result<()> {
    unsafe {
        let flags = libc_fcntl(fd, F_GETFD);
        if flags == -1 {
            return Err(Error::last_os_error());
        }
        
        let new_flags = if enable {
            flags | FD_CLOEXEC
        } else {
            flags & !FD_CLOEXEC
        };
        
        if libc_fcntl(fd, F_SETFD, new_flags) == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

/// Close a file descriptor
fn close_fd(fd: RawFd) -> Result<()> {
    unsafe {
        if libc::close(fd) == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

/// Safe wrapper for fcntl with various operations
pub fn fcntl(fd: RawFd, cmd: c_int, arg: c_int) -> Result<c_int> {
    unsafe {
        let res = libc_fcntl(fd, cmd, arg);
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::os::unix::io::AsRawFd;

    #[test]
    fn test_dupfd() {
        let file = File::open("/dev/null").unwrap();
        let fd = file.as_raw_fd();
        
        let new_fd = dupfd(fd, 10).unwrap();
        assert!(new_fd >= 10);
        
        let _ = close_fd(new_fd);
    }

    #[test]
    fn test_dupfd_cloexec() {
        let file = File::open("/dev/null").unwrap();
        let fd = file.as_raw_fd();
        
        let new_fd = dupfd_cloexec(fd, 20).unwrap();
        assert!(new_fd >= 20);
        
        // Verify FD_CLOEXEC is set
        let flags = fcntl(new_fd, F_GETFD, 0).unwrap();
        assert_eq!(flags & FD_CLOEXEC, FD_CLOEXEC);
        
        let _ = close_fd(new_fd);
    }
}