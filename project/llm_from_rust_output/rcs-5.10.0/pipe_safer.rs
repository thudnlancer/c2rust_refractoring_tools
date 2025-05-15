use std::os::unix::io::{FromRawFd, RawFd};
use std::process;
use nix::errno::errno;
use nix::unistd::{pipe, close};

pub fn pipe_safer() -> Result<(RawFd, RawFd), i32> {
    match pipe() {
        Ok((read_fd, write_fd)) => {
            let safer_read = unsafe { libc::fd_safer(read_fd) };
            if safer_read < 0 {
                let saved_errno = errno();
                let _ = close(write_fd);
                return Err(saved_errno as i32);
            }

            let safer_write = unsafe { libc::fd_safer(write_fd) };
            if safer_write < 0 {
                let saved_errno = errno();
                let _ = close(safer_read);
                return Err(saved_errno as i32);
            }

            Ok((safer_read, safer_write))
        }
        Err(_) => Err(-1),
    }
}