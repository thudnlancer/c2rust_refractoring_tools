// xattr.rs -- POSIX Extended Attribute support.

// Copyright (C) 2016, 2018-2023 Free Software Foundation, Inc.

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3, or (at your option)
// any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program; if not, see <http://www.gnu.org/licenses/>.

use std::ffi::CString;
use std::fs::File;
use std::io;
use std::os::unix::io::AsRawFd;
use std::path::Path;
use url::Url;

#[cfg(any(target_os = "linux", target_os = "macos", target_os = "freebsd"))]
mod xattr_impl {
    use super::*;
    use std::ffi::CString;
    use std::os::raw::c_int;

    #[cfg(target_os = "linux")]
    pub fn write_xattr_metadata(name: &str, value: &str, file: &File) -> io::Result<()> {
        use libc::{fsetxattr, strlen};

        let fd = file.as_raw_fd();
        let name_c = CString::new(name)?;
        let value_c = CString::new(value)?;

        unsafe {
            let res = fsetxattr(
                fd,
                name_c.as_ptr(),
                value_c.as_ptr() as *const _,
                strlen(value_c.as_ptr()),
                0,
            );
            if res == 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }

    #[cfg(target_os = "macos")]
    pub fn write_xattr_metadata(name: &str, value: &str, file: &File) -> io::Result<()> {
        use libc::{fsetxattr, strlen};

        let fd = file.as_raw_fd();
        let name_c = CString::new(name)?;
        let value_c = CString::new(value)?;

        unsafe {
            let res = fsetxattr(
                fd,
                name_c.as_ptr(),
                value_c.as_ptr() as *const _,
                strlen(value_c.as_ptr()),
                0,
                0,
            );
            if res == 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }

    #[cfg(target_os = "freebsd")]
    pub fn write_xattr_metadata(name: &str, value: &str, file: &File) -> io::Result<()> {
        use libc::{extattr_set_fd, strlen, EXTATTR_NAMESPACE_USER};

        let fd = file.as_raw_fd();
        let name_c = CString::new(name)?;
        let value_c = CString::new(value)?;

        unsafe {
            let res = extattr_set_fd(
                fd,
                EXTATTR_NAMESPACE_USER,
                name_c.as_ptr(),
                value_c.as_ptr() as *const _,
                strlen(value_c.as_ptr()),
            );
            if res >= 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }
}

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "freebsd")))]
mod xattr_impl {
    use super::*;

    pub fn write_xattr_metadata(_name: &str, _value: &str, _file: &File) -> io::Result<()> {
        Ok(())
    }
}

pub fn set_file_metadata(
    origin_url: &Url,
    referrer_url: Option<&Url>,
    file: &File,
) -> io::Result<()> {
    /* Save metadata about where the file came from (requested, final URLs) to
     * user POSIX Extended Attributes of retrieved file.
     *
     * For more details about the user namespace see
     * [http://freedesktop.org/wiki/CommonExtendedAttributes] and
     * [http://0pointer.de/lennart/projects/mod_mime_xattr/].
     */

    let origin_value = origin_url.to_string();
    xattr_impl::write_xattr_metadata("user.xdg.origin.url", &origin_value, file)?;

    if let Some(referrer) = referrer_url {
        let mut ref_url = referrer.clone();
        ref_url.set_path("");
        ref_url.set_query(None);
        ref_url.set_fragment(None);

        let referrer_value = ref_url.to_string();
        xattr_impl::write_xattr_metadata("user.xdg.referrer.url", &referrer_value, file)?;
    }

    Ok(())
}