#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn creat(__file: *const libc::c_char, __mode: mode_t) -> libc::c_int;
    fn fd_safer(_: libc::c_int) -> libc::c_int;
}
pub type __mode_t = libc::c_uint;
pub type mode_t = __mode_t;
#[no_mangle]
pub unsafe extern "C" fn creat_safer(
    mut file: *const libc::c_char,
    mut mode: mode_t,
) -> libc::c_int {
    return fd_safer(creat(file, mode));
}
