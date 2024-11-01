#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(linkage)]
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}
pub type context_t = libc::c_int;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_new(mut s: *const libc::c_char) -> context_t {
    *__errno_location() = 95 as libc::c_int;
    return 0 as libc::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_str(mut con: context_t) -> *mut libc::c_char {
    *__errno_location() = 95 as libc::c_int;
    return 0 as *mut libc::c_char;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_free(mut c: context_t) {}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_user_set(
    mut sc: context_t,
    mut s: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_role_set(
    mut sc: context_t,
    mut s: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_range_set(
    mut sc: context_t,
    mut s: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_type_set(
    mut sc: context_t,
    mut s: *const libc::c_char,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_type_get(mut sc: context_t) -> *mut libc::c_char {
    *__errno_location() = 95 as libc::c_int;
    return 0 as *mut libc::c_char;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_range_get(mut sc: context_t) -> *mut libc::c_char {
    *__errno_location() = 95 as libc::c_int;
    return 0 as *mut libc::c_char;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_role_get(mut sc: context_t) -> *mut libc::c_char {
    *__errno_location() = 95 as libc::c_int;
    return 0 as *mut libc::c_char;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_user_get(mut sc: context_t) -> *mut libc::c_char {
    *__errno_location() = 95 as libc::c_int;
    return 0 as *mut libc::c_char;
}
