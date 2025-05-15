/* fclose replacement.
   Copyright (C) 2008-2022 Free Software Foundation, Inc.

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

use std::fs::File;
use std::io::{self, Seek, SeekFrom};
use std::os::unix::io::{AsRawFd, FromRawFd, RawFd};
use std::mem;

fn freading(file: &File) -> bool {
    // Placeholder implementation - actual implementation would depend on platform specifics
    false
}

#[cfg(target_os = "windows")]
mod windows {
    use super::*;
    use std::os::windows::io::{AsRawHandle, FromRawHandle, RawHandle};

    pub fn close(fd: RawFd) -> io::Result<()> {
        // Windows-specific close implementation
        unsafe {
            let handle = fd as RawHandle;
            if winapi::um::handleapi::CloseHandle(handle) == 0 {
                Err(io::Error::last_os_error())
            } else {
                Ok(())
            }
        }
    }
}

#[cfg(not(target_os = "windows"))]
fn close(fd: RawFd) -> io::Result<()> {
    unsafe {
        if libc::close(fd) == -1 {
            Err(io::Error::last_os_error())
        } else {
            Ok(())
        }
    }
}

fn fclose_nothrow(file: File) -> io::Result<()> {
    // On Windows, we might need special handling here
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::io::IntoRawHandle;
        unsafe {
            let handle = file.into_raw_handle();
            if winapi::um::handleapi::CloseHandle(handle) == 0 {
                return Err(io::Error::last_os_error());
            }
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        drop(file); // Rust's drop will handle the closing on Unix-like systems
    }
    
    Ok(())
}

pub fn rpl_fclose(file: File) -> io::Result<()> {
    let fd = file.as_raw_fd();
    if fd < 0 {
        return fclose_nothrow(file);
    }

    let mut saved_errno = None;

    // Check if we need to flush
    if (!freading(&file) || file.seek(SeekFrom::Current(0)).is_ok()) {
        if let Err(e) = file.sync_all() {
            saved_errno = Some(e);
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Err(e) = windows::close(fd) {
            if saved_errno.is_none() {
                saved_errno = Some(e);
            }
        }

        if let Err(e) = fclose_nothrow(file) {
            if saved_errno.is_none() {
                saved_errno = Some(e);
            }
        }
    }

    #[cfg(not(target_os = "windows"))]
    {
        // On Unix-like systems, closing the file will automatically close the fd
        if let Err(e) = fclose_nothrow(file) {
            if saved_errno.is_none() {
                saved_errno = Some(e);
            }
        }
    }

    if let Some(e) = saved_errno {
        Err(e)
    } else {
        Ok(())
    }
}