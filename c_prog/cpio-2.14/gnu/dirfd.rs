#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(extern_types)]
extern "C" {
    pub type __dirstream;
    fn __errno_location() -> *mut libc::c_int;
}
pub type DIR = __dirstream;
#[no_mangle]
pub unsafe extern "C" fn rpl_dirfd(mut dir_p: *mut DIR) -> libc::c_int {
    let mut fd: libc::c_int = -(1 as libc::c_int);
    if fd == -(1 as libc::c_int) {
        *__errno_location() = 95 as libc::c_int;
    }
    return fd;
}
