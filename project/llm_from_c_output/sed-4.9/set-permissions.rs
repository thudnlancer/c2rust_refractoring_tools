/* Set permissions of a file.  -*- coding: utf-8 -*-

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

   Written by Paul Eggert, Andreas Grünbacher, and Bruno Haible.  */

use std::fs;
use std::os::unix::fs::{PermissionsExt, MetadataExt};
use std::path::Path;
use std::io;

#[cfg(target_os = "freebsd")]
use libc::{acl_free, acl_from_text, acl_set_file, acl_set_fd, ACL_TYPE_ACCESS, ACL_TYPE_DEFAULT};

struct PermissionContext {
    mode: u32,
    acls_not_supported: bool,
    acl: Option<*mut libc::acl_t>,
    default_acl: Option<*mut libc::acl_t>,
}

impl Drop for PermissionContext {
    fn drop(&mut self) {
        if let Some(acl) = self.acl {
            unsafe { acl_free(acl as *mut libc::c_void) };
        }
        if let Some(default_acl) = self.default_acl {
            unsafe { acl_free(default_acl as *mut libc::c_void) };
        }
    }
}

#[cfg(target_os = "freebsd")]
fn acl_from_mode(mode: u32) -> Option<*mut libc::acl_t> {
    let mut acl_text = if cfg!(target_os = "tru64") {
        "u::---,g::---,o::---,".to_string()
    } else {
        "u::---,g::---,o::---".to_string()
    };

    let mut acl_chars: Vec<char> = acl_text.chars().collect();
    
    if mode & libc::S_IRUSR != 0 { acl_chars[3] = 'r'; }
    if mode & libc::S_IWUSR != 0 { acl_chars[4] = 'w'; }
    if mode & libc::S_IXUSR != 0 { acl_chars[5] = 'x'; }
    if mode & libc::S_IRGRP != 0 { acl_chars[10] = 'r'; }
    if mode & libc::S_IWGRP != 0 { acl_chars[11] = 'w'; }
    if mode & libc::S_IXGRP != 0 { acl_chars[12] = 'x'; }
    if mode & libc::S_IROTH != 0 { acl_chars[17] = 'r'; }
    if mode & libc::S_IWOTH != 0 { acl_chars[18] = 'w'; }
    if mode & libc::S_IXOTH != 0 { acl_chars[19] = 'x'; }

    acl_text = acl_chars.into_iter().collect();
    let c_str = std::ffi::CString::new(acl_text).ok()?;
    unsafe { Some(acl_from_text(c_str.as_ptr())) }
}

fn chmod_or_fchmod(name: &Path, desc: Option<i32>, mode: u32) -> io::Result<()> {
    if let Some(fd) = desc {
        unsafe {
            if libc::fchmod(fd, mode) == 0 {
                Ok(())
            } else {
                Err(io::Error::last_os_error())
            }
        }
    } else {
        fs::set_permissions(name, fs::Permissions::from_mode(mode))
    }
}

fn set_permissions(ctx: &PermissionContext, name: &Path, desc: Option<i32>) -> io::Result<()> {
    let mut acls_set = false;
    let mut must_chmod = false;
    let mut ret = Ok(());

    let early_chmod = if cfg!(target_os = "aix") {
        false
    } else {
        (ctx.mode & (libc::S_ISUID | libc::S_ISGID | libc::S_ISVTX)) != 0
    };

    if early_chmod {
        ret = chmod_or_fchmod(name, desc, ctx.mode);
        if ret.is_err() {
            return ret;
        }
    }

    #[cfg(target_os = "freebsd")]
    {
        if !ctx.acls_not_supported {
            if ctx.acl.is_none() {
                if let Some(acl) = acl_from_mode(ctx.mode) {
                    unsafe {
                        let result = if let Some(fd) = desc {
                            acl_set_fd(fd, acl)
                        } else {
                            let c_name = std::ffi::CString::new(name.to_str().unwrap()).unwrap();
                            acl_set_file(c_name.as_ptr(), ACL_TYPE_ACCESS, acl)
                        };

                        if result != 0 {
                            if !acl_errno_valid(io::Error::last_os_error().raw_os_error().unwrap()) {
                                ctx.acls_not_supported = true;
                                if acl_access_nontrivial(acl) == 0 {
                                    ret = Ok(());
                                } else {
                                    ret = Err(io::Error::last_os_error());
                                }
                            } else {
                                ret = Err(io::Error::last_os_error());
                            }
                        } else {
                            acls_set = true;
                            if (ctx.mode & libc::S_IFMT) == libc::S_IFDIR {
                                if let Some(default_acl) = ctx.default_acl {
                                    if acl_default_nontrivial(default_acl) {
                                        let c_name = std::ffi::CString::new(name.to_str().unwrap()).unwrap();
                                        let result = acl_set_file(c_name.as_ptr(), ACL_TYPE_DEFAULT, default_acl);
                                        if result != 0 {
                                            ret = Err(io::Error::last_os_error());
                                        }
                                    }
                                } else {
                                    let c_name = std::ffi::CString::new(name.to_str().unwrap()).unwrap();
                                    let result = acl_delete_def_file(c_name.as_ptr());
                                    if result != 0 {
                                        ret = Err(io::Error::last_os_error());
                                    }
                                }
                            }
                        }
                    }
                } else {
                    ret = Err(io::Error::new(io::ErrorKind::Other, "Failed to create ACL from mode"));
                }
            }
        }
    }

    if must_chmod && !early_chmod {
        let saved_errno = if ret.is_err() {
            Some(ret.as_ref().err().unwrap().raw_os_error().unwrap())
        } else {
            None
        };

        ret = chmod_or_fchmod(name, desc, ctx.mode);

        if let Some(errno) = saved_errno {
            ret = Err(io::Error::from_raw_os_error(errno));
        }
    }

    ret
}

// Helper functions (would need proper implementations)
#[cfg(target_os = "freebsd")]
fn acl_errno_valid(errno: i32) -> bool {
    // Implementation depends on platform-specific error codes
    true
}

#[cfg(target_os = "freebsd")]
fn acl_access_nontrivial(acl: *mut libc::acl_t) -> i32 {
    // Implementation would check if ACL is non-trivial
    0
}

#[cfg(target_os = "freebsd")]
fn acl_default_nontrivial(acl: *mut libc::acl_t) -> bool {
    // Implementation would check if default ACL is non-trivial
    false
}

#[cfg(target_os = "freebsd")]
fn acl_delete_def_file(name: *const libc::c_char) -> i32 {
    // Implementation would delete default ACL
    0
}