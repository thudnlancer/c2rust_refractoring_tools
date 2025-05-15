/* Convert a `struct tm` to a time_t value.
   Translated from GNU C Library's mktime.c to Rust.
   Original copyright (C) 1993-2023 Free Software Foundation, Inc.
*/

use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;
use std::cmp::{min, max};
use std::i64;
use std::mem;

const EPOCH_YEAR: i64 = 1970;
const TM_YEAR_BASE: i64 = 1900;
const SECS_PER_HOUR: i64 = 3600;
const SECS_PER_DAY: i64 = 86400;

#[derive(Debug, Clone, Copy)]
struct Tm {
    tm_sec: i32,    // Seconds (0-60)
    tm_min: i32,    // Minutes (0-59)
    tm_hour: i32,   // Hours (0-23)
    tm_mday: i32,   // Day of the month (1-31)
    tm_mon: i32,    // Month (0-11)
    tm_year: i64,   // Year - 1900
    tm_wday: i32,   // Day of the week (0-6, Sunday = 0)
    tm_yday: i32,   // Day in the year (0-365, 1 Jan = 0)
    tm_isdst: i32,  // Daylight saving time
}

static MON_YDAY: [[i32; 13]; 2] = [
    // Normal years
    [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365],
    // Leap years
    [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366],
];

fn leapyear(year: i64) -> bool {
    ((year & 3) == 0) && 
    (year % 100 != 0 || ((year / 100) & 3) == (-(TM_YEAR_BASE / 100) & 3))
}

fn isdst_differ(a: i32, b: i32) -> bool {
    (!a != !b) && (a >= 0) && (b >= 0)
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
    year1: i64, yday1: i64, hour1: i32, min1: i32, sec1: i32,
    year0: i64, yday0: i32, hour0: i32, min0: i32, sec0: i32,
) -> i64 {
    // Compute intervening leap days correctly even if year is negative
    let a4 = shr(year1, 2) + shr(TM_YEAR_BASE, 2) - !(year1 & 3) as i64;
    let b4 = shr(year0, 2) + shr(TM_YEAR_BASE, 2) - !(year0 & 3) as i64;
    let a100 = (a4 + (a4 < 0) as i64) / 25 - (a4 < 0) as i64;
    let b100 = (b4 + (b4 < 0) as i64) / 25 - (b4 < 0) as i64;
    let a400 = shr(a100, 2);
    let b400 = shr(b100, 2);
    let intervening_leap_days = (a4 - b4) - (a100 - b100) + (a400 - b400);

    // Compute the desired time without overflow
    let years = year1 - year0;
    let days = 365 * years + (yday1 - yday0 as i64) + intervening_leap_days;
    let hours = 24 * days + (hour1 - hour0) as i64;
    let minutes = 60 * hours + (min1 - min0) as i64;
    let seconds = 60 * minutes + (sec1 - sec0) as i64;
    seconds
}

fn tm_diff(year: i64, yday: i64, hour: i32, min: i32, sec: i32, tm: &Tm) -> i64 {
    ydhms_diff(
        year, yday, hour, min, sec,
        tm.tm_year, tm.tm_yday, tm.tm_hour, tm.tm_min, tm.tm_sec,
    )
}

fn mktime_internal(tp: &mut Tm, localtime_fn: fn(i64) -> Option<Tm>) -> Option<i64> {
    let mut remaining_probes = 6;
    let sec = tp.tm_sec;
    let min = tp.tm_min;
    let hour = tp.tm_hour;
    let mday = tp.tm_mday;
    let mon = tp.tm_mon;
    let year_requested = tp.tm_year;
    let isdst = tp.tm_isdst;

    let mut dst2 = 0;

    // Ensure mon is in range, and set year accordingly
    let mon_remainder = mon % 12;
    let negative_mon_remainder = mon_remainder < 0;
    let mon_years = mon / 12 - negative_mon_remainder as i32;
    let lyear_requested = year_requested;
    let year = lyear_requested + mon_years as i64;

    // Calculate day of year from year, month, and day of month
    let mon_yday = MON_YDAY[leapyear(year) as usize][(mon_remainder + 12 * negative_mon_remainder as i32) as usize] - 1;
    let yday = mon_yday as i64 + mday as i64;

    let mut negative_offset_guess = 0;
    let sec_requested = sec;

    let mut t0 = ydhms_diff(
        year, yday, hour, min, sec,
        EPOCH_YEAR - TM_YEAR_BASE, 0, 0, 0, negative_offset_guess,
    );
    let mut t = t0;
    let mut t1 = t0;
    let mut t2 = t0;

    loop {
        let tm = match localtime_fn(t) {
            Some(tm) => tm,
            None => return None,
        };

        let dt = tm_diff(year, yday, hour, min, sec, &tm);
        if dt == 0 {
            break;
        }

        if t == t1 && t != t2 && (tm.tm_isdst < 0 || 
            (isdst < 0 && dst2 <= (tm.tm_isdst != 0) as i32) ||
            (isdst != 0) != (tm.tm_isdst != 0))
        {
            // Oscillation detected, return closest time
            return Some(t);
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

    // Check if tm_isdst matches requested value
    if isdst_differ(isdst, tm.tm_isdst) {
        let dst_difference = ((isdst == 0) as i32 - (tm.tm_isdst == 0) as i32) as i64;
        let stride = 601200;
        let duration_max = 457243209;
        let delta_bound = duration_max / 2 + stride;

        let mut delta = stride;
        while delta < delta_bound {
            for direction in &[-1, 1] {
                let ot = t + delta * *direction;
                if let Some(otm) = localtime_fn(ot) {
                    if !isdst_differ(isdst, otm.tm_isdst) {
                        let gt = ot + tm_diff(year, yday, hour, min, sec, &otm);
                        if let Some(tm) = localtime_fn(gt) {
                            *tp = tm;
                            return Some(gt);
                        }
                    }
                }
            }
            delta += stride;
        }

        // Assume one-hour DST difference
        t += SECS_PER_HOUR * dst_difference;
        if let Some(tm) = localtime_fn(t) {
            *tp = tm;
            return Some(t);
        }

        return None;
    }

    *tp = tm;
    Some(t)
}

pub fn mktime(tp: &mut Tm) -> Option<i64> {
    // In Rust we don't need to handle timezone setting like in C
    mktime_internal(tp, |t| {
        let duration = UNIX_EPOCH + std::time::Duration::from_secs(t as u64);
        SystemTime::now()
            .duration_since(duration)
            .ok()
            .and_then(|dur| {
                let secs = dur.as_secs();
                // Simplified conversion - real implementation would need proper local time conversion
                Some(Tm {
                    tm_sec: (secs % 60) as i32,
                    tm_min: ((secs / 60) % 60) as i32,
                    tm_hour: ((secs / 3600) % 24) as i32,
                    tm_mday: ((secs / 86400) % 31 + 1) as i32,
                    tm_mon: (((secs / 2_592_000) % 12) as i32), // Approximate
                    tm_year: ((secs / 31_536_000) + 70) as i64, // Since 1970
                    tm_wday: ((secs / 86400 + 4) % 7) as i32,   // 1970-01-01 was Thursday
                    tm_yday: ((secs / 86400) % 365) as i32,
                    tm_isdst: 0,
                })
            })
    })
}