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
    fn __errno_location() -> *mut i32;
    fn free(__ptr: *mut libc::c_void);
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
pub type size_t = u64;
pub type va_list = __builtin_va_list;
pub type uintptr_t = u64;
#[no_mangle]
pub unsafe extern "C" fn rpl_sprintf(
    mut str: *mut i8,
    mut format: *const i8,
    mut args: ...
) -> i32 {
    let mut output: *mut i8 = 0 as *mut i8;
    let mut len: size_t = 0;
    let mut lenbuf: size_t = 0;
    let mut args_0: ::core::ffi::VaListImpl;
    lenbuf = if (18446744073709551615 as u64) < 2147483647 as i32 as u64 {
        18446744073709551615 as u64
    } else {
        2147483647 as i32 as u64
    };
    if lenbuf > !(str as uintptr_t) {
        lenbuf = !(str as uintptr_t);
    }
    args_0 = args.clone();
    output = vasnprintf(str, &mut lenbuf, format, args_0.as_va_list());
    len = lenbuf;
    if output.is_null() {
        return -(1 as i32);
    }
    if output != str {
        free(output as *mut libc::c_void);
        *__errno_location() = 75 as i32;
        return -(1 as i32);
    }
    if len > 2147483647 as i32 as u64 {
        *__errno_location() = 75 as i32;
        return -(1 as i32);
    }
    return len as i32;
}