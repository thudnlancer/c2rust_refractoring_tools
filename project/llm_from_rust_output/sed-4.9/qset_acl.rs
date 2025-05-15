use std::ffi::CStr;
use std::os::raw::{c_char, c_int};

pub type mode_t = u32;

#[derive(Default)]
pub struct PermissionContext {
    pub mode: mode_t,
}

impl PermissionContext {
    pub fn new(mode: mode_t) -> Self {
        PermissionContext { mode }
    }

    pub fn set_permissions(&mut self, name: &CStr, desc: c_int) -> c_int {
        // Assuming this needs to call into C code, we'd isolate the unsafe block here
        unsafe {
            extern "C" {
                fn set_permissions(
                    ctx: *mut PermissionContext,
                    name: *const c_char,
                    desc: c_int,
                ) -> c_int;
            }
            set_permissions(self as *mut _, name.as_ptr(), desc)
        }
    }
}

impl Drop for PermissionContext {
    fn drop(&mut self) {
        unsafe {
            extern "C" {
                fn free_permission_context(ctx: *mut PermissionContext);
            }
            free_permission_context(self as *mut _);
        }
    }
}

#[no_mangle]
pub extern "C" fn qset_acl(name: *const c_char, desc: c_int, mode: mode_t) -> c_int {
    let mut ctx = PermissionContext::new(mode);
    let name_cstr = unsafe { CStr::from_ptr(name) };
    let ret = ctx.set_permissions(name_cstr, desc);
    ret
}