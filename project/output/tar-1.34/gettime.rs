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
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> i32;
}
pub type __time_t = i64;
pub type __clockid_t = i32;
pub type __syscall_slong_t = i64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type clockid_t = __clockid_t;
#[no_mangle]
pub unsafe extern "C" fn gettime(mut ts: *mut timespec) {
    clock_gettime(0 as i32, ts);
}
#[no_mangle]
pub unsafe extern "C" fn current_timespec() -> timespec {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    gettime(&mut ts);
    return ts;
}