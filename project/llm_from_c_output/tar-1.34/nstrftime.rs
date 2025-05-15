use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Datelike, Timelike, Weekday};
use std::fmt;

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
    tm_gmtoff: i32,
    tm_zone: String,
}

impl Tm {
    fn new() -> Self {
        Tm {
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
        }
    }
}

fn strftime(
    s: &mut String,
    format: &str,
    tm: &Tm,
    upcase: bool,
    yr_spec: i32,
    width: i32,
) -> Result<usize, io::Error> {
    let mut i = 0;
    let mut p = s;
    let mut to_uppcase = upcase;
    let mut to_lowcase = false;
    let mut change_case = false;
    let mut pad = ' ';
    let mut modifier = '\0';
    let mut digits = 0;
    let mut number_value = 0;
    let mut u_number_value = 0;
    let mut negative_number = false;
    let mut always_output_a_sign = false;
    let mut tz_colon_mask = 0;
    let mut subfmt = "";
    let mut buf = [0; 20];
    let mut bufp = 0;
    let mut colons = 0;
    let mut format_char = '\0';
    let mut subwidth = -1;

    for c in format.chars() {
        if c != '%' {
            p.push(c);
            i += 1;
            continue;
        }

        // Handle flags
        loop {
            match format.chars().next() {
                Some('_') | Some('-') | Some('+') | Some('0') => {
                    pad = format.chars().next().unwrap();
                    format = &format[1..];
                }
                Some('^') => {
                    to_uppcase = true;
                    format = &format[1..];
                }
                Some('#') => {
                    change_case = true;
                    format = &format[1..];
                }
                _ => break,
            }
        }

        // Handle width
        if format.chars().next().unwrap().is_digit(10) {
            width = 0;
            while let Some(d) = format.chars().next().unwrap().to_digit(10) {
                width = width * 10 + d as i32;
                format = &format[1..];
            }
        }

        // Handle modifier
        match format.chars().next() {
            Some('E') | Some('O') => {
                modifier = format.chars().next().unwrap();
                format = &format[1..];
            }
            _ => modifier = '\0',
        }

        // Handle format character
        format_char = format.chars().next().unwrap();
        format = &format[1..];

        match format_char {
            '%' => {
                if modifier != '\0' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                p.push('%');
                i += 1;
            }
            'a' => {
                if modifier != '\0' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                if change_case {
                    to_uppcase = true;
                    to_lowcase = false;
                }
                let wday = match tm.tm_wday {
                    0 => "Sun",
                    1 => "Mon",
                    2 => "Tue",
                    3 => "Wed",
                    4 => "Thu",
                    5 => "Fri",
                    6 => "Sat",
                    _ => "?",
                };
                p.push_str(wday);
                i += wday.len();
            }
            'A' => {
                if modifier != '\0' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                if change_case {
                    to_uppcase = true;
                    to_lowcase = false;
                }
                let wday = match tm.tm_wday {
                    0 => "Sunday",
                    1 => "Monday",
                    2 => "Tuesday",
                    3 => "Wednesday",
                    4 => "Thursday",
                    5 => "Friday",
                    6 => "Saturday",
                    _ => "?",
                };
                p.push_str(wday);
                i += wday.len();
            }
            'b' | 'h' => {
                if change_case {
                    to_uppcase = true;
                    to_lowcase = false;
                }
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let mon = match tm.tm_mon {
                    0 => "Jan",
                    1 => "Feb",
                    2 => "Mar",
                    3 => "Apr",
                    4 => "May",
                    5 => "Jun",
                    6 => "Jul",
                    7 => "Aug",
                    8 => "Sep",
                    9 => "Oct",
                    10 => "Nov",
                    11 => "Dec",
                    _ => "?",
                };
                p.push_str(mon);
                i += mon.len();
            }
            'B' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                if change_case {
                    to_uppcase = true;
                    to_lowcase = false;
                }
                let mon = match tm.tm_mon {
                    0 => "January",
                    1 => "February",
                    2 => "March",
                    3 => "April",
                    4 => "May",
                    5 => "June",
                    6 => "July",
                    7 => "August",
                    8 => "September",
                    9 => "October",
                    10 => "November",
                    11 => "December",
                    _ => "?",
                };
                p.push_str(mon);
                i += mon.len();
            }
            'c' => {
                if modifier == 'O' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                subfmt = "%a %b %e %H:%M:%S %Y";
                subwidth = -1;
                let len = strftime(p, subfmt, tm, to_uppcase, pad, subwidth)?;
                i += len;
            }
            'C' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let century = (tm.tm_year + 1900) / 100;
                let s = format!("{:02}", century);
                p.push_str(&s);
                i += s.len();
            }
            'd' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let s = format!("{:02}", tm.tm_mday);
                p.push_str(&s);
                i += s.len();
            }
            'D' => {
                if modifier != '\0' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                subfmt = "%m/%d/%y";
                let len = strftime(p, subfmt, tm, to_uppcase, pad, subwidth)?;
                i += len;
            }
            'e' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let s = format!("{:2}", tm.tm_mday);
                p.push_str(&s);
                i += s.len();
            }
            'F' => {
                if modifier != '\0' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                if pad == '\0' && width < 0 {
                    pad = '+';
                    subwidth = 4;
                } else {
                    subwidth = width - 6;
                    if subwidth < 0 {
                        subwidth = 0;
                    }
                }
                subfmt = "%Y-%m-%d";
                let len = strftime(p, subfmt, tm, to_uppcase, pad, subwidth)?;
                i += len;
            }
            'H' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let s = format!("{:02}", tm.tm_hour);
                p.push_str(&s);
                i += s.len();
            }
            'I' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let hour12 = if tm.tm_hour > 12 {
                    tm.tm_hour - 12
                } else if tm.tm_hour == 0 {
                    12
                } else {
                    tm.tm_hour
                };
                let s = format!("{:02}", hour12);
                p.push_str(&s);
                i += s.len();
            }
            'j' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let s = format!("{:03}", tm.tm_yday + 1);
                p.push_str(&s);
                i += s.len();
            }
            'k' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let s = format!("{:2}", tm.tm_hour);
                p.push_str(&s);
                i += s.len();
            }
            'l' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let hour12 = if tm.tm_hour > 12 {
                    tm.tm_hour - 12
                } else if tm.tm_hour == 0 {
                    12
                } else {
                    tm.tm_hour
                };
                let s = format!("{:2}", hour12);
                p.push_str(&s);
                i += s.len();
            }
            'M' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let s = format!("{:02}", tm.tm_min);
                p.push_str(&s);
                i += s.len();
            }
            'm' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let s = format!("{:02}", tm.tm_mon + 1);
                p.push_str(&s);
                i += s.len();
            }
            'n' => {
                p.push('\n');
                i += 1;
            }
            'P' => {
                to_lowcase = true;
                let ampm = if tm.tm_hour > 11 { "pm" } else { "am" };
                p.push_str(ampm);
                i += ampm.len();
            }
            'p' => {
                if change_case {
                    to_uppcase = false;
                    to_lowcase = true;
                }
                let ampm = if tm.tm_hour > 11 { "PM" } else { "AM" };
                p.push_str(ampm);
                i += ampm.len();
            }
            'R' => {
                subfmt = "%H:%M";
                let len = strftime(p, subfmt, tm, to_uppcase, pad, subwidth)?;
                i += len;
            }
            'r' => {
                subfmt = "%I:%M:%S %p";
                let len = strftime(p, subfmt, tm, to_uppcase, pad, subwidth)?;
                i += len;
            }
            'S' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let s = format!("{:02}", tm.tm_sec);
                p.push_str(&s);
                i += s.len();
            }
            'T' => {
                subfmt = "%H:%M:%S";
                let len = strftime(p, subfmt, tm, to_uppcase, pad, subwidth)?;
                i += len;
            }
            't' => {
                p.push('\t');
                i += 1;
            }
            'U' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let week = (tm.tm_yday - tm.tm_wday + 7) / 7;
                let s = format!("{:02}", week);
                p.push_str(&s);
                i += s.len();
            }
            'u' => {
                let wday = (tm.tm_wday - 1 + 7) % 7 + 1;
                p.push_str(&wday.to_string());
                i += 1;
            }
            'V' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let year = tm.tm_year + 1900;
                let mut days = iso_week_days(tm.tm_yday, tm.tm_wday);
                let mut year_adjust = 0;
                if days < 0 {
                    year_adjust = -1;
                    days = iso_week_days(tm.tm_yday + (365 + is_leap(year - 1)), tm.tm_wday);
                } else {
                    let d = iso_week_days(tm.tm_yday - (365 + is_leap(year)), tm.tm_wday);
                    if d >= 0 {
                        year_adjust = 1;
                        days = d;
                    }
                }
                let week = days / 7 + 1;
                let s = format!("{:02}", week);
                p.push_str(&s);
                i += s.len();
            }
            'W' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let week = (tm.tm_yday - (tm.tm_wday - 1 + 7) % 7 + 7) / 7;
                let s = format!("{:02}", week);
                p.push_str(&s);
                i += s.len();
            }
            'w' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                p.push_str(&tm.tm_wday.to_string());
                i += 1;
            }
            'x' => {
                if modifier == 'O' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                subfmt = "%m/%d/%y";
                let len = strftime(p, subfmt, tm, to_uppcase, pad, subwidth)?;
                i += len;
            }
            'X' => {
                if modifier == 'O' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                subfmt = "%H:%M:%S";
                let len = strftime(p, subfmt, tm, to_uppcase, pad, subwidth)?;
                i += len;
            }
            'y' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let yy = (tm.tm_year % 100 + 100) % 100;
                let s = format!("{:02}", yy);
                p.push_str(&s);
                i += s.len();
            }
            'Y' => {
                if modifier == 'E' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                if modifier == 'O' {
                    return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                }
                let s = format!("{:04}", tm.tm_year + 1900);
                p.push_str(&s);
                i += s.len();
            }
            'Z' => {
                if change_case {
                    to_uppcase = false;
                    to_lowcase = true;
                }
                p.push_str(&tm.tm_zone);
                i += tm.tm_zone.len();
            }
            'z' => {
                if tm.tm_isdst < 0 {
                    continue;
                }
                let diff = tm.tm_gmtoff;
                negative_number = diff < 0 || (diff == 0 && tm.tm_zone.starts_with('-'));
                let hour_diff = diff.abs() / 3600;
                let min_diff = (diff.abs() % 3600) / 60;
                let s = if negative_number {
                    format!("-{:02}{:02}", hour_diff, min_diff)
                } else {
                    format!("+{:02}{:02}", hour_diff, min_diff)
                };
                p.push_str(&s);
                i += s.len();
            }
            _ => {
                p.push('%');
                p.push(format_char);
                i += 2;
            }
        }
    }

    Ok(i)
}

fn iso_week_days(yday: i32, wday: i32) -> i32 {
    let big_enough_multiple_of_7 = (-366 / 7 + 2) * 7;
    yday - (yday