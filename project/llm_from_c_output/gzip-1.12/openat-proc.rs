/* Create /proc/self/fd-related names for subfiles of open directories.

   Copyright (C) 2006, 2009-2022 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Paul Eggert.  */

use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io;
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicI32, Ordering};

const OPENAT_BUFFER_SIZE: usize = 1024; // Adjust as needed
const PROC_SELF_FD_FORMAT: &str = "/proc/self/fd/";

static PROC_STATUS: AtomicI32 = AtomicI32::new(0);

/// Set `buf` to the name of the subfile of the directory identified by
/// `fd`, where the subfile is named `file`. If successful, return `Ok(Some(buf))` if the result fits in `buf`,
/// or `Ok(Some(allocated_string))` if dynamically allocated memory was needed.
/// Return `Ok(None)` when the input file is empty (matching C behavior).
/// Return `Err` on error.
pub fn openat_proc_name(buf: &mut [u8; OPENAT_BUFFER_SIZE], fd: i32, file: &str) -> Result<Option<Box<[u8]>>, io::Error> {
    // Make sure the caller gets ENOENT when appropriate.
    if file.is_empty() {
        buf[0] = b'\0';
        return Ok(None);
    }

    #[cfg(not(target_os = "os2"))]
    {
        let proc_status = PROC_STATUS.load(Ordering::Relaxed);
        if proc_status == 0 {
            // Set PROC_STATUS to a positive value if /proc/self/fd is reliable,
            // and a negative value otherwise.
            let proc_self_fd = OpenOptions::new()
                .read(true)
                .custom_flags(libc::O_DIRECTORY | libc::O_NOCTTY | libc::O_NONBLOCK | libc::O_CLOEXEC)
                .open("/proc/self/fd")?;

            // Detect whether /proc/self/fd/%i/../fd exists
            let dotdot_path = format!("{}{}/../fd", PROC_SELF_FD_FORMAT, proc_self_fd.as_raw_fd());
            let new_status = if Path::new(&dotdot_path).exists() { 1 } else { -1 };
            PROC_STATUS.store(new_status, Ordering::Relaxed);

            if new_status < 0 {
                return Err(io::Error::new(io::ErrorKind::Other, "/proc/self/fd not reliable"));
            }
        } else if proc_status < 0 {
            return Err(io::Error::new(io::ErrorKind::Other, "/proc/self/fd not reliable"));
        }

        let dir_part = format!("{}{}/", PROC_SELF_FD_FORMAT, fd);
        let full_path = format!("{}{}", dir_part, file);

        if full_path.len() < OPENAT_BUFFER_SIZE {
            buf[..full_path.len()].copy_from_slice(full_path.as_bytes());
            buf[full_path.len()] = b'\0';
            Ok(Some(Box::new(buf[..=full_path.len()])))
        } else {
            Ok(Some(full_path.into_bytes().into_boxed_slice()))
        }
    }

    #[cfg(target_os = "os2")]
    {
        // OS/2 kLIBC implementation would go here
        // This is left unimplemented as Rust doesn't have direct kLIBC support
        Err(io::Error::new(io::ErrorKind::Other, "OS/2 kLIBC not supported in Rust"))
    }
}