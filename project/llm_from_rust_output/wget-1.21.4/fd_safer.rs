use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::io::{self, DupError};

fn fd_safer(fd: RawFd) -> Result<RawFd, DupError> {
    if (0..=2).contains(&fd) {
        let new_fd = unsafe { libc::dup(fd) };
        if new_fd == -1 {
            return Err(DupError::last_os_error());
        }
        
        let result = unsafe { libc::close(fd) };
        if result == -1 {
            unsafe { libc::close(new_fd) };
            return Err(DupError::last_os_error());
        }
        
        Ok(new_fd)
    } else {
        Ok(fd)
    }
}