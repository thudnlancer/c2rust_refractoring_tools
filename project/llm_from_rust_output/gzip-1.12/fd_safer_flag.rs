use std::os::unix::io::{RawFd, AsRawFd, FromRawFd};
use std::io::{self, Error};

fn fd_safer_flag(fd: RawFd, flag: i32) -> Result<RawFd, Error> {
    if (0..=2).contains(&fd) {
        let new_fd = unsafe { dup_safer_flag(fd, flag) };
        if new_fd == -1 {
            return Err(Error::last_os_error());
        }
        
        let saved_errno = Error::last_os_error();
        unsafe { close(fd) };
        io::set_errno(saved_errno);
        
        Ok(new_fd)
    } else {
        Ok(fd)
    }
}

// Helper functions to minimize unsafe blocks
unsafe fn dup_safer_flag(fd: RawFd, flag: i32) -> RawFd {
    // Implementation of dup_safer_flag would go here
    // This is just a placeholder since the original was extern
    libc::dup(fd)
}

unsafe fn close(fd: RawFd) {
    let _ = libc::close(fd);
}

// Helper to set errno similar to C's behavior
fn io::set_errno(error: Error) {
    unsafe { *libc::__errno_location() = error.raw_os_error().unwrap_or(0) };
}