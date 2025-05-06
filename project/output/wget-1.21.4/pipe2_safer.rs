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
    fn fd_safer_flag(_: i32, _: i32) -> i32;
    fn close(__fd: i32) -> i32;
    fn rpl_pipe2(fd: *mut i32, flags: i32) -> i32;
    fn __errno_location() -> *mut i32;
}
#[no_mangle]
pub unsafe extern "C" fn pipe2_safer(mut fd: *mut i32, mut flags: i32) -> i32 {
    if rpl_pipe2(fd, flags) == 0 as i32 {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < 2 as i32 {
            *fd.offset(i as isize) = fd_safer_flag(*fd.offset(i as isize), flags);
            if *fd.offset(i as isize) < 0 as i32 {
                let mut e: i32 = *__errno_location();
                close(*fd.offset((1 as i32 - i) as isize));
                *__errno_location() = e;
                return -(1 as i32);
            }
            i += 1;
            i;
        }
        return 0 as i32;
    }
    return -(1 as i32);
}