#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(thread_local)]
#[thread_local]
static mut tls: *mut libc::c_void = 0 as *const libc::c_void as *mut libc::c_void;
#[no_mangle]
pub unsafe extern "C" fn _glp_tls_set_ptr(mut ptr: *mut libc::c_void) {
    tls = ptr;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_tls_get_ptr() -> *mut libc::c_void {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    ptr = tls;
    return ptr;
}