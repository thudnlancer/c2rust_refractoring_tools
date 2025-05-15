/* Get permissions of a file.  -*- coding: utf-8 -*-

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

use std::ffi::CString;
use std::os::unix::prelude::*;
use std::ptr;
use libc::{mode_t, c_int, c_void, EINVAL, ENOSYS, ENOTSUP, EOPNOTSUPP, errno};

#[repr(C)]
struct Acl(usize);

#[repr(C)]
struct AclEntry(usize);

#[repr(C)]
struct PermissionContext {
    mode: mode_t,
    acl: Option<Acl>,
    default_acl: Option<Acl>,
    ace_count: c_int,
    ace_entries: Option<Box<[u8]>>,
    count: c_int,
    entries: Option<Box<[u8]>>,
    aclv_count: c_int,
    aclv_entries: Option<Box<[u8]>>,
    have_u: bool,
    u: [u8; 0],
}

extern "C" {
    fn acl_get_file(path: *const libc::c_char, acl_type: c_int) -> *mut Acl;
    fn acl_get_fd(fd: c_int) -> *mut Acl;
    fn acl_free(acl_p: *mut c_void) -> c_int;
    fn facl(fd: c_int, cmd: c_int, cnt: c_int, buf: *mut c_void) -> c_int;
    fn acl(name: *const libc::c_char, cmd: c_int, cnt: c_int, buf: *mut c_void) -> c_int;
    fn fgetacl(fd: c_int, nentries: c_int, aclbuf: *mut c_void) -> c_int;
    fn getacl(name: *const libc::c_char, nentries: c_int, aclbuf: *mut c_void) -> c_int;
    fn fstatacl(fd: c_int, cmd: c_int, buf: *mut c_void, nbytes: c_int) -> c_int;
    fn statacl(name: *const libc::c_char, cmd: c_int, buf: *mut c_void, nbytes: c_int) -> c_int;
}

fn acl_errno_valid(err: c_int) -> bool {
    err != libc::ENOTSUP && err != libc::EOPNOTSUPP && err != libc::ENOSYS && err != libc::EINVAL
}

fn get_permissions(name: Option<&str>, desc: Option<RawFd>, mode: mode_t) -> Result<PermissionContext, std::io::Error> {
    let mut ctx = PermissionContext {
        mode,
        acl: None,
        default_acl: None,
        ace_count: 0,
        ace_entries: None,
        count: 0,
        entries: None,
        aclv_count: 0,
        aclv_entries: None,
        have_u: false,
        u: [],
    };

    #[cfg(all(USE_ACL, HAVE_ACL_GET_FILE))]
    {
        ctx.acl = if let Some(fd) = desc {
            let acl = unsafe { acl_get_fd(fd) };
            if acl.is_null() {
                if acl_errno_valid(errno()) {
                    return Err(std::io::Error::last_os_error());
                } else {
                    None
                }
            } else {
                Some(unsafe { Acl(acl as usize) })
            }
        } else {
            let cname = CString::new(name.unwrap()).unwrap();
            let acl = unsafe { acl_get_file(cname.as_ptr(), libc::ACL_TYPE_ACCESS) };
            if acl.is_null() {
                if acl_errno_valid(errno()) {
                    return Err(std::io::Error::last_os_error());
                } else {
                    None
                }
            } else {
                Some(unsafe { Acl(acl as usize) })
            }
        };

        if (mode & libc::S_IFMT) == libc::S_IFDIR {
            let cname = CString::new(name.unwrap()).unwrap();
            let default_acl = unsafe { acl_get_file(cname.as_ptr(), libc::ACL_TYPE_DEFAULT) };
            if default_acl.is_null() {
                return Err(std::io::Error::last_os_error());
            }
            ctx.default_acl = Some(unsafe { Acl(default_acl as usize) });
        }
    }

    #[cfg(all(USE_ACL, defined(GETACL)))]
    {
        #[cfg(defined(ACE_GETACL))]
        {
            loop {
                let ret = if let Some(fd) = desc {
                    unsafe { facl(fd, libc::ACE_GETACLCNT, 0, ptr::null_mut()) }
                } else {
                    let cname = CString::new(name.unwrap()).unwrap();
                    unsafe { acl(cname.as_ptr(), libc::ACE_GETACLCNT, 0, ptr::null_mut()) }
                };

                if ret < 0 {
                    if errno() == ENOSYS || errno() == EINVAL {
                        ctx.ace_count = 0;
                        break;
                    } else {
                        return Err(std::io::Error::last_os_error());
                    }
                }
                ctx.ace_count = ret;

                if ctx.ace_count == 0 {
                    break;
                }

                let mut entries = vec![0u8; ctx.ace_count as usize * std::mem::size_of::<u8>()];
                let ret = if let Some(fd) = desc {
                    unsafe { facl(fd, libc::ACE_GETACL, ctx.ace_count, entries.as_mut_ptr() as *mut c_void) }
                } else {
                    let cname = CString::new(name.unwrap()).unwrap();
                    unsafe { acl(cname.as_ptr(), libc::ACE_GETACL, ctx.ace_count, entries.as_mut_ptr() as *mut c_void) }
                };

                if ret < 0 {
                    if errno() == ENOSYS || errno() == EINVAL {
                        ctx.ace_count = 0;
                        break;
                    } else {
                        return Err(std::io::Error::last_os_error());
                    }
                }

                if ret <= ctx.ace_count {
                    ctx.ace_count = ret;
                    ctx.ace_entries = Some(entries.into_boxed_slice());
                    break;
                }
            }
        }

        loop {
            let ret = if let Some(fd) = desc {
                unsafe { facl(fd, libc::GETACLCNT, 0, ptr::null_mut()) }
            } else {
                let cname = CString::new(name.unwrap()).unwrap();
                unsafe { acl(cname.as_ptr(), libc::GETACLCNT, 0, ptr::null_mut()) }
            };

            if ret < 0 {
                if errno() == ENOSYS || errno() == ENOTSUP || errno() == EOPNOTSUPP {
                    ctx.count = 0;
                    break;
                } else {
                    return Err(std::io::Error::last_os_error());
                }
            }
            ctx.count = ret;

            if ctx.count == 0 {
                break;
            }

            let mut entries = vec![0u8; ctx.count as usize * std::mem::size_of::<u8>()];
            let ret = if let Some(fd) = desc {
                unsafe { facl(fd, libc::GETACL, ctx.count, entries.as_mut_ptr() as *mut c_void) }
            } else {
                let cname = CString::new(name.unwrap()).unwrap();
                unsafe { acl(cname.as_ptr(), libc::GETACL, ctx.count, entries.as_mut_ptr() as *mut c_void) }
            };

            if ret < 0 {
                if errno() == ENOSYS || errno() == ENOTSUP || errno() == EOPNOTSUPP {
                    ctx.count = 0;
                    break;
                } else {
                    return Err(std::io::Error::last_os_error());
                }
            }

            if ret <= ctx.count {
                ctx.count = ret;
                ctx.entries = Some(entries.into_boxed_slice());
                break;
            }
        }
    }

    Ok(ctx)
}