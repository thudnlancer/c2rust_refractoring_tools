//! Implementation of close-on-exec functionality for file descriptors

use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::io;

/// Set the 'FD_CLOEXEC' flag of `desc` if `value` is true,
/// or clear the flag if `value` is false.
/// Returns Ok(()) on success, or Err(io::Error) on error.
///
/// Note that on Windows (not applicable in Rust's Unix module), this function does NOT protect `desc` from being
/// inherited into spawned children. Instead, either use `dup_cloexec`
/// followed by closing the original `desc`, or use interfaces such as
/// `open` or `pipe2` that accept flags like O_CLOEXEC to create `desc`
/// non-inheritable in the first place.
pub fn set_cloexec_flag(desc: RawFd, value: bool) -> io::Result<()> {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
    {
        let flags = libc::fcntl(desc, libc::F_GETFD);
        if flags >= 0 {
            let new_flags = if value {
                flags | libc::FD_CLOEXEC
            } else {
                flags & !libc::FD_CLOEXEC
            };

            if flags == new_flags || libc::fcntl(desc, libc::F_SETFD, new_flags) != -1 {
                return Ok(());
            }
        }
        Err(io::Error::last_os_error())
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
    {
        // Use dup2 to reject invalid file descriptors; the cloexec flag
        // will be unaffected.
        if desc < 0 {
            return Err(io::Error::from_raw_os_error(libc::EBADF));
        }
        if unsafe { libc::dup2(desc, desc) } < 0 {
            return Err(io::Error::last_os_error());
        }
        // There is nothing we can do on this kind of platform.
        Ok(())
    }
}

/// Duplicates a file descriptor `fd`, while marking the copy to be closed
/// prior to exec or spawn. Returns the new file descriptor or an error.
pub fn dup_cloexec(fd: RawFd) -> io::Result<RawFd> {
    let new_fd = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0) };
    if new_fd == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(new_fd)
    }
}