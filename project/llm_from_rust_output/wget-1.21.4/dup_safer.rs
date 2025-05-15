use std::os::unix::io::{FromRawFd, RawFd};
use std::fs::File;

pub fn dup_safer(fd: RawFd) -> Result<RawFd, std::io::Error> {
    let new_fd = unsafe { libc::fcntl(fd, libc::F_DUPFD, libc::STDERR_FILENO + 1) };
    if new_fd == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(new_fd)
    }
}