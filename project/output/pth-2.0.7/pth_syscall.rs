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
pub static mut pth_syscall_soft: i32 = 0 as i32;
#[no_mangle]
pub static mut pth_syscall_hard: i32 = 0 as i32;
#[no_mangle]
pub unsafe extern "C" fn __pth_syscall_init() {}
#[no_mangle]
pub unsafe extern "C" fn __pth_syscall_kill() {}