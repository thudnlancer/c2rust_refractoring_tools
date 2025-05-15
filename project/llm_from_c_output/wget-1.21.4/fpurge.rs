/* Flushing buffers of a FILE stream.
   Copyright (C) 2007-2023 Free Software Foundation, Inc.

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

pub fn fpurge(stream: &mut impl Write + Read + Seek) -> io::Result<()> {
    // On Unix-like systems, we can use libc's __fpurge or fpurge if available
    #[cfg(any(
        target_os = "linux",
        target_os = "android",
        target_os = "solaris",
        target_os = "haiku",
        target_os = "cygwin"
    ))]
    {
        #[cfg(feature = "libc")]
        {
            extern crate libc;
            unsafe {
                libc::__fpurge(stream as *mut _ as *mut libc::FILE);
            }
            return Ok(());
        }
    }

    #[cfg(any(
        target_os = "freebsd",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "dragonfly",
        target_os = "macos"
    ))]
    {
        #[cfg(feature = "libc")]
        {
            extern crate libc;
            let result = unsafe { libc::fpurge(stream as *mut _ as *mut libc::FILE) };
            if result != 0 {
                return Err(io::Error::last_os_error());
            }
            return Ok(());
        }
    }

    // For other platforms, we implement the flushing behavior manually
    stream.flush()?;
    
    // For read buffers, we need to seek to current position to clear the buffer
    let pos = stream.stream_position()?;
    stream.seek(io::SeekFrom::Current(0))?;
    stream.seek(io::SeekFrom::Start(pos))?;

    Ok(())
}