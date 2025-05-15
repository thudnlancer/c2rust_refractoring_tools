// cloexec.rs - set or clear the close-on-exec descriptor flag

//   Copyright (C) 2004, 2009-2022 Free Software Foundation, Inc.

//   This file is free software: you can redistribute it and/or modify
//   it under the terms of the GNU Lesser General Public License as
//   published by the Free Software Foundation; either version 2.1 of the
//   License, or (at your option) any later version.

//   This file is distributed in the hope that it will be useful,
//   but WITHOUT ANY WARRANTY; without even the implied warranty of
//   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//   GNU Lesser General Public License for more details.

//   You should have received a copy of the GNU Lesser General Public License
//   along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::os::unix::io::{AsRawFd, RawFd};
use std::io;

/// Set the 'FD_CLOEXEC' flag of DESC if VALUE is true,
/// or clear the flag if VALUE is false.
/// Return Ok(()) on success, or Err with io::Error on failure.
///
/// Note that on MingW, this function does NOT protect DESC from being
/// inherited into spawned children.  Instead, either use dup_cloexec
/// followed by closing the original DESC, or use interfaces such as
/// open or pipe2 that accept flags like O_CLOEXEC to create DESC
/// non-inheritable in the first place.
pub fn set_cloexec_flag(desc: RawFd, value: bool) -> io::Result<()> {
    #[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
    {
        let flags = unsafe { libc::fcntl(desc, libc::F_GETFD) };
        if flags < 0 {
            return Err(io::Error::last_os_error());
        }

        let new_flags = if value {
            flags | libc::FD_CLOEXEC
        } else {
            flags & !libc::FD_CLOEXEC
        };

        if flags != new_flags {
            let result = unsafe { libc::fcntl(desc, libc::F_SETFD, new_flags) };
            if result < 0 {
                return Err(io::Error::last_os_error());
            }
        }

        Ok(())
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
    {
        if desc < 0 {
            return Err(io::Error::from_raw_os_error(libc::EBADF));
        }

        let result = unsafe { libc::dup2(desc, desc) };
        if result < 0 {
            return Err(io::Error::last_os_error());
        }

        Ok(())
    }
}

/// Duplicates a file handle FD, while marking the copy to be closed
/// prior to exec or spawn. Returns Err with io::Error if FD could not
/// be duplicated.
pub fn dup_cloexec(fd: RawFd) -> io::Result<RawFd> {
    let new_fd = unsafe { libc::fcntl(fd, libc::F_DUPFD_CLOEXEC, 0) };
    if new_fd < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(new_fd)
    }
}