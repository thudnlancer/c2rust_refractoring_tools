/* This Rust code provides a safe wrapper around file descriptor operations,
   similar to the functionality in the original C fcntl.h implementation.
   It handles operations like duplicating file descriptors and managing flags. */

use libc::{c_int, F_DUPFD, F_DUPFD_CLOEXEC, F_GETFD, FD_CLOEXEC};
use std::os::unix::io::RawFd;
use std::io::{Error, Result};

/// Safe wrapper for fcntl F_DUPFD operation
pub fn fcntl_dupfd(fd: RawFd, target: RawFd) -> Result<RawFd> {
    unsafe {
        let res = libc::fcntl(fd, F_DUPFD, target);
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Safe wrapper for fcntl F_DUPFD_CLOEXEC operation
pub fn fcntl_dupfd_cloexec(fd: RawFd, target: RawFd) -> Result<RawFd> {
    unsafe {
        let res = libc::fcntl(fd, F_DUPFD_CLOEXEC, target);
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Safe wrapper for fcntl F_GETFD operation
pub fn fcntl_getfd(fd: RawFd) -> Result<c_int> {
    unsafe {
        let res = libc::fcntl(fd, F_GETFD);
        if res == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(res)
        }
    }
}

/// Safe wrapper for setting close-on-exec flag
pub fn set_cloexec(fd: RawFd) -> Result<()> {
    let flags = fcntl_getfd(fd)?;
    unsafe {
        if libc::fcntl(fd, F_GETFD, flags | FD_CLOEXEC) == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

/// File descriptor flags constants
pub mod flags {
    use libc::{
        O_ACCMODE, O_APPEND, O_ASYNC, O_CLOEXEC, O_CREAT, O_DIRECT, O_DIRECTORY,
        O_DSYNC, O_EXCL, O_EXEC, O_NOCTTY, O_NOFOLLOW, O_NONBLOCK, O_RDONLY,
        O_RDWR, O_RSYNC, O_SEARCH, O_SYNC, O_TRUNC, O_WRONLY,
    };

    pub const ACCMODE: c_int = O_ACCMODE;
    pub const APPEND: c_int = O_APPEND;
    pub const ASYNC: c_int = O_ASYNC;
    pub const CLOEXEC: c_int = O_CLOEXEC;
    pub const CREAT: c_int = O_CREAT;
    pub const DIRECT: c_int = O_DIRECT;
    pub const DIRECTORY: c_int = O_DIRECTORY;
    pub const DSYNC: c_int = O_DSYNC;
    pub const EXCL: c_int = O_EXCL;
    pub const EXEC: c_int = O_EXEC;
    pub const NOCTTY: c_int = O_NOCTTY;
    pub const NOFOLLOW: c_int = O_NOFOLLOW;
    pub const NONBLOCK: c_int = O_NONBLOCK;
    pub const RDONLY: c_int = O_RDONLY;
    pub const RDWR: c_int = O_RDWR;
    pub const RSYNC: c_int = O_RSYNC;
    pub const SEARCH: c_int = O_SEARCH;
    pub const SYNC: c_int = O_SYNC;
    pub const TRUNC: c_int = O_TRUNC;
    pub const WRONLY: c_int = O_WRONLY;
}

/// File descriptor control commands
pub mod commands {
    use libc::{
        F_DUPFD, F_DUPFD_CLOEXEC, F_GETFD, F_GETFL, F_GETLEASE, F_GETOWN,
        F_GETPIPE_SZ, F_GETSIG, F_SETFD, F_SETFL, F_SETLEASE, F_SETOWN,
        F_SETPIPE_SZ, F_SETSIG,
    };

    pub const DUPFD: c_int = F_DUPFD;
    pub const DUPFD_CLOEXEC: c_int = F_DUPFD_CLOEXEC;
    pub const GETFD: c_int = F_GETFD;
    pub const GETFL: c_int = F_GETFL;
    pub const GETLEASE: c_int = F_GETLEASE;
    pub const GETOWN: c_int = F_GETOWN;
    pub const GETPIPE_SZ: c_int = F_GETPIPE_SZ;
    pub const GETSIG: c_int = F_GETSIG;
    pub const SETFD: c_int = F_SETFD;
    pub const SETFL: c_int = F_SETFL;
    pub const SETLEASE: c_int = F_SETLEASE;
    pub const SETOWN: c_int = F_SETOWN;
    pub const SETPIPE_SZ: c_int = F_SETPIPE_SZ;
    pub const SETSIG: c_int = F_SETSIG;
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
        let new_fd = fcntl_dupfd(fd, 10).unwrap();
        assert!(new_fd >= 10);
        unsafe { libc::close(new_fd) };
    }

    #[test]
    fn test_getfd() {
        let file = File::open("/dev/null").unwrap();
        let fd = file.as_raw_fd();
        let flags = fcntl_getfd(fd).unwrap();
        assert!(flags >= 0);
    }
}