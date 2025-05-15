use std::os::raw::{c_char, c_int};

pub type mode_t = u32;

#[derive(Default, Clone, Copy)]
pub struct PermissionContext {
    pub mode: mode_t,
}

pub fn get_permissions(
    name: *const c_char,
    desc: c_int,
    mode: mode_t,
    ctx: &mut PermissionContext,
) -> c_int {
    *ctx = PermissionContext { mode };
    0
}