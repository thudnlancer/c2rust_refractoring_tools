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
#[inline]
unsafe extern "C" fn __gl_setmode(mut fd: i32, mut mode: i32) -> i32 {
    return 0 as i32;
}
#[inline]
unsafe extern "C" fn set_binary_mode(mut fd: i32, mut mode: i32) -> i32 {
    return __gl_setmode(fd, mode);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn xset_binary_mode_error() {}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn xset_binary_mode(mut fd: i32, mut mode: i32) {
    if set_binary_mode(fd, mode) < 0 as i32 {
        xset_binary_mode_error();
    }
}