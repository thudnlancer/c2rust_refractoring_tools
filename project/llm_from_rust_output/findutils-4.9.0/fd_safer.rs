use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::io::{self, Error};

pub fn fd_safer(fd: RawFd) -> Result<RawFd, Error> {
    if fd >= 0 && fd <= 2 {
        let f = unsafe { libc::dup(fd) };
        if f == -1 {
            return Err(Error::last_os_error());
        }

        let e = Error::last_os_error();
        unsafe { libc::close(fd) };
        io::set_errno(e);

        Ok(f)
    } else {
        Ok(fd)
    }
}

// Helper function to set errno
fn set_errno(e: Error) {
    unsafe {
        *libc::__errno_location() = e.raw_os_error().unwrap_or(0);
    }
}