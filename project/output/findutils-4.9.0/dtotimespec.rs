#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type __time_t = i64;
pub type __syscall_slong_t = i64;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type C2RustUnnamed = u32;
pub const TIMESPEC_HZ: C2RustUnnamed = 1000000000;
#[inline]
unsafe extern "C" fn make_timespec(mut s: time_t, mut ns: i64) -> timespec {
    let mut r: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    r.tv_sec = s;
    r.tv_nsec = ns;
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn dtotimespec(mut sec: libc::c_double) -> timespec {
    if !((!(if (0 as i32 as time_t) < -(1 as i32) as time_t {
        -(1 as i32) as time_t
    } else {
        (((1 as i32 as time_t)
            << (::core::mem::size_of::<time_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64) * 2 as i32 as i64
            + 1 as i32 as i64
    }) as libc::c_double) < sec)
    {
        return make_timespec(
            !if (0 as i32 as time_t) < -(1 as i32) as time_t {
                -(1 as i32) as time_t
            } else {
                (((1 as i32 as time_t)
                    << (::core::mem::size_of::<time_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64
            },
            0 as i32 as i64,
        )
    } else if !(sec
        < 1.0f64
            + (if (0 as i32 as time_t) < -(1 as i32) as time_t {
                -(1 as i32) as time_t
            } else {
                (((1 as i32 as time_t)
                    << (::core::mem::size_of::<time_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64
            }) as libc::c_double)
    {
        return make_timespec(
            if (0 as i32 as time_t) < -(1 as i32) as time_t {
                -(1 as i32) as time_t
            } else {
                (((1 as i32 as time_t)
                    << (::core::mem::size_of::<time_t>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64
            },
            (TIMESPEC_HZ as i32 - 1 as i32) as i64,
        )
    } else {
        let mut s: time_t = sec as time_t;
        let mut frac: libc::c_double = TIMESPEC_HZ as i32 as libc::c_double
            * (sec - s as libc::c_double);
        let mut ns: i64 = frac as i64;
        ns += ((ns as libc::c_double) < frac) as i32 as i64;
        s += ns / TIMESPEC_HZ as i32 as i64;
        ns %= TIMESPEC_HZ as i32 as i64;
        if ns < 0 as i32 as i64 {
            s -= 1;
            s;
            ns += TIMESPEC_HZ as i32 as i64;
        }
        return make_timespec(s, ns);
    };
}