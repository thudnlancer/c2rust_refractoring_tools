use std::time::{SystemTime, UNIX_EPOCH};
use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
pub struct Tm {
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
    pub tm_zone: String,
}

pub type TimeT = i64;
pub type MktimeOffsetT = TimeT;
pub type LongInt = i64;

static MON_YDAY: [[u16; 13]; 2] = [
    [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365],
    [0, 31, 60, 91, 121, 152, 182, 213, 244, 274, 305, 335, 366],
];

fn shr(a: LongInt, b: i32) -> LongInt {
    let one: LongInt = 1;
    if -one >> 1 == -1 {
        a >> b
    } else {
        (a + (a < 0) as i32 as LongInt) / (one << b) - (a < 0) as i32 as LongInt
    }
}

fn leapyear(year: LongInt) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year / 100 % 4 == (-(1900 / 100) & 3) as LongInt)
}

fn isdst_differ(a: i32, b: i32) -> bool {
    (a == 0) != (b == 0) && a >= 0 && b >= 0
}

fn ydhms_diff(
    year1: LongInt,
    yday1: LongInt,
    hour1: i32,
    min1: i32,
    sec1: i32,
    year0: i32,
    yday0: i32,
    hour0: i32,
    min0: i32,
    sec0: i32,
) -> LongInt {
    let a4 = (shr(year1, 2) + shr(1900, 2) - (year1 % 4 == 0) as i32 as LongInt) as i32;
    let b4 = (shr(year0 as LongInt, 2) + shr(1900, 2) - (year0 % 4 == 0) as i32 as LongInt) as i32;
    
    let a100 = (a4 + (a4 < 0) as i32) / 25 - (a4 < 0) as i32;
    let b100 = (b4 + (b4 < 0) as i32) / 25 - (b4 < 0) as i32;
    
    let a400 = shr(a100 as LongInt, 2) as i32;
    let b400 = shr(b100 as LongInt, 2) as i32;
    
    let intervening_leap_days = a4 - b4 - (a100 - b100) + (a400 - b400);
    let years = year1 - year0 as LongInt;
    let days = 365 * years + yday1 - yday0 as LongInt + intervening_leap_days as LongInt;
    let hours = 24 * days + hour1 as LongInt - hour0 as LongInt;
    let minutes = 60 * hours + min1 as LongInt - min0 as LongInt;
    60 * minutes + sec1 as LongInt - sec0 as LongInt
}

fn long_int_avg(a: LongInt, b: LongInt) -> LongInt {
    shr(a, 1) + shr(b, 1) + ((a | b) & 1)
}

fn tm_diff(
    year: LongInt,
    yday: LongInt,
    hour: i32,
    min: i32,
    sec: i32,
    tp: &Tm,
) -> LongInt {
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
    convert: fn(TimeT) -> Option<Tm>,
    t: LongInt,
) -> Option<Tm> {
    convert(t)
}

fn ranged_convert(
    convert: fn(TimeT) -> Option<Tm>,
    t: &mut LongInt,
) -> Option<Tm> {
    let mktime_min = TimeT::MIN;
    let mktime_max = TimeT::MAX;
    
    let t1 = if *t < mktime_min {
        mktime_min
    } else if *t <= mktime_max {
        *t
    } else {
        mktime_max
    };
    
    if let Some(r) = convert_time(convert, t1) {
        *t = t1;
        return Some(r);
    }
    
    None
}

pub fn mktime_internal(
    tp: &mut Tm,
    convert: fn(TimeT) -> Option<Tm>,
    offset: &mut MktimeOffsetT,
) -> Option<TimeT> {
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
    let negative_mon_remainder = (mon_remainder < 0) as i32;
    let mon_years = mon / 12 - negative_mon_remainder;
    let lyear_requested = year_requested as LongInt;
    let year = lyear_requested + mon_years as LongInt;
    
    let mon_yday = MON_YDAY[leapyear(year) as usize][(mon_remainder + 12 * negative_mon_remainder) as usize] as i32 - 1;
    let yday = mon_yday as LongInt + mday as LongInt;
    
    let negative_offset_guess = -(*offset);
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
    let mut tm = Tm {
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
        tm_zone: String::new(),
    };
    
    loop {
        let converted = ranged_convert(convert, &mut t);
        if converted.is_none() {
            return None;
        }
        tm = converted.unwrap();
        
        let dt = tm_diff(year, yday, hour, min, sec, &tm);
        if dt == 0 {
            break;
        }
        
        if t == t1 && t != t2 && (tm.tm_isdst < 0 || 
            (if isdst < 0 {
                dst2 <= (tm.tm_isdst != 0) as i32
            } else {
                (isdst != 0) as i32 != (tm.tm_isdst != 0) as i32
            }) != 0)
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
        let stride = 601200;
        let duration_max = 536454000;
        let delta_bound = duration_max / 2 + stride;
        let mut delta = stride;
        
        while delta < delta_bound {
            for direction in [-1, 1].iter() {
                let mut ot = t + delta * *direction;
                if let Some(otm) = convert_time(convert, ot) {
                    if !isdst_differ(isdst, otm.tm_isdst) {
                        let gt = ot + tm_diff(year, yday, hour, min, sec, &otm);
                        if gt >= TimeT::MIN && gt <= TimeT::MAX {
                            if let Some(new_tm) = convert_time(convert, gt) {
                                t = gt;
                                tm = new_tm;
                                break;
                            }
                        }
                    }
                }
            }
            delta += stride;
        }
    }
    
    *offset = t - t0 - negative_offset_guess;
    
    if sec_requested != tm.tm_sec {
        let sec_adjustment = (sec == 0 && tm.tm_sec == 60) as i32 as LongInt - sec as LongInt + sec_requested as LongInt;
        t += sec_adjustment;
        
        if t < TimeT::MIN || t > TimeT::MAX {
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

pub fn system_time_to_tm(time: TimeT) -> Option<Tm> {
    let duration = UNIX_EPOCH + std::time::Duration::from_secs(time.try_into().ok()?);
    let datetime = chrono::DateTime::<chrono::Utc>::from(duration);
    
    Some(Tm {
        tm_sec: datetime.second() as i32,
        tm_min: datetime.minute() as i32,
        tm_hour: datetime.hour() as i32,
        tm_mday: datetime.day() as i32,
        tm_mon: datetime.month0() as i32,
        tm_year: datetime.year() - 1900,
        tm_wday: datetime.weekday().num_days_from_sunday() as i32,
        tm_yday: datetime.ordinal0() as i32,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: "UTC".to_string(),
    })
}