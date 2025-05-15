use std::os::unix::io::{RawFd, FromRawFd, IntoRawFd};
use std::io::{self, DupError};

fn fd_safer(fd: RawFd) -> Result<RawFd, DupError> {
    if fd >= 0 && fd <= 2 {
        let new_fd = unsafe { libc::dup(fd) };
        if new_fd == -1 {
            return Err(io::Error::last_os_error().into());
        }
        
        let result = unsafe { libc::close(fd) };
        if result == -1 {
            let e = io::Error::last_os_error();
            unsafe { libc::close(new_fd) };
            return Err(e.into());
        }
        
        Ok(new_fd)
    } else {
        Ok(fd)
    }
}