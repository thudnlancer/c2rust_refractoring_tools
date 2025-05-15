/* Flushing buffers of a FILE stream.
   Copyright (C) 2007-2022 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::io::{self, Write, Read, Seek};
use std::fs::File;

/// Flushes the buffers of a file stream, equivalent to C's fpurge
pub fn fpurge(file: &mut File) -> io::Result<()> {
    // On Unix-like systems, we can use libc's __fpurge or fpurge if available
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "solaris",
        target_os = "haiku",
        target_os = "cygwin"
    ))]
    {
        // Try to use libc's __fpurge if available
        #[cfg(feature = "libc")]
        {
            extern crate libc;
            unsafe {
                libc::__fpurge(file.as_raw_fd() as *mut libc::FILE);
            }
            return Ok(());
        }
    }

    // For BSD systems (FreeBSD, NetBSD, OpenBSD, DragonFly, MacOS)
    #[cfg(any(
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "macos"
    ))]
    {
        // Try to use libc's fpurge if available
        #[cfg(feature = "libc")]
        {
            extern crate libc;
            let result = unsafe { libc::fpurge(file.as_raw_fd() as *mut libc::FILE) };
            if result == 0 {
                // Additional BSD-specific buffer management would go here
                return Ok(());
            } else {
                return Err(io::Error::last_os_error());
            }
        }
    }

    // For other systems, implement the flush behavior in Rust
    // This is a simplified version that just flushes and seeks to current position
    file.flush()?;
    let pos = file.stream_position()?;
    file.seek(io::SeekFrom::Start(pos))?;

    Ok(())
}