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
    fn glp_assert_(expr: *const i8, file: *const i8, line: i32);
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> i32;
}
pub type __time_t = i64;
pub type __suseconds_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}
pub type __timezone_ptr_t = *mut timezone;
#[no_mangle]
pub unsafe extern "C" fn glp_time() -> libc::c_double {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut t: libc::c_double = 0.;
    gettimeofday(&mut tv, 0 as *mut timezone);
    t = tv.tv_sec as libc::c_double + tv.tv_usec as libc::c_double / 1e6f64;
    (0.0f64 <= t && t < 4294967296.0f64
        || {
            glp_assert_(
                b"0.0 <= t && t < 4294967296.0\0" as *const u8 as *const i8,
                b"env/time.c\0" as *const u8 as *const i8,
                79 as i32,
            );
            1 as i32 != 0
        }) as i32;
    return 1000.0f64 * t;
}
#[no_mangle]
pub unsafe extern "C" fn glp_difftime(
    mut t1: libc::c_double,
    mut t0: libc::c_double,
) -> libc::c_double {
    return (t1 - t0) / 1000.0f64;
}