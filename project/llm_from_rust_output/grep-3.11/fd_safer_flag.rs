use std::os::unix::io::{RawFd, FromRawFd, IntoRawFd};
use std::io::{self, Error};

pub fn fd_safer_flag(fd: RawFd, flag: i32) -> Result<RawFd, Error> {
    if fd >= 0 && fd <= 2 {
        let f = unsafe { dup_safer_flag(fd, flag) };
        if f == -1 {
            return Err(Error::last_os_error());
        }
        
        let e = io::Error::last_os_error();
        unsafe { close(fd) };
        io::set_errno(e);
        Ok(f)
    } else {
        Ok(fd)
    }
}

// These would need to be implemented safely or wrapped from libc
extern "C" {
    fn dup_safer_flag(fd: libc::c_int, flag: libc::c_int) -> libc::c_int;
    fn close(fd: libc::c_int) -> libc::c_int;
}

// Helper to set errno in a more Rust-idiomatic way
fn io::set_errno(e: io::Error) {
    unsafe { *libc::__errno_location() = e.raw_os_error().unwrap_or(0) };
}