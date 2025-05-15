/* Test whether a file has a nontrivial ACL.  -*- coding: utf-8 -*-

   Copyright (C) 2002-2003, 2005-2021 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.

   Written by Paul Eggert, Andreas Grünbacher, and Bruno Haible.  */

use std::fs::Metadata;
use std::os::unix::fs::MetadataExt;
use std::path::Path;
use std::io;

#[cfg(target_os = "linux")]
use std::os::linux::fs::MetadataExt as LinuxMetadataExt;

#[cfg(any(target_os = "freebsd", target_os = "macos", target_os = "ios"))]
use std::os::unix::fs::PermissionsExt;

pub fn file_has_acl(name: &Path, metadata: &Metadata) -> io::Result<bool> {
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        if metadata.is_symlink() {
            return Ok(false);
        }

        use std::os::linux::fs::MetadataExt as LinuxMetadataExt;
        use std::ffi::CString;
        use libc::{getxattr, XATTR_NAME_POSIX_ACL_ACCESS, XATTR_NAME_POSIX_ACL_DEFAULT};

        let c_name = CString::new(name.as_os_str().as_bytes()).unwrap();

        unsafe {
            let mut ret = getxattr(
                c_name.as_ptr(),
                XATTR_NAME_POSIX_ACL_ACCESS.as_ptr(),
                std::ptr::null_mut(),
                0,
            );

            if ret < 0 {
                let err = io::Error::last_os_error();
                if err.raw_os_error() == Some(libc::ENODATA) {
                    ret = 0;
                } else {
                    return Err(err);
                }
            } else if ret > 0 {
                return Ok(true);
            }

            if ret == 0 && metadata.is_dir() {
                ret = getxattr(
                    c_name.as_ptr(),
                    XATTR_NAME_POSIX_ACL_DEFAULT.as_ptr(),
                    std::ptr::null_mut(),
                    0,
                );

                if ret < 0 {
                    let err = io::Error::last_os_error();
                    if err.raw_os_error() == Some(libc::ENODATA) {
                        ret = 0;
                    } else {
                        return Err(err);
                    }
                } else if ret > 0 {
                    return Ok(true);
                }
            }

            Ok(ret > 0)
        }
    }

    #[cfg(not(any(target_os = "linux", target_os = "android")))]
    {
        // For other platforms, we currently don't have ACL support
        Ok(false)
    }
}