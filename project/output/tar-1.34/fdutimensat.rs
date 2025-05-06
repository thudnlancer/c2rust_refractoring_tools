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
    fn utimensat(
        __fd: i32,
        __path: *const i8,
        __times: *const timespec,
        __flags: i32,
    ) -> i32;
    fn futimens(__fd: i32, __times: *const timespec) -> i32;
    fn __errno_location() -> *mut i32;
}
pub type __time_t = i64;
pub type __syscall_slong_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[no_mangle]
pub unsafe extern "C" fn fdutimensat(
    mut fd: i32,
    mut dir: i32,
    mut file: *const i8,
    mut ts: *const timespec,
    mut atflag: i32,
) -> i32 {
    let mut result: i32 = 1 as i32;
    if 0 as i32 <= fd {
        result = futimens(fd, ts);
    }
    if !file.is_null()
        && (fd < 0 as i32 || result == -(1 as i32) && *__errno_location() == 38 as i32)
    {
        result = utimensat(dir, file, ts, atflag);
    }
    if result == 1 as i32 {
        *__errno_location() = 9 as i32;
        result = -(1 as i32);
    }
    return result;
}