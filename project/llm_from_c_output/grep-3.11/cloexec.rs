//! Set or clear the close-on-exec descriptor flag
//!
//! This file is free software: you can redistribute it and/or modify
//! it under the terms of the GNU Lesser General Public License as
//! published by the Free Software Foundation; either version 2.1 of the
//! License, or (at your option) any later version.
//!
//! This file is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU Lesser General Public License for more details.
//!
//! You should have received a copy of the GNU Lesser General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::os::unix::io::{AsRawFd, RawFd};
use std::io::{Result, Error, ErrorKind};

/// Set the 'FD_CLOEXEC' flag of DESC if VALUE is true,
/// or clear the flag if VALUE is false.
/// Return Ok(()) on success, or Err with error information.
///
/// Note that on MingW, this function does NOT protect DESC from being
/// inherited into spawned children.  Instead, either use dup_cloexec
/// followed by closing the original DESC, or use interfaces such as
/// open or pipe2 that accept flags like O_CLOEXEC to create DESC
/// non-inheritable in the first place.
pub fn set_cloexec_flag(desc: RawFd, value: bool) -> Result<()> {
    #[cfg(target_os = "linux")]
    {
        let flags = libc::fcntl(desc, libc::F_GETFD);
        if flags >= 0 {
            let newflags = if value {
                flags | libc::FD_CLOEXEC
            } else {
                flags & !libc::FD_CLOEXEC
            };

            if flags == newflags || libc::fcntl(desc, libc::F_SETFD, newflags) != -1 {
                return Ok(());
            }
        }
        Err(Error::last_os_error())
    }

    #[cfg(not(target_os = "linux"))]
    {
        // Use dup2 to reject invalid file descriptors; the cloexec flag
        // will be unaffected.
        if desc < 0 {
            return Err(Error::from_raw_os_error(libc::EBADF));
        }
        if unsafe { libc::dup2(desc, desc) } < 0 {
            // errno is EBADF here.
            return Err(Error::last_os_error());
        }

        // There is nothing we can do on this kind of platform. Punt.
        Ok(())
    }
}

/// Duplicates a file handle FD, while marking the copy to be closed
/// prior to exec or spawn. Returns the new file descriptor or an error.
pub fn dup_cloexec(fd: RawFd) -> Result<RawFd> {
    let new_fd = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0) };
    if new_fd == -1 {
        Err(Error::last_os_error())
    } else {
        Ok(new_fd)
    }
}