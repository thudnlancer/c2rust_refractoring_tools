use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::io;

pub fn fd_safer_flag(fd: RawFd, flag: libc::c_int) -> io::Result<RawFd> {
    if (0..=2).contains(&fd) {
        let f = unsafe { dup_safer_flag(fd, flag) };
        if f == -1 {
            return Err(io::Error::last_os_error());
        }
        
        let e = io::Error::last_os_error();
        unsafe { libc::close(fd) };
        io::Error::set_last_os_error(e.raw_os_error().unwrap_or(0));
        
        Ok(f)
    } else {
        Ok(fd)
    }
}

extern "C" {
    fn dup_safer_flag(fd: libc::c_int, flag: libc::c_int) -> libc::c_int;
}