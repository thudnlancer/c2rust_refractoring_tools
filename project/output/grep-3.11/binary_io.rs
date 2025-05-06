#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(linkage)]
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn set_binary_mode(mut fd: i32, mut mode: i32) -> i32 {
    return __gl_setmode(fd, mode);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn __gl_setmode(mut fd: i32, mut mode: i32) -> i32 {
    return 0 as i32;
}