#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type selinux_opt;
    pub type selabel_handle;
    fn __errno_location() -> *mut libc::c_int;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn selabel_lookup(
    mut hnd: *mut selabel_handle,
    mut context: *mut *mut libc::c_char,
    mut key: *const libc::c_char,
    mut type_0: libc::c_int,
) -> libc::c_int {
    *__errno_location() = 95 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn selabel_open(
    mut backend: libc::c_int,
    mut options: *mut selinux_opt,
    mut nopt: libc::c_uint,
) -> *mut selabel_handle {
    *__errno_location() = 95 as libc::c_int;
    return 0 as *mut selabel_handle;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn selabel_close(mut hnd: *mut selabel_handle) {
    *__errno_location() = 95 as libc::c_int;
}
