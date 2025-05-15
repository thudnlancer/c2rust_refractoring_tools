/*!
Internal implementation of access control lists.

Copyright (C) 2002-2003, 2005-2022 Free Software Foundation, Inc.

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
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
use std::mem;
use std::os::raw::{c_char, c_void};
use std::ffi::CString;
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
    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "cygwin"
    ))]
    pub default_acl: Option<acl_t>,
    pub acls_not_supported: bool,
}

impl Default for PermissionContext {
    fn default() -> Self {
        PermissionContext {
            mode: 0,
            #[cfg(any(
                target_os = "linux",
                target_os = "freebsd",
                target_os = "macos",
                target_os = "cygwin"
            ))]
            acl: None,
            #[cfg(any(
                target_os = "linux",
                target_os = "freebsd",
                target_os = "macos",
                target_os = "cygwin"
            ))]
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
        {
            if let Some(acl) = self.acl {
                unsafe { libc::acl_free(acl as *mut c_void) };
            }
            if let Some(default_acl) = self.default_acl {
                unsafe { libc::acl_free(default_acl as *mut c_void) };
            }
        }
    }
}

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "cygwin"
))]
pub fn acl_access_nontrivial(acl: acl_t) -> Result<bool, i32> {
    unsafe {
        let mut entry: acl_entry_t = ptr::null_mut();
        let mut ret = libc::acl_get_entry(acl, ACL_FIRST_ENTRY, &mut entry);
        
        while ret == 1 {
            let mut tag: acl_tag_t = 0;
            if libc::acl_get_tag_type(entry, &mut tag) != 0 {
                return Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(-1));
            }
            
            if !(tag == ACL_USER_OBJ || tag == ACL_GROUP_OBJ || tag == ACL_OTHER) {
                return Ok(true);
            }
            
            ret = libc::acl_get_entry(acl, ACL_NEXT_ENTRY, &mut entry);
        }
        
        if ret < 0 {
            Err(std::io::Error::last_os_error().raw_os_error().unwrap_or(-1))
        } else {
            Ok(false)
        }
    }
}

#[cfg(any(
    target_os = "linux",
    target_os = "freebsd",
    target_os = "macos",
    target_os = "cygwin"
))]
pub fn acl_default_nontrivial(acl: acl_t) -> bool {
    unsafe {
        let mut cnt = 0;
        let mut entry: acl_entry_t = ptr::null_mut();
        while libc::acl_get_entry(acl, ACL_NEXT_ENTRY, &mut entry) == 1 {
            cnt += 1;
        }
        cnt > 0
    }
}

pub fn get_permissions(path: &Path, fd: Option<c_int>, mode: mode_t) -> Result<PermissionContext, i32> {
    let mut ctx = PermissionContext::default();
    ctx.mode = mode;

    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "cygwin"
    ))]
    {
        let path_cstr = CString::new(path.to_str().unwrap()).unwrap();
        ctx.acl = unsafe {
            let acl = libc::acl_get_file(path_cstr.as_ptr(), libc::ACL_TYPE_ACCESS);
            if acl.is_null() {
                let err = std::io::Error::last_os_error();
                if err.raw_os_error().unwrap() != libc::ENOTSUP {
                    return Err(err.raw_os_error().unwrap());
                }
                ctx.acls_not_supported = true;
                None
            } else {
                Some(acl)
            }
        };

        if !ctx.acls_not_supported {
            ctx.default_acl = unsafe {
                let acl = libc::acl_get_file(path_cstr.as_ptr(), libc::ACL_TYPE_DEFAULT);
                if !acl.is_null() {
                    Some(acl)
                } else {
                    None
                }
            };
        }
    }

    Ok(ctx)
}

pub fn set_permissions(ctx: &PermissionContext, path: &Path, fd: Option<c_int>) -> Result<(), i32> {
    #[cfg(any(
        target_os = "linux",
        target_os = "freebsd",
        target_os = "macos",
        target_os = "cygwin"
    ))]
    {
        if let Some(acl) = ctx.acl {
            let path_cstr = CString::new(path.to_str().unwrap()).unwrap();
            if unsafe { libc::acl_set_file(path_cstr.as_ptr(), libc::ACL_TYPE_ACCESS, acl) } != 0 {
                return Err(std::io::Error::last_os_error().raw_os_error().unwrap());
            }
        }
    }
    
    Ok(())
}