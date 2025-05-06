#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn memset(_: *mut libc::c_void, _: i32, _: u64) -> *mut libc::c_void;
}
pub type __mode_t = u32;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct permission_context {
    pub mode: mode_t,
}
#[no_mangle]
pub unsafe extern "C" fn get_permissions(
    mut name: *const i8,
    mut desc: i32,
    mut mode: mode_t,
    mut ctx: *mut permission_context,
) -> i32 {
    memset(
        ctx as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<permission_context>() as u64,
    );
    (*ctx).mode = mode;
    return 0 as i32;
}