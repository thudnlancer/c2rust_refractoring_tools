//! Implementation of close-on-exec functionality for file descriptors
//!
//! This module provides functions to manipulate the close-on-exec flag
//! on file descriptors, following the same functionality as the C version.
//!
//! The code is free software under the GNU Lesser General Public License.

use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::io::{Result, Error, ErrorKind};

/// Set the 'FD_CLOEXEC' flag of `fd` if `value` is true,
/// or clear the flag if `value` is false.
/// Returns Ok(()) on success, or Err on error.
///
/// Note that on some platforms (like Windows), this function may not
/// protect the descriptor from being inherited into spawned children.
pub fn set_cloexec_flag(fd: RawFd, value: bool) -> Result<()> {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
    {
        let flags = libc::fcntl(fd, libc::F_GETFD);
        if flags < 0 {
            return Err(Error::last_os_error());
        }

        let new_flags = if value {
            flags | libc::FD_CLOEXEC
        } else {
            flags & !libc::FD_CLOEXEC
        };

        if flags != new_flags {
            if libc::fcntl(fd, libc::F_SETFD, new_flags) < 0 {
                return Err(Error::last_os_error());
            }
        }
        Ok(())
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
    {
        // Fallback implementation for platforms without F_SETFD
        if fd < 0 {
            return Err(Error::new(ErrorKind::InvalidInput, "invalid file descriptor"));
        }
        
        // Try to duplicate the descriptor to verify it's valid
        unsafe {
            if libc::dup2(fd, fd) < 0 {
                return Err(Error::last_os_error());
            }
        }
        
        // Can't actually set cloexec on this platform
        Ok(())
    }
}

/// Duplicates a file descriptor while marking the copy to be closed
/// prior to exec or spawn. Returns the new file descriptor or an error.
pub fn dup_cloexec(fd: RawFd) -> Result<RawFd> {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
    {
        let new_fd = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0) };
        if new_fd < 0 {
            Err(Error::last_os_error())
        } else {
            Ok(new_fd)
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
    {
        // Fallback implementation for platforms without F_DUPFD_CLOEXEC
        let new_fd = unsafe { libc::dup(fd) };
        if new_fd < 0 {
            return Err(Error::last_os_error());
        }
        
        if let Err(e) = set_cloexec_flag(new_fd, true) {
            unsafe { libc::close(new_fd); }
            return Err(e);
        }
        
        Ok(new_fd)
    }
}