use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::fs::File;

fn dup_safer_flag(fd: RawFd, flag: i32) -> std::io::Result<RawFd> {
    let cmd = if flag & 0o2000000 != 0 {
        libc::F_DUPFD_CLOEXEC
    } else {
        libc::F_DUPFD
    };
    
    let new_fd = unsafe { libc::fcntl(fd, cmd, 3) };
    if new_fd == -1 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(new_fd)
    }
}