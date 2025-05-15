use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::io;

pub fn fd_safer_flag(fd: RawFd, flag: libc::c_int) -> io::Result<RawFd> {
    if fd <= 2 {
        let new_fd = unsafe { dup_safer_flag(fd, flag) };
        if new_fd == -1 {
            return Err(io::Error::last_os_error());
        }
        
        unsafe { libc::close(fd) };
        Ok(new_fd)
    } else {
        Ok(fd)
    }
}

extern "C" {
    fn dup_safer_flag(fd: libc::c_int, flag: libc::c_int) -> libc::c_int;
}