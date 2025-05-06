#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic)]
extern "C" {
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn fd_safer(_: i32) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: u32,
    pub fp_offset: u32,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type __mode_t = u32;
pub type mode_t = __mode_t;
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn open_safer(
    mut file: *const i8,
    mut flags: i32,
    mut args: ...
) -> i32 {
    let mut mode: mode_t = 0 as i32 as mode_t;
    if flags & 0o100 as i32 != 0 {
        let mut ap: ::core::ffi::VaListImpl;
        ap = args.clone();
        mode = ap.arg::<mode_t>();
    }
    return fd_safer(open(file, flags, mode));
}