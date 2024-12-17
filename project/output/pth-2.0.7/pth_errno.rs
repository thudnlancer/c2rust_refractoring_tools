#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#[no_mangle]
pub static mut __pth_errno_storage: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut __pth_errno_flag: libc::c_int = 0 as libc::c_int;
