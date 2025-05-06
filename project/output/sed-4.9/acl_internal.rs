#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __mode_t = u32;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct permission_context {
    pub mode: mode_t,
}
#[no_mangle]
pub unsafe extern "C" fn free_permission_context(mut ctx: *mut permission_context) {}