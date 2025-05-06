#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn dup_safer_flag(_: i32, _: i32) -> i32;
    fn __errno_location() -> *mut i32;
    fn close(__fd: i32) -> i32;
}
#[no_mangle]
pub unsafe extern "C" fn fd_safer_flag(mut fd: i32, mut flag: i32) -> i32 {
    if 0 as i32 <= fd && fd <= 2 as i32 {
        let mut f: i32 = dup_safer_flag(fd, flag);
        let mut e: i32 = *__errno_location();
        close(fd);
        *__errno_location() = e;
        fd = f;
    }
    return fd;
}