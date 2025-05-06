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
    fn abort() -> !;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timezone {
    pub tz_minuteswest: i32,
    pub tz_dsttime: i32,
}
pub type __timezone_ptr_t = *mut timezone;
#[no_mangle]
pub unsafe extern "C" fn rpl_time(mut tp: *mut time_t) -> time_t {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tt: time_t = 0;
    if gettimeofday(&mut tv, 0 as *mut timezone) < 0 as i32 {
        abort();
    }
    tt = tv.tv_sec;
    if !tp.is_null() {
        *tp = tt;
    }
    return tt;
}