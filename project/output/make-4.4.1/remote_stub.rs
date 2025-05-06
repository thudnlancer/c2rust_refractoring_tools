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
    fn __errno_location() -> *mut i32;
}
pub type __pid_t = i32;
pub type pid_t = __pid_t;
#[no_mangle]
pub static mut remote_description: *mut i8 = 0 as *const i8 as *mut i8;
#[no_mangle]
pub unsafe extern "C" fn remote_setup() {}
#[no_mangle]
pub unsafe extern "C" fn remote_cleanup() {}
#[no_mangle]
pub unsafe extern "C" fn start_remote_job_p(mut first_p: i32) -> i32 {
    return 0 as i32;
}
#[no_mangle]
pub unsafe extern "C" fn start_remote_job(
    mut argv: *mut *mut i8,
    mut envp: *mut *mut i8,
    mut stdin_fd: i32,
    mut is_remote: *mut i32,
    mut id_ptr: *mut pid_t,
    mut used_stdin: *mut i32,
) -> i32 {
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn remote_status(
    mut exit_code_ptr: *mut i32,
    mut signal_ptr: *mut i32,
    mut coredump_ptr: *mut i32,
    mut block: i32,
) -> i32 {
    *__errno_location() = 10 as i32;
    return -(1 as i32);
}
#[no_mangle]
pub unsafe extern "C" fn block_remote_children() {}
#[no_mangle]
pub unsafe extern "C" fn unblock_remote_children() {}
#[no_mangle]
pub unsafe extern "C" fn remote_kill(mut id: pid_t, mut sig: i32) -> i32 {
    return -(1 as i32);
}