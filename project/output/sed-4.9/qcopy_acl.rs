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
    fn set_permissions(_: *mut permission_context, _: *const i8, _: i32) -> i32;
    fn get_permissions(
        _: *const i8,
        _: i32,
        _: mode_t,
        _: *mut permission_context,
    ) -> i32;
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
pub unsafe extern "C" fn qcopy_acl(
    mut src_name: *const i8,
    mut source_desc: i32,
    mut dst_name: *const i8,
    mut dest_desc: i32,
    mut mode: mode_t,
) -> i32 {
    let mut ctx: permission_context = permission_context { mode: 0 };
    let mut ret: i32 = 0;
    ret = get_permissions(src_name, source_desc, mode, &mut ctx);
    if ret != 0 as i32 {
        return -(2 as i32);
    }
    ret = set_permissions(&mut ctx, dst_name, dest_desc);
    free_permission_context(&mut ctx);
    return ret;
}