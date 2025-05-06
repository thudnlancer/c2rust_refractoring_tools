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
    fn chmod(__file: *const i8, __mode: __mode_t) -> i32;
    fn fchmod(__fd: i32, __mode: __mode_t) -> i32;
    fn __errno_location() -> *mut i32;
}
pub type __mode_t = u32;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct permission_context {
    pub mode: mode_t,
}
#[no_mangle]
pub unsafe extern "C" fn chmod_or_fchmod(
    mut name: *const i8,
    mut desc: i32,
    mut mode: mode_t,
) -> i32 {
    if 1 as i32 != 0 && desc != -(1 as i32) {
        return fchmod(desc, mode)
    } else {
        return chmod(name, mode)
    };
}
#[no_mangle]
pub unsafe extern "C" fn set_permissions(
    mut ctx: *mut permission_context,
    mut name: *const i8,
    mut desc: i32,
) -> i32 {
    let mut acls_set: bool = 0 as i32 != 0;
    let mut early_chmod: bool = false;
    let mut must_chmod: bool = 0 as i32 != 0;
    let mut ret: i32 = 0 as i32;
    early_chmod = 1 as i32 != 0;
    if early_chmod {
        ret = chmod_or_fchmod(name, desc, (*ctx).mode);
        if ret != 0 as i32 {
            return -(1 as i32);
        }
    }
    if must_chmod as i32 != 0 && !early_chmod {
        let mut saved_errno: i32 = if ret != 0 { *__errno_location() } else { 0 as i32 };
        ret = chmod_or_fchmod(name, desc, (*ctx).mode);
        if saved_errno != 0 {
            *__errno_location() = saved_errno;
            ret = -(1 as i32);
        }
    }
    return ret;
}