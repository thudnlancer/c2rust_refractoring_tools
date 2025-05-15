use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::io::{self, Error};

pub fn fd_safer(fd: RawFd) -> Result<RawFd, Error> {
    if (0..=2).contains(&fd) {
        let new_fd = unsafe { libc::dup(fd) };
        if new_fd == -1 {
            return Err(Error::last_os_error());
        }

        let e = Error::last_os_error();
        if unsafe { libc::close(fd) } == -1 {
            let _ = unsafe { libc::close(new_fd) }; // Clean up new_fd on failure
            return Err(Error::last_os_error());
        }

        // Restore the original errno
        unsafe { *libc::__errno_location() = e.raw_os_error().unwrap_or(0) };
        Ok(new_fd)
    } else {
        Ok(fd)
    }
}