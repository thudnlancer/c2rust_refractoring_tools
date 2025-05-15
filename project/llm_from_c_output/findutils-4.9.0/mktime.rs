use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;
use std::cmp::{min, max};
use std::i64;
use std::mem;

const EPOCH_YEAR: i64 = 1970;
const TM_YEAR_BASE: i64 = 1900;
const LEAP_SECONDS_POSSIBLE: bool = true;

#[derive(Debug, Clone, Copy)]
struct Tm {
    tm_sec: i32,
    tm_min: i32,
    tm_hour: i32,
    tm_mday: i32,
    tm_mon: i32,
    tm_year: i32,
    tm_wday: i32,
    tm_yday: i32,
    tm_isdst: i32,
}

fn leapyear(year: i64) -> bool {
    ((year & 3) == 0 && (year % 100 != 0 || ((year / 100) & 3) == (-(TM_YEAR_BASE / 100) & 3)))
}

const MON_YDAY: [[i32; 13]; 2] = [
    [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365],
    [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366],
];

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
    let b4 = shr(year0 as i64, 2) + shr(TM_YEAR_BASE, 2) - !(year0 & 3) as i64;
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

fn long_int_avg(a: i64, b: i64) -> i64 {
    shr(a, 1) + shr(b, 1) + ((a | b) & 1)
}

fn tm_diff(year: i64, yday: i64, hour: i32, min: i32, sec: i32, tp: &Tm) -> i64 {
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

fn convert_time(convert: fn(i64) -> Option<Tm>, t: i64) -> Option<Tm> {
    convert(t)
}

fn ranged_convert(convert: fn(i64) -> Option<Tm>, t: &mut i64, tp: &mut Tm) -> Option<()> {
    let t1 = if *t < i64::MIN {
        i64::MIN
    } else if *t <= i64::MAX {
        *t
    } else {
        i64::MAX
    };

    if let Some(r) = convert_time(convert, t1) {
        *t = t1;
        *tp = r;
        Some(())
    } else {
        None
    }
}

fn mktime_internal(tp: &mut Tm, convert: fn(i64) -> Option<Tm>, offset: &mut i64) -> Option<i64> {
    let mut tm = *tp;
    let mut remaining_probes = 6;

    let sec = tp.tm_sec;
    let min = tp.tm_min;
    let hour = tp.tm_hour;
    let mday = tp.tm_mday;
    let mon = tp.tm_mon;
    let year_requested = tp.tm_year;
    let isdst = tp.tm_isdst;

    let mut dst2 = 0;

    let mon_remainder = mon % 12;
    let negative_mon_remainder = mon_remainder < 0;
    let mon_years = mon / 12 - negative_mon_remainder as i32;
    let lyear_requested = year_requested as i64;
    let year = lyear_requested + mon_years as i64;

    let mon_yday = MON_YDAY[leapyear(year) as usize][(mon_remainder + 12 * negative_mon_remainder as i32) as usize] - 1;
    let yday = mon_yday as i64 + mday as i64;

    let mut off = *offset;
    let negative_offset_guess = 0i64.wrapping_sub(off);

    let sec_requested = sec;

    let sec = if LEAP_SECONDS_POSSIBLE {
        if sec < 0 {
            0
        } else if sec > 59 {
            59
        } else {
            sec
        }
    } else {
        sec
    };

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
        negative_offset_guess as i32,
    );
    let mut t = t0;
    let mut t1 = t0;
    let mut t2 = t0;

    loop {
        let mut tm_tmp = Tm {
            tm_sec: 0,
            tm_min: 0,
            tm_hour: 0,
            tm_mday: 0,
            tm_mon: 0,
            tm_year: 0,
            tm_wday: 0,
            tm_yday: 0,
            tm_isdst: 0,
        };
        if !ranged_convert(convert, &mut t, &mut tm_tmp).is_some() {
            return None;
        }
        let dt = tm_diff(year, yday, hour, min, sec, &tm_tmp);
        if dt == 0 {
            tm = tm_tmp;
            break;
        }

        if t == t1 && t != t2 && (tm_tmp.tm_isdst < 0 || (isdst < 0 && dst2 <= (tm_tmp.tm_isdst != 0) as i32 || (isdst != 0) != (tm_tmp.tm_isdst != 0)) {
            break;
        }

        remaining_probes -= 1;
        if remaining_probes == 0 {
            return None;
        }

        t1 = t2;
        t2 = t;
        t += dt;
        dst2 = (tm_tmp.tm_isdst != 0) as i32;
    }

    if isdst_differ(isdst, tm.tm_isdst) {
        let stride = 601200;
        let duration_max = 536454000;
        let delta_bound = duration_max / 2 + stride;

        for delta in (stride..delta_bound).step_by(stride) {
            for direction in &[-1, 1] {
                let ot = t + delta * direction;
                if let Some(otm) = convert_time(convert, ot) {
                    if !isdst_differ(isdst, otm.tm_isdst) {
                        let gt = ot + tm_diff(year, yday, hour, min, sec, &otm);
                        if gt >= i64::MIN && gt <= i64::MAX {
                            if let Some(new_tm) = convert_time(convert, gt) {
                                t = gt;
                                tm = new_tm;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    *offset = t.wrapping_sub(t0).wrapping_sub(negative_offset_guess);

    if LEAP_SECONDS_POSSIBLE && sec_requested != tm.tm_sec {
        let sec_adjustment = if sec == 0 && tm.tm_sec == 60 { 1 } else { 0 };
        let sec_adjustment = sec_adjustment - sec + sec_requested;
        t = t.checked_add(sec_adjustment as i64)?;
        if t < i64::MIN || t > i64::MAX {
            return None;
        }
        if let Some(new_tm) = convert_time(convert, t) {
            tm = new_tm;
        } else {
            return None;
        }
    }

    *tp = tm;
    Some(t)
}

fn mktime(tp: &mut Tm) -> Option<i64> {
    let mut localtime_offset = 0;
    mktime_internal(tp, |t| {
        let dur = UNIX_EPOCH + std::time::Duration::from_secs(t as u64);
        SystemTime::now().duration_since(dur).ok().and_then(|d| {
            let secs = d.as_secs();
            let tm = Tm {
                tm_sec: (secs % 60) as i32,
                tm_min: ((secs / 60) % 60) as i32,
                tm_hour: ((secs / 3600) % 24) as i32,
                tm_mday: ((secs / 86400) % 31) as i32 + 1,
                tm_mon: ((secs / 2678400) % 12) as i32,
                tm_year: ((secs / 31536000) as i32) + 70,
                tm_wday: 0,
                tm_yday: 0,
                tm_isdst: 0,
            };
            Some(tm)
        })
    }, &mut localtime_offset)
}