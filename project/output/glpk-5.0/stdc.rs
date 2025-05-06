#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(thread_local)]
extern "C" {
    fn strtok_r(__s: *mut i8, __delim: *const i8, __save_ptr: *mut *mut i8) -> *mut i8;
    fn strerror_r(__errnum: i32, __buf: *mut i8, __buflen: size_t) -> i32;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
}
pub type __time_t = i64;
pub type size_t = u64;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: i32,
    pub tm_min: i32,
    pub tm_hour: i32,
    pub tm_mday: i32,
    pub tm_mon: i32,
    pub tm_year: i32,
    pub tm_wday: i32,
    pub tm_yday: i32,
    pub tm_isdst: i32,
    pub tm_gmtoff: i64,
    pub tm_zone: *const i8,
}
#[no_mangle]
pub unsafe extern "C" fn _glp_xgmtime(mut timer: *const time_t) -> *mut tm {
    #[thread_local]
    static mut result: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const i8,
    };
    gmtime_r(timer, &mut result);
    return &mut result;
}
#[no_mangle]
pub unsafe extern "C" fn _glp_xstrerr(mut errnum: i32) -> *mut i8 {
    #[thread_local]
    static mut s: [i8; 1024] = [0; 1024];
    strerror_r(errnum, s.as_mut_ptr(), ::core::mem::size_of::<[i8; 1024]>() as u64);
    return s.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn _glp_xstrtok(mut s1: *mut i8, mut s2: *const i8) -> *mut i8 {
    #[thread_local]
    static mut ptr: *mut i8 = 0 as *const i8 as *mut i8;
    return strtok_r(s1, s2, &mut ptr);
}