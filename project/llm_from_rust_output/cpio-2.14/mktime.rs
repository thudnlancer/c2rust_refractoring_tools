use std::time::{SystemTime, UNIX_EPOCH};
use std::mem;
use std::ptr;
use libc::{c_int, c_long, c_char, c_ushort};

pub type time_t = c_long;
pub type mktime_offset_t = time_t;
pub type long_int = c_long;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct tm {
    pub tm_sec: c_int,
    pub tm_min: c_int,
    pub tm_hour: c_int,
    pub tm_mday: c_int,
    pub tm_mon: c_int,
    pub tm_year: c_int,
    pub tm_wday: c_int,
    pub tm_yday: c_int,
    pub tm_isdst: c_int,
    pub tm_gmtoff: c_long,
    pub tm_zone: *const c_char,
}

static mut __mon_yday: [[c_ushort; 13]; 2] = [
    [
        0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365,
    ],
    [
        0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366,
    ],
];

static mut mktime_min: long_int = 0;
static mut mktime_max: long_int = 0;

fn my_tzset() {
    unsafe { libc::tzset(); }
}

fn shr(a: long_int, b: c_int) -> long_int {
    let one: long_int = 1;
    if -one >> 1 == -1 {
        a >> b
    } else {
        (a + (a < 0) as c_int as long_int) / (one << b) - (a < 0) as c_int as long_int
    }
}

fn leapyear(year: long_int) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn isdst_differ(a: c_int, b: c_int) -> bool {
    (a == 0) != (b == 0) && a >= 0 && b >= 0
}

fn ydhms_diff(
    year1: long_int,
    yday1: long_int,
    hour1: c_int,
    min1: c_int,
    sec1: c_int,
    year0: c_int,
    yday0: c_int,
    hour0: c_int,
    min0: c_int,
    sec0: c_int,
) -> long_int {
    let a4 = (shr(year1, 2) + shr(1900, 2) - (year1 % 4 == 0) as long_int) as c_int;
    let b4 = (shr(year0 as long_int, 2) + shr(1900, 2) - (year0 % 4 == 0) as long_int) as c_int;
    let a100 = (a4 + (a4 < 0) as c_int) / 25 - (a4 < 0) as c_int;
    let b100 = (b4 + (b4 < 0) as c_int) / 25 - (b4 < 0) as c_int;
    let a400 = shr(a100 as long_int, 2) as c_int;
    let b400 = shr(b100 as long_int, 2) as c_int;
    let intervening_leap_days = a4 - b4 - (a100 - b100) + (a400 - b400);
    let years = year1 - year0 as long_int;
    let days = 365 * years + yday1 - yday0 as long_int + intervening_leap_days as long_int;
    let hours = 24 * days + hour1 as long_int - hour0 as long_int;
    let minutes = 60 * hours + min1 as long_int - min0 as long_int;
    60 * minutes + sec1 as long_int - sec0 as long_int
}

fn long_int_avg(a: long_int, b: long_int) -> long_int {
    shr(a, 1) + shr(b, 1) + ((a | b) & 1)
}

fn tm_diff(
    year: long_int,
    yday: long_int,
    hour: c_int,
    min: c_int,
    sec: c_int,
    tp: &tm,
) -> long_int {
    ydhms_diff(
        year,
        yday,
        hour,
        min,
        sec,
        tp.tm_year,
        tp.tm_yday,
        tp.tm_hour,
        tp.tm_min,
        tp.tm_sec,
    )
}

fn convert_time(
    convert: unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm,
    t: long_int,
    tm: &mut tm,
) -> Option<&mut tm> {
    let x = t;
    unsafe {
        let result = convert(&x, tm);
        if result.is_null() {
            None
        } else {
            Some(&mut *result)
        }
    }
}

fn ranged_convert(
    convert: unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm,
    t: &mut long_int,
    tp: &mut tm,
) -> Option<&mut tm> {
    let mut t1 = if *t < unsafe { mktime_min } {
        unsafe { mktime_min }
    } else if *t <= unsafe { mktime_max } {
        *t
    } else {
        unsafe { mktime_max }
    };

    if let Some(r) = convert_time(convert, t1, tp) {
        *t = t1;
        return Some(r);
    }

    if unsafe { *libc::__errno_location() } != libc::EOVERFLOW {
        return None;
    }

    let mut bad = t1;
    let mut ok = 0;
    let mut oktm = tm {
        tm_sec: -1,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: ptr::null(),
    };

    loop {
        let mid = long_int_avg(ok, bad);
        if mid == ok || mid == bad {
            break;
        }
        if convert_time(convert, mid, tp).is_some() {
            ok = mid;
            oktm = *tp;
        } else if unsafe { *libc::__errno_location() } != libc::EOVERFLOW {
            return None;
        } else {
            bad = mid;
        }
    }

    if oktm.tm_sec < 0 {
        return None;
    }

    *t = ok;
    *tp = oktm;
    Some(tp)
}

pub fn mktime_internal(
    tp: &mut tm,
    convert: unsafe extern "C" fn(*const time_t, *mut tm) -> *mut tm,
    offset: &mut mktime_offset_t,
) -> Option<time_t> {
    let mut remaining_probes = 6;
    let sec_requested = tp.tm_sec;
    let mut sec = tp.tm_sec.max(0).min(59);
    let min = tp.tm_min;
    let hour = tp.tm_hour;
    let mday = tp.tm_mday;
    let mon = tp.tm_mon;
    let year_requested = tp.tm_year;
    let isdst = tp.tm_isdst;
    let mut dst2 = 0;

    let mon_remainder = mon % 12;
    let negative_mon_remainder = (mon_remainder < 0) as c_int;
    let mon_years = mon / 12 - negative_mon_remainder;
    let year = year_requested as long_int + mon_years as long_int;

    let mon_yday = unsafe {
        __mon_yday[leapyear(year) as usize][(mon_remainder + 12 * negative_mon_remainder) as usize]
    } as c_int - 1;
    let yday = mon_yday as long_int + mday as long_int;

    let off = *offset;
    let negative_offset_guess = 0 - off;
    let t0 = ydhms_diff(
        year,
        yday,
        hour,
        min,
        sec,
        1970 - 1900,
        0,
        0,
        0,
        negative_offset_guess,
    );

    let mut t = t0;
    let mut t1 = t0;
    let mut t2 = t0;
    let mut tm = tm {
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
        tm_zone: ptr::null(),
    };

    loop {
        let tm_ref = ranged_convert(convert, &mut t, &mut tm)?;
        let dt = tm_diff(year, yday, hour, min, sec, tm_ref);

        if dt == 0 {
            break;
        }

        if t == t1 && t != t2 && (tm.tm_isdst < 0 || {
            if isdst < 0 {
                dst2 <= (tm.tm_isdst != 0) as c_int
            } else {
                (isdst != 0) as c_int != (tm.tm_isdst != 0) as c_int
            } != 0
        }) {
            break;
        }

        remaining_probes -= 1;
        if remaining_probes == 0 {
            unsafe { *libc::__errno_location() = libc::EOVERFLOW };
            return None;
        }

        t1 = t2;
        t2 = t;
        t += dt;
        dst2 = (tm.tm_isdst != 0) as c_int;
    }

    if isdst_differ(isdst, tm.tm_isdst) {
        let dst_difference = (isdst == 0) as c_int - (tm.tm_isdst == 0) as c_int;
        let stride = 601200;
        let duration_max = 457243209;
        let delta_bound = duration_max / 2 + stride;
        let mut delta = 0;

        'outer: loop {
            if delta >= delta_bound {
                t += (60 * 60 * dst_difference) as long_int;
                if !(unsafe { mktime_min } <= t && t <= unsafe { mktime_max } && convert_time(convert, t, &mut tm).is_some()) {
                    unsafe { *libc::__errno_location() = libc::EOVERFLOW };
                    return None;
                }
                break;
            }

            delta += stride;
            for direction in [-1, 1].iter() {
                let ot = t + delta * *direction;
                let mut otm = tm;
                if let Some(otm_ref) = ranged_convert(convert, &mut ot, &mut otm) {
                    if !isdst_differ(isdst, otm_ref.tm_isdst) {
                        let gt = ot + tm_diff(year, yday, hour, min, sec, otm_ref);
                        if unsafe { mktime_min } <= gt && gt <= unsafe { mktime_max } {
                            if convert_time(convert, gt, &mut tm).is_some() {
                                t = gt;
                                break 'outer;
                            } else if unsafe { *libc::__errno_location() } != libc::EOVERFLOW {
                                return None;
                            }
                        }
                    }
                }
            }
        }
    }

    *offset = t - t0 - negative_offset_guess;

    if sec_requested != tm.tm_sec {
        let sec_adjustment = ((sec == 0 && tm.tm_sec == 60) as c_int - sec + sec_requested) as long_int;
        t += sec_adjustment;
        if t < unsafe { mktime_min } || t > unsafe { mktime_max } {
            unsafe { *libc::__errno_location() = libc::EOVERFLOW };
            return None;
        }
        if convert_time(convert, t, &mut tm).is_none() {
            return None;
        }
    }

    *tp = tm;
    Some(t)
}

pub fn rpl_mktime(tp: &mut tm) -> Option<time_t> {
    my_tzset();
    static mut localtime_offset: mktime_offset_t = 0;
    unsafe {
        mktime_internal(
            tp,
            libc::localtime_r,
            &mut localtime_offset,
        )
    }
}

unsafe fn run_static_initializers() {
    mktime_min = if !(0 < -1 as time_t) && !(if 0 < -1 as time_t { -1 } else {
        ((1 << (mem::size_of::<time_t>() * 8 - 2)) - 1) * 2 + 1
    }) < !(if 0 < -1 as long_int { -1 } else {
        ((1 << (mem::size_of::<long_int>() * 8 - 2)) - 1) * 2 + 1
    }) {
        !if 0 < -1 as long_int { -1 } else {
            ((1 << (mem::size_of::<long_int>() * 8 - 2)) - 1) * 2 + 1
        }
    } else {
        !if 0 < -1 as time_t { -1 } else {
            ((1 << (mem::size_of::<time_t>() * 8 - 2)) - 1) * 2 + 1
        }
    };

    mktime_max = if (if 0 < -1 as long_int { -1 } else {
        ((1 << (mem::size_of::<long_int>() * 8 - 2)) - 1) * 2 + 1
    }) < (if 0 < -1 as time_t { -1 } else {
        ((1 << (mem::size_of::<time_t>() * 8 - 2)) - 1) * 2 + 1
    }) {
        if 0 < -1 as long_int { -1 } else {
            ((1 << (mem::size_of::<long_int>() * 8 - 2)) - 1) * 2 + 1
        }
    } else if 0 < -1 as time_t { -1 } else {
        ((1 << (mem::size_of::<time_t>() * 8 - 2)) - 1) * 2 + 1
    };
}

#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];