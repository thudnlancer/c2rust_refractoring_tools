use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc, TimeZone, Datelike, Timelike};
use std::fmt::Write;
use std::str::FromStr;
use std::num::ParseIntError;
use std::cmp::Ordering;

const MAX_LENGTH: usize = 256;

struct MPL;

impl MPL {
    fn error(&self, msg: &str) {
        eprintln!("Error: {}", msg);
        std::process::exit(1);
    }
}

fn fn_gmtime(mpl: &MPL) -> f64 {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs_f64(),
        Err(_) => {
            mpl.error("gmtime(); unable to obtain current calendar time");
            0.0
        }
    }
}

static WEEK: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
static MOON: [&str; 12] = [
    "January", "February", "March", "April", "May", "June",
    "July", "August", "September", "October", "November", "December"
];

fn error1(mpl: &MPL, str: &str, s: &str, fmt: &str, f: &str, msg: &str) {
    println!("Input string passed to str2time:\n{}", str);
    println!("{:>1$}", "^", s.len() + 1);
    println!("Format string passed to str2time:\n{}", fmt);
    println!("{:>1$}", "^", f.len() + 1);
    mpl.error(msg);
}

fn fn_str2time(mpl: &MPL, str: &str, fmt: &str) -> f64 {
    let mut year = -1;
    let mut month = -1;
    let mut day = -1;
    let mut hh = -1;
    let mut mm = -1;
    let mut ss = -1;
    let mut zone = i32::MAX;

    let mut s_iter = str.chars().peekable();
    let mut f_iter = fmt.chars().peekable();

    while let Some(f) = f_iter.next() {
        if f == '%' {
            let f_next = f_iter.next().unwrap_or('\0');
            match f_next {
                'b' | 'h' => {
                    if month >= 0 {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "month multiply specified");
                    }
                    while s_iter.peek() == Some(&' ') {
                        s_iter.next();
                    }
                    let mut found = false;
                    for (i, name) in MOON.iter().enumerate() {
                        let mut name_chars = name.chars();
                        let mut matched = true;
                        for _ in 0..3 {
                            if let (Some(s), Some(n)) = (s_iter.peek(), name_chars.next()) {
                                if !s.eq_ignore_ascii_case(&n) {
                                    matched = false;
                                    break;
                                }
                            } else {
                                matched = false;
                                break;
                            }
                            s_iter.next();
                        }
                        if matched {
                            month = (i + 1) as i32;
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "abbreviated month name missing or invalid");
                    }
                }
                'd' => {
                    if day >= 0 {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "day multiply specified");
                    }
                    while s_iter.peek() == Some(&' ') {
                        s_iter.next();
                    }
                    day = parse_number(&mut s_iter, 2).unwrap_or_else(|| {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "day missing or invalid");
                        0
                    });
                    if !(1..=31).contains(&day) {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "day out of range");
                    }
                }
                'H' => {
                    if hh >= 0 {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "hour multiply specified");
                    }
                    while s_iter.peek() == Some(&' ') {
                        s_iter.next();
                    }
                    hh = parse_number(&mut s_iter, 2).unwrap_or_else(|| {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "hour missing or invalid");
                        0
                    });
                    if !(0..=23).contains(&hh) {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "hour out of range");
                    }
                }
                'm' => {
                    if month >= 0 {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "month multiply specified");
                    }
                    while s_iter.peek() == Some(&' ') {
                        s_iter.next();
                    }
                    month = parse_number(&mut s_iter, 2).unwrap_or_else(|| {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "month missing or invalid");
                        0
                    });
                    if !(1..=12).contains(&month) {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "month out of range");
                    }
                }
                'M' => {
                    if mm >= 0 {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "minute multiply specified");
                    }
                    while s_iter.peek() == Some(&' ') {
                        s_iter.next();
                    }
                    mm = parse_number(&mut s_iter, 2).unwrap_or_else(|| {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "minute missing or invalid");
                        0
                    });
                    if !(0..=59).contains(&mm) {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "minute out of range");
                    }
                }
                'S' => {
                    if ss >= 0 {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "second multiply specified");
                    }
                    while s_iter.peek() == Some(&' ') {
                        s_iter.next();
                    }
                    ss = parse_number(&mut s_iter, 2).unwrap_or_else(|| {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "second missing or invalid");
                        0
                    });
                    if !(0..=60).contains(&ss) {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "second out of range");
                    }
                }
                'y' => {
                    if year >= 0 {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "year multiply specified");
                    }
                    while s_iter.peek() == Some(&' ') {
                        s_iter.next();
                    }
                    let yy = parse_number(&mut s_iter, 2).unwrap_or_else(|| {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "year missing or invalid");
                        0
                    });
                    year = if yy >= 69 { 1900 + yy } else { 2000 + yy };
                }
                'Y' => {
                    if year >= 0 {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "year multiply specified");
                    }
                    while s_iter.peek() == Some(&' ') {
                        s_iter.next();
                    }
                    year = parse_number(&mut s_iter, 4).unwrap_or_else(|| {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "year missing or invalid");
                        0
                    });
                    if !(1..=4000).contains(&year) {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "year out of range");
                    }
                }
                'z' => {
                    if zone != i32::MAX {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "time zone offset multiply specified");
                    }
                    while s_iter.peek() == Some(&' ') {
                        s_iter.next();
                    }
                    let mut z = 0;
                    let sign = match s_iter.peek() {
                        Some('Z') => {
                            s_iter.next();
                            zone = 0;
                            continue;
                        }
                        Some('+') => {
                            s_iter.next();
                            1
                        }
                        Some('-') => {
                            s_iter.next();
                            -1
                        }
                        _ => {
                            error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "time zone offset sign missing");
                            0
                        }
                    };
                    let hh = parse_number(&mut s_iter, 2).unwrap_or_else(|| {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "time zone offset value incomplete or invalid");
                        0
                    });
                    if hh > 23 {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "time zone offset value out of range");
                    }
                    if s_iter.peek() == Some(&':') {
                        s_iter.next();
                    }
                    let mm = parse_number(&mut s_iter, 2).unwrap_or(0);
                    if mm > 59 {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "time zone offset value out of range");
                    }
                    zone = sign * (60 * hh + mm);
                }
                '%' => {
                    if s_iter.next() != Some('%') {
                        error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "character mismatch");
                    }
                }
                _ => {
                    error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "invalid conversion specifier");
                }
            }
        } else if f == ' ' {
            while s_iter.peek() == Some(&' ') {
                s_iter.next();
            }
        } else {
            if s_iter.next() != Some(f) {
                error1(mpl, str, &s_iter.as_str(), fmt, &f_iter.as_str(), "character mismatch");
            }
        }
    }

    let year = if year < 0 { 1970 } else { year };
    let month = if month < 0 { 1 } else { month };
    let day = if day < 0 { 1 } else { day };
    let hh = if hh < 0 { 0 } else { hh };
    let mm = if mm < 0 { 0 } else { mm };
    let ss = if ss < 0 { 0 } else { ss };
    let zone = if zone == i32::MAX { 0 } else { zone };

    let dt = Utc.ymd(year, month as u32, day as u32)
        .and_hms(hh as u32, mm as u32, ss as u32);
    let timestamp = dt.timestamp() as f64 - (60.0 * zone as f64);
    timestamp
}

fn parse_number<I: Iterator<Item = char>>(iter: &mut std::iter::Peekable<I>, max_digits: usize) -> Option<i32> {
    let mut num = 0;
    let mut digits = 0;
    while digits < max_digits {
        match iter.peek() {
            Some(c) if c.is_ascii_digit() => {
                num = num * 10 + (iter.next().unwrap().to_digit(10).unwrap() as i32);
                digits += 1;
            }
            _ => break,
        }
    }
    if digits == 0 { None } else { Some(num) }
}

fn error2(mpl: &MPL, fmt: &str, f: &str, msg: &str) {
    println!("Format string passed to time2str:\n{}", fmt);
    println!("{:>1$}", "^", f.len() + 1);
    mpl.error(msg);
}

fn weekday(j: i32) -> i32 {
    (j + 2440588) % 7 + 1
}

fn firstday(year: i32) -> i32 {
    let j = Utc.ymd(year, 1, 1).and_hms(0, 0, 0).timestamp() / 86400;
    let wd = weekday(j);
    match wd {
        1 => j,
        2 => j - 1,
        3 => j - 2,
        4 => j - 3,
        5 => j + 3,
        6 => j + 2,
        7 => j + 1,
        _ => unreachable!(),
    }
}

fn fn_time2str(mpl: &MPL, str: &mut String, t: f64, fmt: &str) {
    if !(-62135596800.0..=64092211199.0).contains(&t) {
        mpl.error(&format!("time2str({:.15},...); argument out of range", t));
    }

    let t = t.round() as i64;
    let dt = DateTime::<Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(t, 0),
        Utc,
    );

    let year = dt.year();
    let month = dt.month() as i32;
    let day = dt.day() as i32;
    let hh = dt.hour() as i32;
    let mm = dt.minute() as i32;
    let ss = dt.second() as i32;
    let j = (t / 86400) as i32;

    let mut buf = String::new();
    let mut f_iter = fmt.chars().peekable();

    while let Some(f) = f_iter.next() {
        if f == '%' {
            let f_next = f_iter.next().unwrap_or('\0');
            match f_next {
                'a' => buf.push_str(&WEEK[(weekday(j) - 1) as usize][..3]),
                'A' => buf.push_str(WEEK[(weekday(j) - 1) as usize]),
                'b' | 'h' => buf.push_str(&MOON[(month - 1) as usize][..3]),
                'B' => buf.push_str(MOON[(month - 1) as usize]),
                'C' => write!(buf, "{:02}", year / 100).unwrap(),
                'd' => write!(buf, "{:02}", day).unwrap(),
                'D' => write!(buf, "{:02}/{:02}/{:02}", month, day, year % 100).unwrap(),
                'e' => write!(buf, "{:2}", day).unwrap(),
                'F' => write!(buf, "{:04}-{:02}-{:02}", year, month, day).unwrap(),
                'g' => {
                    let iso = if j < firstday(year) {
                        year - 1
                    } else if j < firstday(year + 1) {
                        year
                    } else {
                        year + 1
                    };
                    write!(buf, "{:02}", iso % 100).unwrap();
                }
                'G' => {
                    let iso = if j < firstday(year) {
                        year - 1
                    } else if j < firstday(year + 1) {
                        year
                    } else {
                        year + 1
                    };
                    write!(buf, "{:04}", iso).unwrap();
                }
                'H' => write!(buf, "{:02}", hh).unwrap(),
                'I' => write!(buf, "{:02}", if hh == 0 { 12 } else if hh <= 12 { hh } else { hh - 12 }).unwrap(),
                'j' => {
                    let jd = Utc.ymd(year, month as u32, day as u32).and_hms(0, 0, 0).timestamp() / 86400;
                    let jd1 = Utc.ymd(year, 1, 1).and_hms(0, 0, 0).timestamp() / 86400;
                    write!(buf, "{:03}", jd - jd1 + 1).unwrap();
                }
                'k' => write!(buf, "{:2}", hh).unwrap(),
                'l' => write!(buf, "{:2}", if hh == 0 { 12 } else if hh <= 12 { hh } else { hh - 12 }).unwrap(),
                'm' => write!(buf, "{:02}", month).unwrap(),
                'M' => write!(buf, "{:02}", mm).unwrap(),
                'p' => buf.push_str(if hh <= 11 { "AM" } else { "PM" }),
                'P' => buf.push_str(if hh <= 11 { "am" } else { "pm" }),
                'r' => write!(
                    buf,
                    "{:02}:{:02}:{:02} {}",
                    if hh == 0 { 12 } else if hh <= 12 { hh } else { hh - 12 },
                    mm,
                    ss,
                    if hh <= 11 { "AM" } else { "PM" }
                ).unwrap(),
                'R' => write!(buf, "{:02}:{:02}", hh, mm).unwrap(),
                'S' => write!(buf, "{:02}", ss).unwrap(),
                'T' => write!(buf, "{:02}:{:02}:{:02}", hh, mm, ss).unwrap(),
                'u' => write!(buf, "{}", weekday(j)).unwrap(),
                'U' => {
                    let sun = Utc.ymd