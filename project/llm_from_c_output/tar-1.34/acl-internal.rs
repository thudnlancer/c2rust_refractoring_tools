/*!
Internal implementation of access control lists.

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

Written by Paul Eggert, Andreas Grünbacher, and Bruno Haible.
*/

use libc::{mode_t, c_int};
use std::ptr;
use std::os::unix::ffi::OsStrExt;
use std::ffi::CStr;
use std::path::Path;

#[cfg(target_os = "linux")]
use libc::{acl_t, acl_entry_t, acl_tag_t, ACL_FIRST_ENTRY, ACL_NEXT_ENTRY, ACL_USER_OBJ, ACL_GROUP_OBJ, ACL_OTHER};

#[derive(Debug)]
pub struct PermissionContext {
    pub mode: mode_t,
    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "cygwin"
    ))]
    pub acl: Option<acl_t>,
    #[cfg(not(any(target_os = "macos", target_os = "cygwin")))]
    pub default_acl: Option<acl_t>,
    pub acls_not_supported: bool,
}

impl Default for PermissionContext {
    fn default() -> Self {
        Self {
            mode: 0,
            #[cfg(any(
                target_os = "linux",
                target_os = "freebsd",
                target_os = "macos",
                target_os = "cygwin"
            ))]
            acl: None,
            #[cfg(not(any(target_os = "macos", target_os = "cygwin")))]
            default_acl: None,
            acls_not_supported: false,
        }
    }
}

impl Drop for PermissionContext {
    fn drop(&mut self) {
        #[cfg(any(
            target_os = "linux",
            target_os = "freebsd",
            target_os = "macos",
            target_os = "cygwin"
        ))]
        if let Some(acl) = self.acl {
            unsafe { libc::acl_free(acl as *mut _) };
        }
        #[cfg(not(any(target_os = "macos", target_os = "cygwin")))]
        if let Some(acl) = self.default_acl {
            unsafe { libc::acl_free(acl as *mut _) };
        }
    }
}

#[cfg(target_os = "linux")]
pub fn acl_access_nontrivial(acl: acl_t) -> Result<bool, std::io::Error> {
    unsafe {
        let mut entry: acl_entry_t = ptr::null_mut();
        let mut ret = libc::acl_get_entry(acl, ACL_FIRST_ENTRY, &mut entry);

        while ret == 1 {
            let mut tag: acl_tag_t = 0;
            if libc::acl_get_tag_type(entry, &mut tag) != 0 {
                return Err(std::io::Error::last_os_error());
            }

            if !(tag == ACL_USER_OBJ || tag == ACL_GROUP_OBJ || tag == ACL_OTHER) {
                return Ok(true);
            }

            ret = libc::acl_get_entry(acl, ACL_NEXT_ENTRY, &mut entry);
        }

        if ret == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(false)
        }
    }
}

pub fn get_permissions(
    path: &Path,
    fd: c_int,
    mode: mode_t,
) -> Result<PermissionContext, std::io::Error> {
    let mut ctx = PermissionContext::default();
    ctx.mode = mode;

    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "cygwin"
    ))]
    {
        let path_cstr = std::ffi::CString::new(path.as_os_str().as_bytes()).unwrap();
        let acl = unsafe { libc::acl_get_file(path_cstr.as_ptr(), libc::ACL_TYPE_ACCESS) };

        if acl.is_null() {
            let err = std::io::Error::last_os_error();
            if err.raw_os_error() != Some(libc::ENOTSUP) {
                return Err(err);
            }
            ctx.acls_not_supported = true;
        } else {
            ctx.acl = Some(acl);
        }

        #[cfg(not(any(target_os = "macos", target_os = "cygwin")))]
        {
            let default_acl = unsafe { libc::acl_get_file(path_cstr.as_ptr(), libc::ACL_TYPE_DEFAULT) };
            if !default_acl.is_null() {
                ctx.default_acl = Some(default_acl);
            }
        }
    }

    Ok(ctx)
}

pub fn set_permissions(
    ctx: &PermissionContext,
    path: &Path,
    fd: c_int,
) -> Result<(), std::io::Error> {
    let path_cstr = std::ffi::CString::new(path.as_os_str().as_bytes()).unwrap();

    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "cygwin"
    ))]
    if let Some(acl) = ctx.acl {
        if unsafe { libc::acl_set_file(path_cstr.as_ptr(), libc::ACL_TYPE_ACCESS, acl) } != 0 {
            return Err(std::io::Error::last_os_error());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_get_set_permissions() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file");
        File::create(&file_path).unwrap();

        let ctx = get_permissions(&file_path, -1, 0o644).unwrap();
        assert!(set_permissions(&ctx, &file_path, -1).is_ok());
    }
}