/* Get permissions of a file.  -*- coding: utf-8 -*-

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

use std::ffi::CString;
use std::os::unix::prelude::*;
use std::ptr;
use std::mem;
use libc::{mode_t, c_int, c_void, EINVAL, ENOSYS, ENOTSUP, EOPNOTSUPP, errno};

#[repr(C)]
struct PermissionContext {
    mode: mode_t,
    acl: *mut c_void,
    default_acl: *mut c_void,
    count: c_int,
    entries: *mut c_void,
    ace_count: c_int,
    ace_entries: *mut c_void,
    aclv_count: c_int,
    aclv_entries: *mut c_void,
    have_u: bool,
    u: [u8; 0],
}

extern "C" {
    fn acl_get_file(path: *const libc::c_char, acl_type: c_int) -> *mut c_void;
    fn acl_get_fd(fd: c_int) -> *mut c_void;
    fn acl_free(acl: *mut c_void) -> c_int;
    fn acl_errno_valid(err: c_int) -> c_int;
    fn facl(fd: c_int, cmd: c_int, cnt: c_int, aclbuf: *mut c_void) -> c_int;
    fn acl(name: *const libc::c_char, cmd: c_int, cnt: c_int, aclbuf: *mut c_void) -> c_int;
    fn fgetacl(fd: c_int, nentries: c_int, aclbuf: *mut c_void) -> c_int;
    fn getacl(name: *const libc::c_char, nentries: c_int, aclbuf: *mut c_void) -> c_int;
    fn fstatacl(fd: c_int, cmd: c_int, buf: *mut c_void, bufsize: c_int) -> c_int;
    fn statacl(name: *const libc::c_char, cmd: c_int, buf: *mut c_void, bufsize: c_int) -> c_int;
}

fn get_permissions(name: Option<&str>, desc: Option<RawFd>, mode: mode_t) -> Result<PermissionContext, std::io::Error> {
    let mut ctx = PermissionContext {
        mode,
        acl: ptr::null_mut(),
        default_acl: ptr::null_mut(),
        count: 0,
        entries: ptr::null_mut(),
        ace_count: 0,
        ace_entries: ptr::null_mut(),
        aclv_count: 0,
        aclv_entries: ptr::null_mut(),
        have_u: false,
        u: [],
    };

    unsafe {
        #[cfg(all(USE_ACL, HAVE_ACL_GET_FILE))]
        {
            if !HAVE_ACL_TYPE_EXTENDED {
                ctx.acl = if let Some(fd) = desc {
                    acl_get_fd(fd)
                } else {
                    let cname = CString::new(name.unwrap()).unwrap();
                    acl_get_file(cname.as_ptr(), ACL_TYPE_ACCESS)
                };

                if ctx.acl.is_null() {
                    if acl_errno_valid(errno()) != 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    return Ok(ctx);
                }

                if (mode & libc::S_IFMT) == libc::S_IFDIR {
                    let cname = CString::new(name.unwrap()).unwrap();
                    ctx.default_acl = acl_get_file(cname.as_ptr(), ACL_TYPE_DEFAULT);
                    if ctx.default_acl.is_null() {
                        acl_free(ctx.acl);
                        return Err(std::io::Error::last_os_error());
                    }
                }
            } else {
                ctx.acl = if let Some(fd) = desc {
                    acl_get_fd(fd)
                } else {
                    let cname = CString::new(name.unwrap()).unwrap();
                    acl_get_file(cname.as_ptr(), ACL_TYPE_EXTENDED)
                };

                if ctx.acl.is_null() {
                    if acl_errno_valid(errno()) != 0 {
                        return Err(std::io::Error::last_os_error());
                    }
                    return Ok(ctx);
                }
            }
        }

        #[cfg(all(USE_ACL, defined(GETACL)))]
        {
            #[cfg(defined(ACE_GETACL))]
            {
                loop {
                    let ret = if let Some(fd) = desc {
                        facl(fd, ACE_GETACLCNT, 0, ptr::null_mut())
                    } else {
                        let cname = CString::new(name.unwrap()).unwrap();
                        acl(cname.as_ptr(), ACE_GETACLCNT, 0, ptr::null_mut())
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

                    ctx.ace_entries = libc::malloc(ctx.ace_count as usize * mem::size_of::<ace_t>()) as *mut _;
                    if ctx.ace_entries.is_null() {
                        return Err(std::io::Error::from_raw_os_error(libc::ENOMEM));
                    }

                    let ret = if let Some(fd) = desc {
                        facl(fd, ACE_GETACL, ctx.ace_count, ctx.ace_entries)
                    } else {
                        let cname = CString::new(name.unwrap()).unwrap();
                        acl(cname.as_ptr(), ACE_GETACL, ctx.ace_count, ctx.ace_entries)
                    };

                    if ret < 0 {
                        if errno() == ENOSYS || errno() == EINVAL {
                            libc::free(ctx.ace_entries);
                            ctx.ace_entries = ptr::null_mut();
                            ctx.ace_count = 0;
                            break;
                        } else {
                            libc::free(ctx.ace_entries);
                            return Err(std::io::Error::last_os_error());
                        }
                    }

                    if ret <= ctx.ace_count {
                        ctx.ace_count = ret;
                        break;
                    }

                    libc::free(ctx.ace_entries);
                    ctx.ace_entries = ptr::null_mut();
                }
            }

            loop {
                let ret = if let Some(fd) = desc {
                    facl(fd, GETACLCNT, 0, ptr::null_mut())
                } else {
                    let cname = CString::new(name.unwrap()).unwrap();
                    acl(cname.as_ptr(), GETACLCNT, 0, ptr::null_mut())
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

                ctx.entries = libc::malloc(ctx.count as usize * mem::size_of::<aclent_t>()) as *mut _;
                if ctx.entries.is_null() {
                    return Err(std::io::Error::from_raw_os_error(libc::ENOMEM));
                }

                let ret = if let Some(fd) = desc {
                    facl(fd, GETACL, ctx.count, ctx.entries)
                } else {
                    let cname = CString::new(name.unwrap()).unwrap();
                    acl(cname.as_ptr(), GETACL, ctx.count, ctx.entries)
                };

                if ret < 0 {
                    if errno() == ENOSYS || errno() == ENOTSUP || errno() == EOPNOTSUPP {
                        libc::free(ctx.entries);
                        ctx.entries = ptr::null_mut();
                        ctx.count = 0;
                        break;
                    } else {
                        libc::free(ctx.entries);
                        return Err(std::io::Error::last_os_error());
                    }
                }

                if ret <= ctx.count {
                    ctx.count = ret;
                    break;
                }

                libc::free(ctx.entries);
                ctx.entries = ptr::null_mut();
            }
        }
    }

    Ok(ctx)
}