use std::ffi::CStr;
use std::os::unix::io::RawFd;

type Mode = u32;

#[derive(Clone, Copy)]
pub struct PermissionContext {
    pub mode: Mode,
}

impl PermissionContext {
    fn new() -> Self {
        PermissionContext { mode: 0 }
    }

    fn get_permissions(&mut self, src_name: &CStr, source_desc: RawFd, mode: Mode) -> Result<(), i32> {
        let ret = unsafe {
            libc::get_permissions(
                src_name.as_ptr(),
                source_desc,
                mode,
                self as *mut PermissionContext,
            )
        };
        if ret != 0 {
            Err(-2)
        } else {
            Ok(())
        }
    }

    fn set_permissions(&self, dst_name: &CStr, dest_desc: RawFd) -> Result<(), i32> {
        let ret = unsafe {
            libc::set_permissions(
                self as *const PermissionContext as *mut PermissionContext,
                dst_name.as_ptr(),
                dest_desc,
            )
        };
        if ret != 0 {
            Err(ret)
        } else {
            Ok(())
        }
    }
}

impl Drop for PermissionContext {
    fn drop(&mut self) {
        unsafe {
            libc::free_permission_context(self as *mut PermissionContext);
        }
    }
}

pub fn qcopy_acl(
    src_name: &CStr,
    source_desc: RawFd,
    dst_name: &CStr,
    dest_desc: RawFd,
    mode: Mode,
) -> Result<(), i32> {
    let mut ctx = PermissionContext::new();
    ctx.get_permissions(src_name, source_desc, mode)?;
    ctx.set_permissions(dst_name, dest_desc)
}

mod libc {
    use super::{Mode, PermissionContext};
    use std::os::raw::{c_char, c_int};

    extern "C" {
        pub fn set_permissions(
            ctx: *mut PermissionContext,
            path: *const c_char,
            fd: c_int,
        ) -> c_int;
        pub fn get_permissions(
            path: *const c_char,
            fd: c_int,
            mode: Mode,
            ctx: *mut PermissionContext,
        ) -> c_int;
        pub fn free_permission_context(ctx: *mut PermissionContext);
    }
}