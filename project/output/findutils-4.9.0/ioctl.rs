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
    fn ioctl(__fd: i32, __request: u64, _: ...) -> i32;
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
pub type va_list = __builtin_va_list;
#[no_mangle]
pub unsafe extern "C" fn rpl_ioctl(mut fd: i32, mut request: i32, mut args: ...) -> i32 {
    let mut buf: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut args_0: ::core::ffi::VaListImpl;
    args_0 = args.clone();
    buf = args_0.arg::<*mut libc::c_void>();
    return ioctl(fd, request as u32 as u64, buf);
}