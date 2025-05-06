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
pub type context_t = i32;
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_new(mut s: *const i8) -> context_t {
    *__errno_location() = 95 as i32;
    return 0 as i32;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_str(mut con: context_t) -> *mut i8 {
    *__errno_location() = 95 as i32;
    return 0 as *mut i8;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_free(mut c: context_t) {}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_user_set(mut sc: context_t, mut s: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_role_set(mut sc: context_t, mut s: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_range_set(mut sc: context_t, mut s: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_type_set(mut sc: context_t, mut s: *const i8) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_type_get(mut sc: context_t) -> *mut i8 {
    *__errno_location() = 95 as i32;
    return 0 as *mut i8;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_range_get(mut sc: context_t) -> *mut i8 {
    *__errno_location() = 95 as i32;
    return 0 as *mut i8;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_role_get(mut sc: context_t) -> *mut i8 {
    *__errno_location() = 95 as i32;
    return 0 as *mut i8;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn context_user_get(mut sc: context_t) -> *mut i8 {
    *__errno_location() = 95 as i32;
    return 0 as *mut i8;
}