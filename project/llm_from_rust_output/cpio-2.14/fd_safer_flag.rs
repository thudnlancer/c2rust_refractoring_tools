use std::os::unix::io::{RawFd, FromRawFd, IntoRawFd};
use std::io;

pub fn fd_safer_flag(fd: RawFd, flag: i32) -> io::Result<RawFd> {
    if (0..=2).contains(&fd) {
        let f = unsafe { dup_safer_flag(fd, flag) };
        if f == -1 {
            return Err(io::Error::last_os_error());
        }
        
        let e = io::Error::last_os_error();
        unsafe {
            let _ = libc::close(fd);
        }
        io::Error::set_last_os_error(e.raw_os_error().unwrap_or(0));
        Ok(f)
    } else {
        Ok(fd)
    }
}

// This would need to be implemented separately as a safe wrapper
// around the unsafe C function
fn dup_safer_flag(fd: RawFd, flag: i32) -> RawFd {
    unsafe { libc::dup2(fd, flag) }
}