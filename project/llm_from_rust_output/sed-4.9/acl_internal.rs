use libc::c_uint;

pub type __mode_t = c_uint;
pub type mode_t = __mode_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct PermissionContext {
    pub mode: mode_t,
}

impl PermissionContext {
    pub fn free(self) {}
}