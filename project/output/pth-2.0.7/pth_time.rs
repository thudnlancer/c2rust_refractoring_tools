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
    fn usleep(__useconds: __useconds_t) -> i32;
    fn gettimeofday(__tv: *mut timeval, __tz: __timezone_ptr_t) -> i32;
}
pub type __time_t = i64;
pub type __useconds_t = u32;
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
pub type pth_time_t = timeval;
#[no_mangle]
pub static mut __pth_time_zero: pth_time_t = {
    let mut init = timeval {
        tv_sec: 0 as i64,
        tv_usec: 0 as i64,
    };
    init
};
#[no_mangle]
pub unsafe extern "C" fn __pth_time_usleep(mut usec: u64) {
    usleep(usec as u32);
}
#[no_mangle]
pub unsafe extern "C" fn pth_time(mut sec: i64, mut usec: i64) -> pth_time_t {
    let mut tv: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    tv.tv_sec = sec;
    tv.tv_usec = usec;
    return tv;
}
#[no_mangle]
pub unsafe extern "C" fn pth_timeout(mut sec: i64, mut usec: i64) -> pth_time_t {
    let mut tv: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut tvd: pth_time_t = pth_time_t {
        tv_sec: 0,
        tv_usec: 0,
    };
    if (0 as *mut pth_time_t).is_null() {
        gettimeofday(&mut tv, 0 as *mut timezone);
    } else {
        tv.tv_sec = (*(0 as *mut pth_time_t)).tv_sec;
        tv.tv_usec = (*(0 as *mut pth_time_t)).tv_usec;
    }
    tvd.tv_sec = sec;
    tvd.tv_usec = usec;
    tv.tv_sec += tvd.tv_sec;
    tv.tv_usec += tvd.tv_usec;
    if tv.tv_usec > 1000000 as i32 as i64 {
        tv.tv_sec += 1 as i32 as i64;
        tv.tv_usec -= 1000000 as i32 as i64;
    }
    return tv;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_time_cmp(
    mut t1: *mut pth_time_t,
    mut t2: *mut pth_time_t,
) -> i32 {
    let mut rc: i32 = 0;
    rc = ((*t1).tv_sec - (*t2).tv_sec) as i32;
    if rc == 0 as i32 {
        rc = ((*t1).tv_usec - (*t2).tv_usec) as i32;
    }
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_time_div(mut t1: *mut pth_time_t, mut n: i32) {
    let mut q: i64 = 0;
    let mut r: i64 = 0;
    q = (*t1).tv_sec / n as i64;
    r = (*t1).tv_sec % n as i64 * 1000000 as i32 as i64 / n as i64
        + (*t1).tv_usec / n as i64;
    if r > 1000000 as i32 as i64 {
        q += 1 as i32 as i64;
        r -= 1000000 as i32 as i64;
    }
    (*t1).tv_sec = q;
    (*t1).tv_usec = r;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_time_mul(mut t1: *mut pth_time_t, mut n: i32) {
    (*t1).tv_sec *= n as i64;
    (*t1).tv_usec *= n as i64;
    (*t1).tv_sec += (*t1).tv_usec / 1000000 as i32 as i64;
    (*t1).tv_usec = (*t1).tv_usec % 1000000 as i32 as i64;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_time_t2d(mut t: *mut pth_time_t) -> libc::c_double {
    let mut d: libc::c_double = 0.;
    d = ((*t).tv_sec as libc::c_double * 1000000 as i32 as libc::c_double
        + (*t).tv_usec as libc::c_double) / 1000000 as i32 as libc::c_double;
    return d;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_time_t2i(mut t: *mut pth_time_t) -> i32 {
    let mut i: i32 = 0;
    i = (((*t).tv_sec * 1000000 as i32 as i64 + (*t).tv_usec) / 1000000 as i32 as i64)
        as i32;
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn __pth_time_pos(mut t: *mut pth_time_t) -> i32 {
    if (*t).tv_sec > 0 as i32 as i64 && (*t).tv_usec > 0 as i32 as i64 {
        return 1 as i32
    } else {
        return 0 as i32
    };
}