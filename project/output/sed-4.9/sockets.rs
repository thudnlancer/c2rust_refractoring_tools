#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn gl_sockets_startup(mut version: i32) -> i32 {
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn gl_sockets_cleanup() -> i32 {
    return 0 as i32;
}