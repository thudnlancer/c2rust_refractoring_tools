#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(linkage)]
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
pub type security_class_t = libc::c_ushort;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn matchpathcon_init_prefix(
    mut path: *const libc::c_char,
    mut prefix: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn getcon(mut con: *mut *mut libc::c_char) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn freecon(mut con: *mut libc::c_char) {}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn getfscreatecon(mut con: *mut *mut libc::c_char) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn setfscreatecon(mut con: *const libc::c_char) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn matchpathcon(
    mut file: *const libc::c_char,
    mut m: mode_t,
    mut con: *mut *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn getfilecon(
    mut file: *const libc::c_char,
    mut con: *mut *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn lgetfilecon(
    mut file: *const libc::c_char,
    mut con: *mut *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn fgetfilecon(
    mut fd: libc::c_int,
    mut con: *mut *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn setfilecon(
    mut file: *const libc::c_char,
    mut con: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn lsetfilecon(
    mut file: *const libc::c_char,
    mut con: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn fsetfilecon(
    mut fd: libc::c_int,
    mut con: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn security_check_context(
    mut con: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn security_check_context_raw(
    mut con: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn setexeccon(mut con: *const libc::c_char) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn security_compute_create(
    mut scon: *const libc::c_char,
    mut tcon: *const libc::c_char,
    mut tclass: security_class_t,
    mut newcon: *mut *mut libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn string_to_security_class(
    mut name: *const libc::c_char,
) -> security_class_t {
    *__errno_location() = 95 as libc::c_int;
    return 0 as libc::c_int as security_class_t;
}
