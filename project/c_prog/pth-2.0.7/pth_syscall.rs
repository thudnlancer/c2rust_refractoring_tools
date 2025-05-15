use ::libc;
#[no_mangle]
pub static mut pth_syscall_soft: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut pth_syscall_hard: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn __pth_syscall_init() {}
#[no_mangle]
pub unsafe extern "C" fn __pth_syscall_kill() {}
