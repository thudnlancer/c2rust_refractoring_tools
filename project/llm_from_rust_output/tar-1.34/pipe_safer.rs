use std::os::unix::io::{FromRawFd, IntoRawFd, RawFd};
use std::process;
use std::io::{self, Error};

fn pipe_safer() -> Result<(RawFd, RawFd), Error> {
    let mut fds = [0; 2];
    
    unsafe {
        if libc::pipe(fds.as_mut_ptr()) != 0 {
            return Err(Error::last_os_error());
        }
    }

    let r = unsafe { libc::fd_safer(fds[0]) };
    let w = unsafe { libc::fd_safer(fds[1]) };

    if r < 0 || w < 0 {
        let saved_errno = io::Error::last_os_error();
        unsafe {
            if r >= 0 {
                libc::close(r);
            }
            if w >= 0 {
                libc::close(w);
            }
        }
        return Err(Error::from_raw_os_error(saved_errno.raw_os_error().unwrap()));
    }

    Ok((r, w))
}