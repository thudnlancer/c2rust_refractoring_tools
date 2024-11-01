#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    fn glp_assert_(
        expr: *const libc::c_char,
        file: *const libc::c_char,
        line: libc::c_int,
    );
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> libc::c_int;
}
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: libc::c_int,
    pub tz_dsttime: libc::c_int,
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
                b"0.0 <= t && t < 4294967296.0\0" as *const u8 as *const libc::c_char,
                b"env/time.c\0" as *const u8 as *const libc::c_char,
                79 as libc::c_int,
            );
            1 as libc::c_int != 0
        }) as libc::c_int;
    return 1000.0f64 * t;
}
#[no_mangle]
pub unsafe extern "C" fn glp_difftime(
    mut t1: libc::c_double,
    mut t0: libc::c_double,
) -> libc::c_double {
    return (t1 - t0) / 1000.0f64;
}
