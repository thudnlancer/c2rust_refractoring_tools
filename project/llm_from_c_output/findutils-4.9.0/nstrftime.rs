use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Datelike, Timelike, Weekday};
use std::fmt;

const TM_YEAR_BASE: i32 = 1900;

struct EraEntry {
    era_name: String,
    era_wname: String,
    era_format: String,
    era_wformat: String,
    start_date: [i32; 3],
    offset: i32,
    absolute_direction: i32,
}

fn __isleap(year: i32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

fn iso_week_days(yday: i32, wday: i32) -> i32 {
    const ISO_WEEK_START_WDAY: i32 = 1; // Monday
    const ISO_WEEK1_WDAY: i32 = 4; // Thursday
    const YDAY_MINIMUM: i32 = -366;

    let big_enough_multiple_of_7 = (-YDAY_MINIMUM / 7 + 2) * 7;
    yday - (yday - wday + ISO_WEEK1_WDAY + big_enough_multiple_of_7) % 7
        + ISO_WEEK1_WDAY - ISO_WEEK_START_WDAY
}

fn tm_diff(a: &chrono::NaiveDateTime, b: &chrono::NaiveDateTime) -> i32 {
    let a4 = (a.year() >> 2) + (TM_YEAR_BASE >> 2) - !(a.year() & 3);
    let b4 = (b.year() >> 2) + (TM_YEAR_BASE >> 2) - !(b.year() & 3);
    let a100 = (a4 + (a4 < 0) as i32) / 25 - (a4 < 0) as i32;
    let b100 = (b4 + (b4 < 0) as i32) / 25 - (b4 < 0) as i32;
    let a400 = a100 >> 2;
    let b400 = b100 >> 2;
    let intervening_leap_days = (a4 - b4) - (a100 - b100) + (a400 - b400);
    let years = a.year() - b.year();
    let days = 365 * years + intervening_leap_days + (a.ordinal() - b.ordinal()) as i32;
    60 * (60 * (24 * days + (a.hour() - b.hour()) as i32 + (a.minute() - b.minute()) as i32)
        + (a.second() - b.second()) as i32
}

fn strftime_internal(
    s: &mut String,
    format: &str,
    tm: &chrono::NaiveDateTime,
    upcase: bool,
    pad: char,
    width: i32,
) -> Result<usize, io::Error> {
    let mut i = 0;
    let mut f = format.chars().peekable();
    let mut buf = String::new();

    while let Some(c) = f.next() {
        if c != '%' {
            s.push(c);
            i += 1;
            continue;
        }

        let percent = c;
        let mut pad = pad;
        let mut modifier = '\0';
        let mut digits = 0;
        let mut to_uppercase = upcase;
        let mut to_lowercase = false;
        let mut change_case = false;

        // Parse flags
        while let Some(&next_c) = f.peek() {
            match next_c {
                '_' | '-' | '+' | '0' => {
                    pad = f.next().unwrap();
                    continue;
                }
                '^' => {
                    to_uppercase = true;
                    f.next();
                    continue;
                }
                '#' => {
                    change_case = true;
                    f.next();
                    continue;
                }
                _ => break,
            }
        }

        // Parse width
        let mut width = width;
        if let Some(&next_c) = f.peek() {
            if next_c.is_ascii_digit() {
                width = 0;
                while let Some(&next_c) = f.peek() {
                    if next_c.is_ascii_digit() {
                        width = width * 10 + (f.next().unwrap() as i32 - '0' as i32);
                    } else {
                        break;
                    }
                }
            }
        }

        // Parse modifier
        if let Some(&next_c) = f.peek() {
            if next_c == 'E' || next_c == 'O' {
                modifier = f.next().unwrap();
            }
        }

        // Handle format specifier
        if let Some(format_char) = f.next() {
            match format_char {
                '%' => {
                    s.push('%');
                    i += 1;
                }
                'a' => {
                    if modifier != '\0' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    if change_case {
                        to_uppercase = true;
                        to_lowercase = false;
                    }
                    let wday = tm.weekday();
                    let abday = match wday {
                        Weekday::Mon => "Mon",
                        Weekday::Tue => "Tue",
                        Weekday::Wed => "Wed",
                        Weekday::Thu => "Thu",
                        Weekday::Fri => "Fri",
                        Weekday::Sat => "Sat",
                        Weekday::Sun => "Sun",
                    };
                    let mut abday = abday.to_string();
                    if to_uppercase {
                        abday = abday.to_uppercase();
                    }
                    s.push_str(&abday);
                    i += abday.len();
                }
                'A' => {
                    if modifier != '\0' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    if change_case {
                        to_uppercase = true;
                        to_lowercase = false;
                    }
                    let wday = tm.weekday();
                    let day = match wday {
                        Weekday::Mon => "Monday",
                        Weekday::Tue => "Tuesday",
                        Weekday::Wed => "Wednesday",
                        Weekday::Thu => "Thursday",
                        Weekday::Fri => "Friday",
                        Weekday::Sat => "Saturday",
                        Weekday::Sun => "Sunday",
                    };
                    let mut day = day.to_string();
                    if to_uppercase {
                        day = day.to_uppercase();
                    }
                    s.push_str(&day);
                    i += day.len();
                }
                'b' | 'h' => {
                    if change_case {
                        to_uppercase = true;
                        to_lowercase = false;
                    }
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let month = tm.month();
                    let abmon = match month {
                        1 => "Jan",
                        2 => "Feb",
                        3 => "Mar",
                        4 => "Apr",
                        5 => "May",
                        6 => "Jun",
                        7 => "Jul",
                        8 => "Aug",
                        9 => "Sep",
                        10 => "Oct",
                        11 => "Nov",
                        12 => "Dec",
                        _ => "?",
                    };
                    let mut abmon = abmon.to_string();
                    if to_uppercase {
                        abmon = abmon.to_uppercase();
                    }
                    s.push_str(&abmon);
                    i += abmon.len();
                }
                'B' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    if change_case {
                        to_uppercase = true;
                        to_lowercase = false;
                    }
                    let month = tm.month();
                    let mon = match month {
                        1 => "January",
                        2 => "February",
                        3 => "March",
                        4 => "April",
                        5 => "May",
                        6 => "June",
                        7 => "July",
                        8 => "August",
                        9 => "September",
                        10 => "October",
                        11 => "November",
                        12 => "December",
                        _ => "?",
                    };
                    let mut mon = mon.to_string();
                    if to_uppercase {
                        mon = mon.to_uppercase();
                    }
                    s.push_str(&mon);
                    i += mon.len();
                }
                'c' => {
                    if modifier == 'O' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let dt_fmt = "%a %b %e %H:%M:%S %Y";
                    let formatted = tm.format(dt_fmt).to_string();
                    s.push_str(&formatted);
                    i += formatted.len();
                }
                'C' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let year = tm.year();
                    let century = (year - 99 * (year < 0) as i32) / 100 + TM_YEAR_BASE / 100;
                    let century_str = format!("{:02}", century);
                    s.push_str(&century_str);
                    i += century_str.len();
                }
                'd' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let day = tm.day();
                    let day_str = format!("{:02}", day);
                    s.push_str(&day_str);
                    i += day_str.len();
                }
                'D' => {
                    if modifier != '\0' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let formatted = tm.format("%m/%d/%y").to_string();
                    s.push_str(&formatted);
                    i += formatted.len();
                }
                'e' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let day = tm.day();
                    let day_str = format!("{:2}", day);
                    s.push_str(&day_str);
                    i += day_str.len();
                }
                'F' => {
                    if modifier != '\0' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let formatted = tm.format("%Y-%m-%d").to_string();
                    s.push_str(&formatted);
                    i += formatted.len();
                }
                'H' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let hour = tm.hour();
                    let hour_str = format!("{:02}", hour);
                    s.push_str(&hour_str);
                    i += hour_str.len();
                }
                'I' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let hour12 = tm.hour() % 12;
                    let hour12 = if hour12 == 0 { 12 } else { hour12 };
                    let hour_str = format!("{:02}", hour12);
                    s.push_str(&hour_str);
                    i += hour_str.len();
                }
                'j' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let yday = tm.ordinal();
                    let yday_str = format!("{:03}", yday);
                    s.push_str(&yday_str);
                    i += yday_str.len();
                }
                'k' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let hour = tm.hour();
                    let hour_str = format!("{:2}", hour);
                    s.push_str(&hour_str);
                    i += hour_str.len();
                }
                'l' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let hour12 = tm.hour() % 12;
                    let hour12 = if hour12 == 0 { 12 } else { hour12 };
                    let hour_str = format!("{:2}", hour12);
                    s.push_str(&hour_str);
                    i += hour_str.len();
                }
                'm' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let month = tm.month();
                    let month_str = format!("{:02}", month);
                    s.push_str(&month_str);
                    i += month_str.len();
                }
                'M' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let minute = tm.minute();
                    let minute_str = format!("{:02}", minute);
                    s.push_str(&minute_str);
                    i += minute_str.len();
                }
                'n' => {
                    s.push('\n');
                    i += 1;
                }
                'p' => {
                    if change_case {
                        to_uppercase = false;
                        to_lowercase = true;
                    }
                    let ampm = if tm.hour() >= 12 { "PM" } else { "AM" };
                    let mut ampm = ampm.to_string();
                    if to_lowercase {
                        ampm = ampm.to_lowercase();
                    }
                    s.push_str(&ampm);
                    i += ampm.len();
                }
                'P' => {
                    to_lowercase = true;
                    let ampm = if tm.hour() >= 12 { "pm" } else { "am" };
                    s.push_str(ampm);
                    i += ampm.len();
                }
                'r' => {
                    let formatted = tm.format("%I:%M:%S %p").to_string();
                    s.push_str(&formatted);
                    i += formatted.len();
                }
                'R' => {
                    let formatted = tm.format("%H:%M").to_string();
                    s.push_str(&formatted);
                    i += formatted.len();
                }
                'S' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let second = tm.second();
                    let second_str = format!("{:02}", second);
                    s.push_str(&second_str);
                    i += second_str.len();
                }
                't' => {
                    s.push('\t');
                    i += 1;
                }
                'T' => {
                    let formatted = tm.format("%H:%M:%S").to_string();
                    s.push_str(&formatted);
                    i += formatted.len();
                }
                'u' => {
                    let wday = tm.weekday().num_days_from_monday() + 1;
                    let wday_str = format!("{}", wday);
                    s.push_str(&wday_str);
                    i += wday_str.len();
                }
                'U' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let yday = tm.ordinal() as i32;
                    let wday = tm.weekday().num_days_from_sunday() as i32;
                    let week = (yday - wday + 7) / 7;
                    let week_str = format!("{:02}", week);
                    s.push_str(&week_str);
                    i += week_str.len();
                }
                'V' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let year = tm.year();
                    let yday = tm.ordinal() as i32;
                    let wday = tm.weekday().num_days_from_monday() as i32;
                    let days = iso_week_days(yday, wday);
                    let week = days / 7 + 1;
                    let week_str = format!("{:02}", week);
                    s.push_str(&week_str);
                    i += week_str.len();
                }
                'w' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let wday = tm.weekday().num_days_from_sunday();
                    let wday_str = format!("{}", wday);
                    s.push_str(&wday_str);
                    i += wday_str.len();
                }
                'W' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let yday = tm.ordinal() as i32;
                    let wday = (tm.weekday().num_days_from_monday() + 1) as i32;
                    let week = (yday - (wday - 1 + 7) % 7 + 7) / 7;
                    let week_str = format!("{:02}", week);
                    s.push_str(&week_str);
                    i += week_str.len();
                }
                'x' => {
                    if modifier == 'O' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let formatted = tm.format("%m/%d/%y").to_string();
                    s.push_str(&formatted);
                    i += formatted.len();
                }
                'X' => {
                    if modifier == 'O' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let formatted = tm.format("%H:%M:%S").to_string();
                    s.push_str(&formatted);
                    i += formatted.len();
                }
                'y' => {
                    if modifier == 'E' {
                        return Err(io::Error::new(io::ErrorKind::InvalidInput, "bad format"));
                    }
                    let year = tm.year() % 100;
                    let year_str = format!("{:02}", year.abs());
                    s.push_str(&year_str);
                    i += year_str.len();
                }
                'Y'