#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn dup_safer(_: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fd_safer(mut fd: libc::c_int) -> libc::c_int {
    if 0 as libc::c_int <= fd && fd <= 2 as libc::c_int {
        let mut f: libc::c_int = dup_safer(fd);
        let mut e: libc::c_int = *__errno_location();
        close(fd);
        *__errno_location() = e;
        fd = f;
    }
    return fd;
}
