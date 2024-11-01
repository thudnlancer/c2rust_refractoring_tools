#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub unsafe extern "C" fn gl_sockets_startup(mut version: libc::c_int) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gl_sockets_cleanup() -> libc::c_int {
    return 0 as libc::c_int;
}
