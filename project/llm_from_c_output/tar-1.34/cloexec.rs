//! Set or clear the close-on-exec descriptor flag
//!
//! This code is translated from C to Rust while maintaining the same functionality.
//! The original C code is from the GNU project and is licensed under GPLv3+.

use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::io::{Result, Error, ErrorKind};

/// Set the 'FD_CLOEXEC' flag of DESC if VALUE is true,
/// or clear the flag if VALUE is false.
/// Returns Ok(()) on success, or Err on error.
///
/// Note that on Windows (MingW), this function does NOT protect DESC from being
/// inherited into spawned children. Instead, either use dup_cloexec
/// followed by closing the original DESC, or use interfaces such as
/// open or pipe2 that accept flags like O_CLOEXEC to create DESC
/// non-inheritable in the first place.
pub fn set_cloexec_flag(desc: RawFd, value: bool) -> Result<()> {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "unix"))] {
        use libc::{fcntl, F_GETFD, F_SETFD, FD_CLOEXEC};
        
        let flags = unsafe { fcntl(desc, F_GETFD) };
        if flags >= 0 {
            let newflags = if value {
                flags | FD_CLOEXEC
            } else {
                flags & !FD_CLOEXEC
            };
            
            if flags == newflags || unsafe { fcntl(desc, F_SETFD, newflags) } != -1 {
                return Ok(());
            }
        }
        Err(Error::last_os_error())
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "unix")))] {
        // Use dup2 to reject invalid file descriptors; the cloexec flag
        // will be unaffected.
        if desc < 0 {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid file descriptor"));
        }
        
        unsafe {
            if libc::dup2(desc, desc) < 0 {
                return Err(Error::last_os_error());
            }
        }
        
        // There is nothing we can do on this kind of platform.
        Ok(())
    }
}

/// Duplicates a file handle FD, while marking the copy to be closed
/// prior to exec or spawn. Returns the new file descriptor on success,
/// or Err if FD could not be duplicated.
pub fn dup_cloexec(fd: RawFd) -> Result<RawFd> {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "unix"))] {
        use libc::{fcntl, F_DUPFD_CLOEXEC};
        
        let new_fd = unsafe { fcntl(fd, F_DUPFD_CLOEXEC, 0) };
        if new_fd == -1 {
            Err(Error::last_os_error())
        } else {
            Ok(new_fd)
        }
    }
    
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "unix")))] {
        // Fallback implementation for platforms without F_DUPFD_CLOEXEC
        let new_fd = unsafe { libc::dup(fd) };
        if new_fd == -1 {
            return Err(Error::last_os_error());
        }
        
        if let Err(e) = set_cloexec_flag(new_fd, true) {
            unsafe { libc::close(new_fd); }
            return Err(e);
        }
        
        Ok(new_fd)
    }
}