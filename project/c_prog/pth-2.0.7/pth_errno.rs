use ::libc;
#[no_mangle]
pub static mut __pth_errno_storage: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut __pth_errno_flag: libc::c_int = 0 as libc::c_int;
