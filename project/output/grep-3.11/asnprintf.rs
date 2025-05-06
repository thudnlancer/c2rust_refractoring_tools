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
    fn vasnprintf(
        resultbuf: *mut i8,
        lengthp: *mut size_t,
        format: *const i8,
        args: ::core::ffi::VaList,
    ) -> *mut i8;
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
pub type size_t = u64;
#[no_mangle]
pub unsafe extern "C" fn asnprintf(
    mut resultbuf: *mut i8,
    mut lengthp: *mut size_t,
    mut format: *const i8,
    mut args: ...
) -> *mut i8 {
    let mut args_0: ::core::ffi::VaListImpl;
    let mut result: *mut i8 = 0 as *mut i8;
    args_0 = args.clone();
    result = vasnprintf(resultbuf, lengthp, format, args_0.as_va_list());
    return result;
}