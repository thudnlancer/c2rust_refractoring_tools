#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}
pub type __pid_t = libc::c_int;
pub type pid_t = __pid_t;
#[no_mangle]
pub static mut remote_description: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn remote_setup() {}
#[no_mangle]
pub unsafe extern "C" fn remote_cleanup() {}
#[no_mangle]
pub unsafe extern "C" fn start_remote_job_p(mut first_p: libc::c_int) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn start_remote_job(
    mut argv: *mut *mut libc::c_char,
    mut envp: *mut *mut libc::c_char,
    mut stdin_fd: libc::c_int,
    mut is_remote: *mut libc::c_int,
    mut id_ptr: *mut pid_t,
    mut used_stdin: *mut libc::c_int,
) -> libc::c_int {
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn remote_status(
    mut exit_code_ptr: *mut libc::c_int,
    mut signal_ptr: *mut libc::c_int,
    mut coredump_ptr: *mut libc::c_int,
    mut block: libc::c_int,
) -> libc::c_int {
    *__errno_location() = 10 as libc::c_int;
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn block_remote_children() {}
#[no_mangle]
pub unsafe extern "C" fn unblock_remote_children() {}
#[no_mangle]
pub unsafe extern "C" fn remote_kill(
    mut id: pid_t,
    mut sig: libc::c_int,
) -> libc::c_int {
    return -(1 as libc::c_int);
}
