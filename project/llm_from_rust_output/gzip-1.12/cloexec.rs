use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::fs::File;

const F_GETFD: libc::c_int = 1;
const F_SETFD: libc::c_int = 2;
const F_DUPFD_CLOEXEC: libc::c_int = 1030;
const FD_CLOEXEC: libc::c_int = 1;

pub fn set_cloexec_flag(desc: RawFd, value: bool) -> Result<(), std::io::Error> {
    let flags = unsafe { libc::fcntl(desc, F_GETFD) };
    if flags >= 0 {
        let newflags = if value {
            flags | FD_CLOEXEC
        } else {
            flags & !FD_CLOEXEC
        };
        if flags == newflags || unsafe { libc::fcntl(desc, F_SETFD, newflags) } != -1 {
            return Ok(());
        }
    }
    Err(std::io::Error::last_os_error())
}

pub fn dup_cloexec(fd: RawFd) -> Result<File, std::io::Error> {
    let new_fd = unsafe { libc::fcntl(fd, F_DUPFD_CLOEXEC, 0) };
    if new_fd == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(unsafe { File::from_raw_fd(new_fd) })
    }
}