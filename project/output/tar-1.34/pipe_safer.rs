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
    fn fd_safer(_: i32) -> i32;
    fn close(__fd: i32) -> i32;
    fn pipe(__pipedes: *mut i32) -> i32;
    fn __errno_location() -> *mut i32;
}
#[no_mangle]
pub unsafe extern "C" fn pipe_safer(mut fd: *mut i32) -> i32 {
    if pipe(fd) == 0 as i32 {
        let mut i: i32 = 0;
        i = 0 as i32;
        while i < 2 as i32 {
            *fd.offset(i as isize) = fd_safer(*fd.offset(i as isize));
            if *fd.offset(i as isize) < 0 as i32 {
                let mut saved_errno: i32 = *__errno_location();
                close(*fd.offset((1 as i32 - i) as isize));
                *__errno_location() = saved_errno;
                return -(1 as i32);
            }
            i += 1;
            i;
        }
        return 0 as i32;
    }
    return -(1 as i32);
}