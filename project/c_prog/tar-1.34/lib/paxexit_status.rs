use ::libc;
#[no_mangle]
pub static mut exit_status: libc::c_int = 0 as libc::c_int;
