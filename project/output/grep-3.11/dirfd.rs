#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]
extern "C" {
    pub type __dirstream;
    fn __errno_location() -> *mut i32;
}
pub type DIR = __dirstream;
#[no_mangle]
pub unsafe extern "C" fn rpl_dirfd(mut dir_p: *mut DIR) -> i32 {
    let mut fd: i32 = -(1 as i32);
    if fd == -(1 as i32) {
        *__errno_location() = 95 as i32;
    }
    return fd;
}