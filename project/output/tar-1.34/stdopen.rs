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
    fn open(__file: *const i8, __oflag: i32, _: ...) -> i32;
    fn rpl_fcntl(fd: i32, action: i32, _: ...) -> i32;
    fn close(__fd: i32) -> i32;
    fn __errno_location() -> *mut i32;
}
#[no_mangle]
pub unsafe extern "C" fn stdopen() -> i32 {
    let mut fd: i32 = 0;
    fd = 0 as i32;
    while fd <= 2 as i32 {
        if rpl_fcntl(fd, 1 as i32) < 0 as i32 {
            let mut mode: i32 = if fd == 0 as i32 { 0o1 as i32 } else { 0 as i32 };
            let mut full_fd: i32 = if fd == 0 as i32 {
                open(b"/dev/full\0" as *const u8 as *const i8, mode)
            } else {
                -(1 as i32)
            };
            let mut new_fd: i32 = if full_fd < 0 as i32 {
                open(b"/dev/null\0" as *const u8 as *const i8, mode)
            } else {
                full_fd
            };
            if new_fd < 0 as i32 {
                return *__errno_location();
            }
            if (2 as i32) < new_fd {
                close(new_fd);
                return 0 as i32;
            }
        }
        fd += 1;
        fd;
    }
    return 0 as i32;
}