#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn rpl_fcntl(fd: libc::c_int, action: libc::c_int, _: ...) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn dup_safer(mut fd: libc::c_int) -> libc::c_int {
    return rpl_fcntl(fd, 0 as libc::c_int, 2 as libc::c_int + 1 as libc::c_int);
}
