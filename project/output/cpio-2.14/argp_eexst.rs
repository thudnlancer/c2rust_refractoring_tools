#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type error_t = i32;
#[no_mangle]
pub static mut argp_err_exit_status: error_t = 64 as i32;