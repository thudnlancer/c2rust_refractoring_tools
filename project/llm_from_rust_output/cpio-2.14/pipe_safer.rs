use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::process;

fn pipe_safer() -> Result<(RawFd, RawFd), i32> {
    let mut fds = [0 as RawFd; 2];
    
    unsafe {
        if libc::pipe(fds.as_mut_ptr()) != 0 {
            return Err(-1);
        }
    }

    for i in 0..2 {
        let safer_fd = unsafe { libc::fd_safer(fds[i]) };
        if safer_fd < 0 {
            let saved_errno = unsafe { *libc::__errno_location() };
            unsafe { libc::close(fds[1 - i]) };
            unsafe { *libc::__errno_location() = saved_errno };
            return Err(-1);
        }
        fds[i] = safer_fd;
    }

    Ok((fds[0], fds[1]))
}