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
    fn __errno_location() -> *mut i32;
    fn localtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn tzset();
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
pub type long_int = i64;
unsafe extern "C" fn my_tzset() {
    tzset();
}
unsafe extern "C" fn shr(mut a: long_int, mut b: i32) -> long_int {
    let mut one: long_int = 1 as i32 as long_int;
    return if -one >> 1 as i32 == -(1 as i32) as i64 {
        a >> b
    } else {
        (a + (a < 0 as i32 as i64) as i32 as i64) / (one << b)
            - (a < 0 as i32 as i64) as i32 as i64
    };
}
static mut mktime_min: long_int = 0;
static mut mktime_max: long_int = 0;
unsafe extern "C" fn leapyear(mut year: long_int) -> bool {
    return year & 3 as i32 as i64 == 0 as i32 as i64
        && (year % 100 as i32 as i64 != 0 as i32 as i64
            || year / 100 as i32 as i64 & 3 as i32 as i64
                == (-(1900 as i32 / 100 as i32) & 3 as i32) as i64);
}
static mut __mon_yday: [[libc::c_ushort; 13]; 2] = [
    [
        0 as i32 as libc::c_ushort,
        31 as i32 as libc::c_ushort,
        59 as i32 as libc::c_ushort,
        90 as i32 as libc::c_ushort,
        120 as i32 as libc::c_ushort,
        151 as i32 as libc::c_ushort,
        181 as i32 as libc::c_ushort,
        212 as i32 as libc::c_ushort,
        243 as i32 as libc::c_ushort,
        273 as i32 as libc::c_ushort,
        304 as i32 as libc::c_ushort,
        334 as i32 as libc::c_ushort,
        365 as i32 as libc::c_ushort,
    ],
    [
        0 as i32 as libc::c_ushort,
        31 as i32 as libc::c_ushort,
        60 as i32 as libc::c_ushort,
        91 as i32 as libc::c_ushort,
        121 as i32 as libc::c_ushort,
        152 as i32 as libc::c_ushort,
        182 as i32 as libc::c_ushort,
        213 as i32 as libc::c_ushort,
        244 as i32 as libc::c_ushort,
        274 as i32 as libc::c_ushort,
        305 as i32 as libc::c_ushort,
        335 as i32 as libc::c_ushort,
        366 as i32 as libc::c_ushort,
    ],
];
unsafe extern "C" fn isdst_differ(mut a: i32, mut b: i32) -> bool {
    return (a == 0) as i32 != (b == 0) as i32 && 0 as i32 <= a && 0 as i32 <= b;
}
unsafe extern "C" fn ydhms_diff(
    mut year1: long_int,
    mut yday1: long_int,
    mut hour1: i32,
    mut min1: i32,
    mut sec1: i32,
    mut year0: i32,
    mut yday0: i32,
    mut hour0: i32,
    mut min0: i32,
    mut sec0: i32,
) -> long_int {
    let mut a4: i32 = (shr(year1, 2 as i32) + shr(1900 as i32 as long_int, 2 as i32)
        - (year1 & 3 as i32 as i64 == 0) as i32 as i64) as i32;
    let mut b4: i32 = (shr(year0 as long_int, 2 as i32)
        + shr(1900 as i32 as long_int, 2 as i32) - (year0 & 3 as i32 == 0) as i32 as i64)
        as i32;
    let mut a100: i32 = (a4 + (a4 < 0 as i32) as i32) / 25 as i32
        - (a4 < 0 as i32) as i32;
    let mut b100: i32 = (b4 + (b4 < 0 as i32) as i32) / 25 as i32
        - (b4 < 0 as i32) as i32;
    let mut a400: i32 = shr(a100 as long_int, 2 as i32) as i32;
    let mut b400: i32 = shr(b100 as long_int, 2 as i32) as i32;
    let mut intervening_leap_days: i32 = a4 - b4 - (a100 - b100) + (a400 - b400);
    let mut years: long_int = year1 - year0 as i64;
    let mut days: long_int = 365 as i32 as i64 * years + yday1 - yday0 as i64
        + intervening_leap_days as i64;
    let mut hours: long_int = 24 as i32 as i64 * days + hour1 as i64 - hour0 as i64;
    let mut minutes: long_int = 60 as i32 as i64 * hours + min1 as i64 - min0 as i64;
    let mut seconds: long_int = 60 as i32 as i64 * minutes + sec1 as i64 - sec0 as i64;
    return seconds;
}
unsafe extern "C" fn long_int_avg(mut a: long_int, mut b: long_int) -> long_int {
    return shr(a, 1 as i32) + shr(b, 1 as i32) + ((a | b) & 1 as i32 as i64);
}
unsafe extern "C" fn tm_diff(
    mut year: long_int,
    mut yday: long_int,
    mut hour: i32,
    mut min: i32,
    mut sec: i32,
    mut tp: *const tm,
) -> long_int {
    return ydhms_diff(
        year,
        yday,
        hour,
        min,
        sec,
        (*tp).tm_year,
        (*tp).tm_yday,
        (*tp).tm_hour,
        (*tp).tm_min,
        (*tp).tm_sec,
    );
}
unsafe extern "C" fn convert_time(
    mut convert: Option<unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm>,
    mut t: long_int,
    mut tm: *mut tm,
) -> *mut tm {
    let mut x: time_t = t;
    return convert.expect("non-null function pointer")(&mut x, tm);
}
unsafe extern "C" fn ranged_convert(
    mut convert: Option<unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm>,
    mut t: *mut long_int,
    mut tp: *mut tm,
) -> *mut tm {
    let mut t1: long_int = if *t < mktime_min {
        mktime_min
    } else if *t <= mktime_max {
        *t
    } else {
        mktime_max
    };
    let mut r: *mut tm = convert_time(convert, t1, tp);
    if !r.is_null() {
        *t = t1;
        return r;
    }
    if *__errno_location() != 75 as i32 {
        return 0 as *mut tm;
    }
    let mut bad: long_int = t1;
    let mut ok: long_int = 0 as i32 as long_int;
    let mut oktm: tm = tm {
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
    oktm.tm_sec = -(1 as i32);
    loop {
        let mut mid: long_int = long_int_avg(ok, bad);
        if mid == ok || mid == bad {
            break;
        }
        if !(convert_time(convert, mid, tp)).is_null() {
            ok = mid;
            oktm = *tp;
        } else if *__errno_location() != 75 as i32 {
            return 0 as *mut tm
        } else {
            bad = mid;
        }
    }
    if oktm.tm_sec < 0 as i32 {
        return 0 as *mut tm;
    }
    *t = ok;
    *tp = oktm;
    return tp;
}
#[no_mangle]
pub unsafe extern "C" fn mktime_internal(
    mut tp: *mut tm,
    mut convert: Option<unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm>,
    mut offset: *mut mktime_offset_t,
) -> time_t {
    let mut current_block: u64;
    let mut tm: tm = tm {
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
    let mut remaining_probes: i32 = 6 as i32;
    let mut sec: i32 = (*tp).tm_sec;
    let mut min: i32 = (*tp).tm_min;
    let mut hour: i32 = (*tp).tm_hour;
    let mut mday: i32 = (*tp).tm_mday;
    let mut mon: i32 = (*tp).tm_mon;
    let mut year_requested: i32 = (*tp).tm_year;
    let mut isdst: i32 = (*tp).tm_isdst;
    let mut dst2: i32 = 0 as i32;
    let mut mon_remainder: i32 = mon % 12 as i32;
    let mut negative_mon_remainder: i32 = (mon_remainder < 0 as i32) as i32;
    let mut mon_years: i32 = mon / 12 as i32 - negative_mon_remainder;
    let mut lyear_requested: long_int = year_requested as long_int;
    let mut year: long_int = lyear_requested + mon_years as i64;
    let mut mon_yday: i32 = __mon_yday[leapyear(year)
        as usize][(mon_remainder + 12 as i32 * negative_mon_remainder) as usize] as i32
        - 1 as i32;
    let mut lmday: long_int = mday as long_int;
    let mut yday: long_int = mon_yday as i64 + lmday;
    let mut off: mktime_offset_t = *offset;
    let mut negative_offset_guess: i32 = 0;
    let mut sec_requested: i32 = sec;
    if sec < 0 as i32 {
        sec = 0 as i32;
    }
    if (59 as i32) < sec {
        sec = 59 as i32;
    }
    let (fresh0, fresh1) = (0 as i32).overflowing_sub(off);
    *(&mut negative_offset_guess as *mut i32) = fresh0;
    let mut t0: long_int = ydhms_diff(
        year,
        yday,
        hour,
        min,
        sec,
        1970 as i32 - 1900 as i32,
        0 as i32,
        0 as i32,
        0 as i32,
        negative_offset_guess,
    );
    let mut t: long_int = t0;
    let mut t1: long_int = t0;
    let mut t2: long_int = t0;
    loop {
        if (ranged_convert(convert, &mut t, &mut tm)).is_null() {
            return -(1 as i32) as time_t;
        }
        let mut dt: long_int = tm_diff(year, yday, hour, min, sec, &mut tm);
        if dt == 0 as i32 as i64 {
            current_block = 10652014663920648156;
            break;
        }
        if t == t1 && t != t2
            && (tm.tm_isdst < 0 as i32
                || (if isdst < 0 as i32 {
                    (dst2 <= (tm.tm_isdst != 0 as i32) as i32) as i32
                } else {
                    ((isdst != 0 as i32) as i32 != (tm.tm_isdst != 0 as i32) as i32)
                        as i32
                }) != 0)
        {
            current_block = 12482757203295995358;
            break;
        }
        remaining_probes -= 1;
        remaining_probes;
        if remaining_probes == 0 as i32 {
            *__errno_location() = 75 as i32;
            return -(1 as i32) as time_t;
        }
        t1 = t2;
        t2 = t;
        t += dt;
        dst2 = (tm.tm_isdst != 0 as i32) as i32;
    }
    match current_block {
        10652014663920648156 => {
            if isdst_differ(isdst, tm.tm_isdst) {
                let mut dst_difference: i32 = (isdst == 0 as i32) as i32
                    - (tm.tm_isdst == 0 as i32) as i32;
                let mut stride: i32 = 601200 as i32;
                let mut duration_max: i32 = 457243209 as i32;
                let mut delta_bound: i32 = duration_max / 2 as i32 + stride;
                let mut delta: i32 = 0;
                let mut direction: i32 = 0;
                delta = stride;
                's_135: loop {
                    if !(delta < delta_bound) {
                        current_block = 572715077006366937;
                        break;
                    }
                    direction = -(1 as i32);
                    while direction <= 1 as i32 {
                        let mut ot: long_int = 0;
                        let (fresh2, fresh3) = t.overflowing_add(delta * direction);
                        *(&mut ot as *mut long_int) = fresh2;
                        if !fresh3 {
                            let mut otm: tm = tm {
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
                            if (ranged_convert(convert, &mut ot, &mut otm)).is_null() {
                                return -(1 as i32) as time_t;
                            }
                            if !isdst_differ(isdst, otm.tm_isdst) {
                                let mut gt: long_int = ot
                                    + tm_diff(year, yday, hour, min, sec, &mut otm);
                                if mktime_min <= gt && gt <= mktime_max {
                                    if !(convert_time(convert, gt, &mut tm)).is_null() {
                                        t = gt;
                                        current_block = 12482757203295995358;
                                        break 's_135;
                                    } else if *__errno_location() != 75 as i32 {
                                        return -(1 as i32) as time_t
                                    }
                                }
                            }
                        }
                        direction += 2 as i32;
                    }
                    delta += stride;
                }
                match current_block {
                    12482757203295995358 => {}
                    _ => {
                        t += (60 as i32 * 60 as i32 * dst_difference) as i64;
                        if !(mktime_min <= t && t <= mktime_max
                            && !(convert_time(convert, t, &mut tm)).is_null())
                        {
                            *__errno_location() = 75 as i32;
                            return -(1 as i32) as time_t;
                        }
                    }
                }
            }
        }
        _ => {}
    }
    let (fresh4, fresh5) = t.overflowing_sub(t0);
    *offset = fresh4;
    let (fresh6, fresh7) = (*offset).overflowing_sub(negative_offset_guess);
    *offset = fresh6;
    if 1 as i32 != 0 && sec_requested != tm.tm_sec {
        let mut sec_adjustment: long_int = (sec == 0 as i32 && tm.tm_sec == 60 as i32)
            as i32 as long_int;
        sec_adjustment -= sec as i64;
        sec_adjustment += sec_requested as i64;
        let (fresh8, fresh9) = t.overflowing_add(sec_adjustment);
        *(&mut t as *mut long_int) = fresh8;
        if fresh9 as i32 != 0 || !(mktime_min <= t && t <= mktime_max) {
            *__errno_location() = 75 as i32;
            return -(1 as i32) as time_t;
        }
        if (convert_time(convert, t, &mut tm)).is_null() {
            return -(1 as i32) as time_t;
        }
    }
    *tp = tm;
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn rpl_mktime(mut tp: *mut tm) -> time_t {
    my_tzset();
    static mut localtime_offset: mktime_offset_t = 0;
    return mktime_internal(
        tp,
        Some(localtime_r as unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm),
        &mut localtime_offset,
    );
}
unsafe extern "C" fn run_static_initializers() {
    mktime_min = if !((0 as i32 as time_t) < -(1 as i32) as time_t)
        && !(if (0 as i32 as time_t) < -(1 as i32) as time_t {
            -(1 as i32) as time_t
        } else {
            (((1 as i32 as time_t)
                << (::core::mem::size_of::<time_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64) * 2 as i32 as i64
                + 1 as i32 as i64
        })
            < !(if (0 as i32 as long_int) < -(1 as i32) as long_int {
                -(1 as i32) as long_int
            } else {
                (((1 as i32 as long_int)
                    << (::core::mem::size_of::<long_int>() as u64)
                        .wrapping_mul(8 as i32 as u64)
                        .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64)
                    * 2 as i32 as i64 + 1 as i32 as i64
            })
    {
        !if (0 as i32 as long_int) < -(1 as i32) as long_int {
            -(1 as i32) as long_int
        } else {
            (((1 as i32 as long_int)
                << (::core::mem::size_of::<long_int>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64) * 2 as i32 as i64
                + 1 as i32 as i64
        }
    } else {
        !if (0 as i32 as time_t) < -(1 as i32) as time_t {
            -(1 as i32) as time_t
        } else {
            (((1 as i32 as time_t)
                << (::core::mem::size_of::<time_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64) * 2 as i32 as i64
                + 1 as i32 as i64
        }
    };
    mktime_max = if (if (0 as i32 as long_int) < -(1 as i32) as long_int {
        -(1 as i32) as long_int
    } else {
        (((1 as i32 as long_int)
            << (::core::mem::size_of::<long_int>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64) * 2 as i32 as i64
            + 1 as i32 as i64
    })
        < (if (0 as i32 as time_t) < -(1 as i32) as time_t {
            -(1 as i32) as time_t
        } else {
            (((1 as i32 as time_t)
                << (::core::mem::size_of::<time_t>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64) * 2 as i32 as i64
                + 1 as i32 as i64
        })
    {
        if (0 as i32 as long_int) < -(1 as i32) as long_int {
            -(1 as i32) as long_int
        } else {
            (((1 as i32 as long_int)
                << (::core::mem::size_of::<long_int>() as u64)
                    .wrapping_mul(8 as i32 as u64)
                    .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64) * 2 as i32 as i64
                + 1 as i32 as i64
        }
    } else if (0 as i32 as time_t) < -(1 as i32) as time_t {
        -(1 as i32) as time_t
    } else {
        (((1 as i32 as time_t)
            << (::core::mem::size_of::<time_t>() as u64)
                .wrapping_mul(8 as i32 as u64)
                .wrapping_sub(2 as i32 as u64)) - 1 as i32 as i64) * 2 as i32 as i64
            + 1 as i32 as i64
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];