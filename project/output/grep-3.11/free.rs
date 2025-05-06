#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn __errno_location() -> *mut i32;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_free(mut p: *mut libc::c_void) {
    let mut err: i32 = *__errno_location();
    free(p);
    *__errno_location() = err;
}