use std::os::unix::io::{FromRawFd, RawFd};
use std::process;
use std::io::{self, Error};

fn pipe_safer() -> Result<(RawFd, RawFd), Error> {
    let mut fds = [0 as RawFd; 2];
    
    unsafe {
        if libc::pipe(fds.as_mut_ptr()) != 0 {
            return Err(Error::last_os_error());
        }
    }

    let fd0 = unsafe { libc::fd_safer(fds[0]) };
    if fd0 < 0 {
        let saved_errno = io::Error::last_os_error();
        unsafe { libc::close(fds[1]) };
        return Err(Error::from_raw_os_error(saved_errno.raw_os_error().unwrap()));
    }

    let fd1 = unsafe { libc::fd_safer(fds[1]) };
    if fd1 < 0 {
        let saved_errno = io::Error::last_os_error();
        unsafe { libc::close(fd0) };
        return Err(Error::from_raw_os_error(saved_errno.raw_os_error().unwrap()));
    }

    Ok((fd0, fd1))
}