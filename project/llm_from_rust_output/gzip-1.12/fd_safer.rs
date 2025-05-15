use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::io::{self, Error};

pub fn fd_safer(fd: RawFd) -> Result<RawFd, Error> {
    if (0..=2).contains(&fd) {
        let new_fd = unsafe { libc::dup(fd) };
        if new_fd == -1 {
            return Err(Error::last_os_error());
        }

        let e = io::Error::last_os_error();
        unsafe { libc::close(fd) };
        io::set_errno(e);

        Ok(new_fd)
    } else {
        Ok(fd)
    }
}

fn set_errno(e: Error) {
    unsafe {
        *libc::__errno_location() = e.raw_os_error().unwrap_or(0);
    }
}