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
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn mktime_internal(
        tp: *mut tm,
        func: Option<unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm>,
        offset: *mut mktime_offset_t,
    ) -> time_t;
}
pub type __time_t = i64;
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
pub type mktime_offset_t = time_t;
#[no_mangle]
pub unsafe extern "C" fn rpl_timegm(mut tmp: *mut tm) -> time_t {
    static mut gmtime_offset: mktime_offset_t = 0;
    (*tmp).tm_isdst = 0 as i32;
    return mktime_internal(
        tmp,
        Some(gmtime_r as unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm),
        &mut gmtime_offset,
    );
}