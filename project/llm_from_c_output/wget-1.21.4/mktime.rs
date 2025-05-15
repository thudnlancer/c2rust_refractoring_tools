use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryFrom;
use std::cmp::{min, max};
use std::ops::{Add, Sub};
use std::i64;

const EPOCH_YEAR: i64 = 1970;
const TM_YEAR_BASE: i64 = 1900;
const SECS_PER_HOUR: i64 = 3600;
const SECS_PER_DAY: i64 = SECS_PER_HOUR * 24;

static MON_YDAY: [[u16; 13]; 2] = [
    [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365],
    [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366],
];

fn leapyear(year: i64) -> bool {
    ((year & 3) == 0) && (year % 100 != 0 || ((year / 100) & 3) == (-(TM_YEAR_BASE / 100) & 3))
}

fn isdst_differ(a: i32, b: i32) -> bool {
    (!a != !b) && (0 <= a) && (0 <= b)
}

fn shr(a: i64, b: i32) -> i64 {
    if (-1 >> 1) == -1 {
        a >> b
    } else {
        (a + (a < 0) as i64) / (1 << b) - (a < 0) as i64
    }
}

fn long_int_avg(a: i64, b: i64) -> i64 {
    shr(a, 1) + shr(b, 1) + ((a | b) & 1)
}

fn ydhms_diff(
    year1: i64,
    yday1: i64,
    hour1: i32,
    min1: i32,
    sec1: i32,
    year0: i32,
    yday0: i32,
    hour0: i32,
    min0: i32,
    sec0: i32,
) -> i64 {
    let a4 = shr(year1, 2) + shr(TM_YEAR_BASE, 2) - !(year1 & 3) as i64;
    let b4 = shr(year0 as i64, 2) + shr(TM_YEAR_BASE, 2) - !(year0 as i64 & 3) as i64;
    let a100 = (a4 + (a4 < 0) as i64) / 25 - (a4 < 0) as i64;
    let b100 = (b4 + (b4 < 0) as i64) / 25 - (b4 < 0) as i64;
    let a400 = shr(a100, 2);
    let b400 = shr(b100, 2);
    let intervening_leap_days = (a4 - b4) - (a100 - b100) + (a400 - b400);

    let years = year1 - year0 as i64;
    let days = 365 * years + yday1 - yday0 as i64 + intervening_leap_days;
    let hours = 24 * days + hour1 as i64 - hour0 as i64;
    let minutes = 60 * hours + min1 as i64 - min0 as i64;
    let seconds = 60 * minutes + sec1 as i64 - sec0 as i64;
    seconds
}

fn tm_diff(
    year: i64,
    yday: i64,
    hour: i32,
    min: i32,
    sec: i32,
    tm: &std::time::Tm,
) -> i64 {
    ydhms_diff(
        year,
        yday,
        hour,
        min,
        sec,
        tm.tm_year,
        tm.tm_yday,
        tm.tm_hour,
        tm.tm_min,
        tm.tm_sec,
    )
}

fn mktime_internal(
    tp: &mut std::time::Tm,
    convert: fn(i64) -> Option<std::time::Tm>,
    offset: &mut i64,
) -> Option<i64> {
    let mut tm = *tp;
    let mut remaining_probes = 6;

    let sec = tm.tm_sec;
    let min = tm.tm_min;
    let hour = tm.tm_hour;
    let mday = tm.tm_mday;
    let mon = tm.tm_mon;
    let year_requested = tm.tm_year;
    let isdst = tm.tm_isdst;

    let mut dst2 = 0;

    let mon_remainder = mon % 12;
    let negative_mon_remainder = mon_remainder < 0;
    let mon_years = mon / 12 - negative_mon_remainder as i32;
    let lyear_requested = year_requested as i64;
    let year = lyear_requested + mon_years as i64;

    let mon_yday = (MON_YDAY[leapyear(year) as usize][(mon_remainder + 12 * negative_mon_remainder as i32) as usize] as i64 - 1;
    let yday = mon_yday + mday as i64;

    let mut negative_offset_guess = 0;
    let sec_requested = sec;

    let t0 = ydhms_diff(
        year,
        yday,
        hour,
        min,
        sec,
        (EPOCH_YEAR - TM_YEAR_BASE) as i32,
        0,
        0,
        0,
        negative_offset_guess,
    );
    let mut t = t0;
    let mut t1 = t0;
    let mut t2 = t0;

    loop {
        let converted = match convert(t) {
            Some(tm) => tm,
            None => return None,
        };
        let dt = tm_diff(year, yday, hour, min, sec, &converted);
        if dt == 0 {
            tm = converted;
            break;
        }

        if t == t1 && t != t2 && (tm.tm_isdst < 0 || 
            (isdst < 0 && dst2 <= (tm.tm_isdst != 0) as i32) || 
            (isdst != 0) != (tm.tm_isdst != 0))
        {
            break;
        }

        remaining_probes -= 1;
        if remaining_probes == 0 {
            return None;
        }

        t1 = t2;
        t2 = t;
        t += dt;
        dst2 = (tm.tm_isdst != 0) as i32;
    }

    if isdst_differ(isdst, tm.tm_isdst) {
        let dst_difference = (isdst == 0) as i32 - (tm.tm_isdst == 0) as i32;
        let stride = 601200;
        let duration_max = 457243209;
        let delta_bound = duration_max / 2 + stride;

        for delta in (stride..delta_bound).step_by(stride) {
            for direction in &[-1, 1] {
                let ot = t + delta * *direction;
                if let Some(otm) = convert(ot) {
                    if !isdst_differ(isdst, otm.tm_isdst) {
                        let gt = ot + tm_diff(year, yday, hour, min, sec, &otm);
                        if let Some(converted) = convert(gt) {
                            t = gt;
                            tm = converted;
                            break;
                        }
                    }
                }
            }
        }

        t += 60 * 60 * dst_difference as i64;
        if let Some(converted) = convert(t) {
            tm = converted;
        } else {
            return None;
        }
    }

    *offset = t - t0 - negative_offset_guess as i64;

    if sec_requested != tm.tm_sec {
        let sec_adjustment = (sec == 0 && tm.tm_sec == 60) as i64;
        let sec_adjustment = sec_adjustment - sec as i64 + sec_requested as i64;
        t += sec_adjustment;
        if let Some(converted) = convert(t) {
            tm = converted;
        } else {
            return None;
        }
    }

    *tp = tm;
    Some(t)
}

pub fn mktime(tp: &mut std::time::Tm) -> Option<i64> {
    let mut localtime_offset = 0;
    mktime_internal(tp, |t| {
        let duration = std::time::Duration::from_secs(t as u64);
        UNIX_EPOCH.add(duration).to_tm()
    }, &mut localtime_offset)
}

pub fn timelocal(tp: &mut std::time::Tm) -> Option<i64> {
    mktime(tp)
}