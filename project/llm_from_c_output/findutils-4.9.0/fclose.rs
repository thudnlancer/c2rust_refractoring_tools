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
use std::os::unix::io::AsRawFd;
use std::sync::atomic::{AtomicBool, Ordering};

static HAVE_MSVC_INVALID_PARAMETER_HANDLER: AtomicBool = AtomicBool::new(false);

fn fclose_nothrow(file: File) -> io::Result<()> {
    if HAVE_MSVC_INVALID_PARAMETER_HANDLER.load(Ordering::Relaxed) {
        match file.sync_all() {
            Ok(_) => Ok(()),
            Err(e) => {
                if e.raw_os_error() == Some(libc::EBADF) {
                    Err(io::Error::from_raw_os_error(libc::EBADF))
                } else {
                    Err(e)
                }
            }
        }
    } else {
        file.sync_all()
    }
}

fn freading(file: &File) -> bool {
    // Placeholder implementation - actual implementation depends on platform specifics
    false
}

pub fn rpl_fclose(mut file: File) -> io::Result<()> {
    let fd = file.as_raw_fd();
    if fd < 0 {
        return fclose_nothrow(file);
    }

    let mut saved_errno = None;

    // Check if we need to flush
    let needs_flush = !freading(&file) || file.seek(SeekFrom::Current(0)).is_ok();
    if needs_flush {
        if let Err(e) = file.sync_all() {
            saved_errno = Some(e);
        }
    }

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::io::FromRawFd;
        unsafe {
            let raw_fd = file.as_raw_fd();
            let _ = libc::close(raw_fd);
            if saved_errno.is_none() && std::io::Error::last_os_error().raw_os_error().is_some() {
                saved_errno = Some(std::io::Error::last_os_error());
            }
        }
        fclose_nothrow(file)?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        let result = fclose_nothrow(file);
        #[cfg(feature = "replace_fchdir")]
        {
            if result.is_ok() {
                // _gl_unregister_fd(fd);
            }
        }
        if let Some(e) = saved_errno {
            return Err(e);
        }
        result?;
    }

    Ok(())
}