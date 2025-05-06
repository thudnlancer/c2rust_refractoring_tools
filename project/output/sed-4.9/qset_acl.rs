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
    fn set_permissions(_: *mut permission_context, _: *const i8, _: i32) -> i32;
    fn free_permission_context(_: *mut permission_context);
}
pub type __mode_t = u32;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct permission_context {
    pub mode: mode_t,
}
#[no_mangle]
pub unsafe extern "C" fn qset_acl(
    mut name: *const i8,
    mut desc: i32,
    mut mode: mode_t,
) -> i32 {
    let mut ctx: permission_context = permission_context { mode: 0 };
    let mut ret: i32 = 0;
    memset(
        &mut ctx as *mut permission_context as *mut libc::c_void,
        0 as i32,
        ::core::mem::size_of::<permission_context>() as u64,
    );
    ctx.mode = mode;
    ret = set_permissions(&mut ctx, name, desc);
    free_permission_context(&mut ctx);
    return ret;
}