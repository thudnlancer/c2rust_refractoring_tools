use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::io;

pub fn fd_safer(fd: RawFd) -> io::Result<RawFd> {
    if (0..=2).contains(&fd) {
        let new_fd = unsafe { libc::dup(fd) };
        if new_fd == -1 {
            return Err(io::Error::last_os_error());
        }

        let res = unsafe { libc::close(fd) };
        if res == -1 {
            let e = io::Error::last_os_error();
            unsafe { libc::close(new_fd) };
            return Err(e);
        }

        Ok(new_fd)
    } else {
        Ok(fd)
    }
}