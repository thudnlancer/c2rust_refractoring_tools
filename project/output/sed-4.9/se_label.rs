#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types, linkage)]
extern "C" {
    pub type selinux_opt;
    pub type selabel_handle;
    fn __errno_location() -> *mut i32;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn selabel_lookup(
    mut hnd: *mut selabel_handle,
    mut context: *mut *mut i8,
    mut key: *const i8,
    mut type_0: i32,
) -> i32 {
    *__errno_location() = 95 as i32;
    return -(1 as i32);
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn selabel_open(
    mut backend: i32,
    mut options: *mut selinux_opt,
    mut nopt: u32,
) -> *mut selabel_handle {
    *__errno_location() = 95 as i32;
    return 0 as *mut selabel_handle;
}
#[no_mangle]
#[inline]
#[linkage = "external"]
pub unsafe extern "C" fn selabel_close(mut hnd: *mut selabel_handle) {
    *__errno_location() = 95 as i32;
}