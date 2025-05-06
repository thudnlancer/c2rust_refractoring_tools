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
extern "C" {
    fn __errno_location() -> *mut i32;
}
pub type __mode_t = u32;
pub type mode_t = __mode_t;
pub type security_class_t = libc::c_ushort;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn matchpathcon_init_prefix(
    mut path: *const i8,
    mut prefix: *const i8,
) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn string_to_security_class(
    mut name: *const i8,
) -> security_class_t {
    *__errno_location() = 95 as i32;
    return 0 as i32 as security_class_t;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn security_compute_create(
    mut scon: *const i8,
    mut tcon: *const i8,
    mut tclass: security_class_t,
    mut newcon: *mut *mut i8,
) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn setexeccon(mut con: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn security_check_context_raw(mut con: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn security_check_context(mut con: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn fsetfilecon(mut fd: i32, mut con: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn lsetfilecon(mut file: *const i8, mut con: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn setfilecon(mut file: *const i8, mut con: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn fgetfilecon(mut fd: i32, mut con: *mut *mut i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn lgetfilecon(mut file: *const i8, mut con: *mut *mut i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn getfilecon(mut file: *const i8, mut con: *mut *mut i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn matchpathcon(
    mut file: *const i8,
    mut m: mode_t,
    mut con: *mut *mut i8,
) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn setfscreatecon(mut con: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn getfscreatecon(mut con: *mut *mut i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn freecon(mut con: *mut i8) {}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn getcon(mut con: *mut *mut i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}