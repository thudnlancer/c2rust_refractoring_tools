use std::fmt::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Datelike, Timelike, Weekday};
use libc::{tm, time_t};
use locale_config::Locale;

const TM_YEAR_BASE: i32 = 1900;

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

fn tm_diff(a: &tm, b: &tm) -> i32 {
    let a4 = (a.tm_year >> 2) + (TM_YEAR_BASE >> 2) - !(a.tm_year & 3);
    let b4 = (b.tm_year >> 2) + (TM_YEAR_BASE >> 2) - !(b.tm_year & 3);
    let a100 = (a4 + (a4 < 0) as i32) / 25 - (a4 < 0) as i32;
    let b100 = (b4 + (b4 < 0) as i32) / 25 - (b4 < 0) as i32;
    let a400 = a100 >> 2;
    let b400 = b100 >> 2;
    let intervening_leap_days = (a4 - b4) - (a100 - b100) + (a400 - b400);
    let years = a.tm_year - b.tm_year;
    let days = 365 * years + intervening_leap_days + (a.tm_yday - b.tm_yday);
    60 * (60 * (24 * days + (a.tm_hour - b.tm_hour)) + (a.tm_min - b.tm_min))
        + (a.tm_sec - b.tm_sec)
}

struct StrftimeOutput {
    buf: String,
    maxsize: usize,
    locale: Locale,
}

impl StrftimeOutput {
    fn new(maxsize: usize, locale: Locale) -> Self {
        Self {
            buf: String::with_capacity(maxsize),
            maxsize,
            locale,
        }
    }

    fn add_char(&mut self, c: char) -> Result<(), fmt::Error> {
        if self.buf.len() < self.maxsize {
            self.buf.push(c);
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }

    fn add_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        if self.buf.len() + s.len() <= self.maxsize {
            self.buf.push_str(s);
            Ok(())
        } else {
            Err(fmt::Error)
        }
    }

    fn pad_number(
        &mut self,
        num: i32,
        width: usize,
        pad_char: char,
        always_sign: bool,
    ) -> Result<(), fmt::Error> {
        let sign = if num < 0 {
            self.add_char('-')?;
            -num as u32
        } else if always_sign {
            self.add_char('+')?;
            num as u32
        } else {
            num as u32
        };

        let num_str = sign.to_string();
        let padding = width.saturating_sub(num_str.len());

        for _ in 0..padding {
            self.add_char(pad_char)?;
        }

        self.add_str(&num_str)
    }
}

fn strftime_internal(
    output: &mut StrftimeOutput,
    format: &str,
    tm: &tm,
    upcase: bool,
    yr_spec: i32,
    width: i32,
) -> Result<(), fmt::Error> {
    let mut chars = format.chars().peekable();
    let mut width = width;

    while let Some(c) = chars.next() {
        if c != '%' {
            output.add_char(c)?;
            continue;
        }

        let mut pad_char = ' ';
        let mut modifier = '\0';
        let mut digits = 0;
        let mut subwidth = -1;

        // Parse flags
        while let Some(&next) = chars.peek() {
            match next {
                '_' | '-' | '+' | '0' => {
                    pad_char = chars.next().unwrap();
                }
                '^' => {
                    chars.next();
                    // upcase = true;
                }
                '#' => {
                    chars.next();
                    // change_case = true;
                }
                _ => break,
            }
        }

        // Parse width
        if let Some(&next) = chars.peek() {
            if next.is_ascii_digit() {
                let mut w = 0;
                while let Some(&d) = chars.peek() {
                    if d.is_ascii_digit() {
                        w = w * 10 + d.to_digit(10).unwrap() as i32;
                        chars.next();
                    } else {
                        break;
                    }
                }
                width = w;
            }
        }

        // Parse modifier
        if let Some(&next) = chars.peek() {
            if next == 'E' || next == 'O' {
                modifier = chars.next().unwrap();
            }
        }

        let format_char = chars.next().ok_or(fmt::Error)?;

        match format_char {
            '%' => output.add_char('%')?,
            'a' => {
                let wday = tm.tm_wday;
                if wday >= 0 && wday <= 6 {
                    let abday = match wday {
                        0 => "Sun",
                        1 => "Mon",
                        2 => "Tue",
                        3 => "Wed",
                        4 => "Thu",
                        5 => "Fri",
                        6 => "Sat",
                        _ => unreachable!(),
                    };
                    output.add_str(abday)?;
                } else {
                    output.add_char('?')?;
                }
            }
            'A' => {
                let wday = tm.tm_wday;
                if wday >= 0 && wday <= 6 {
                    let day = match wday {
                        0 => "Sunday",
                        1 => "Monday",
                        2 => "Tuesday",
                        3 => "Wednesday",
                        4 => "Thursday",
                        5 => "Friday",
                        6 => "Saturday",
                        _ => unreachable!(),
                    };
                    output.add_str(day)?;
                } else {
                    output.add_char('?')?;
                }
            }
            'b' | 'h' => {
                let mon = tm.tm_mon;
                if mon >= 0 && mon <= 11 {
                    let abmon = match mon {
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
                        _ => unreachable!(),
                    };
                    output.add_str(abmon)?;
                } else {
                    output.add_char('?')?;
                }
            }
            'B' => {
                let mon = tm.tm_mon;
                if mon >= 0 && mon <= 11 {
                    let month = match mon {
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
                        _ => unreachable!(),
                    };
                    output.add_str(month)?;
                } else {
                    output.add_char('?')?;
                }
            }
            'c' => {
                let dt = chrono::NaiveDateTime::from_timestamp(
                    mktime(tm) as i64,
                    tm.tm_sec as u32,
                );
                output.add_str(&dt.format("%a %b %e %H:%M:%S %Y").to_string())?;
            }
            'C' => {
                let century = (tm.tm_year + TM_YEAR_BASE) / 100;
                output.pad_number(century, 2, pad_char, false)?;
            }
            'd' => output.pad_number(tm.tm_mday, 2, pad_char, false)?,
            'D' => {
                strftime_internal(output, "%m/%d/%y", tm, upcase, yr_spec, -1)?;
            }
            'e' => output.pad_number(tm.tm_mday, 2, ' ', false)?,
            'F' => {
                strftime_internal(
                    output,
                    "%Y-%m-%d",
                    tm,
                    upcase,
                    yr_spec,
                    if width < 0 { 4 } else { width - 6 },
                )?;
            }
            'H' => output.pad_number(tm.tm_hour, 2, pad_char, false)?,
            'I' => {
                let hour12 = if tm.tm_hour % 12 == 0 {
                    12
                } else {
                    tm.tm_hour % 12
                };
                output.pad_number(hour12, 2, pad_char, false)?;
            }
            'j' => output.pad_number(tm.tm_yday + 1, 3, pad_char, false)?,
            'm' => output.pad_number(tm.tm_mon + 1, 2, pad_char, false)?,
            'M' => output.pad_number(tm.tm_min, 2, pad_char, false)?,
            'n' => output.add_char('\n')?,
            'p' => {
                let ampm = if tm.tm_hour < 12 { "AM" } else { "PM" };
                output.add_str(ampm)?;
            }
            'R' => {
                strftime_internal(output, "%H:%M", tm, upcase, yr_spec, -1)?;
            }
            'S' => output.pad_number(tm.tm_sec, 2, pad_char, false)?,
            't' => output.add_char('\t')?,
            'T' => {
                strftime_internal(output, "%H:%M:%S", tm, upcase, yr_spec, -1)?;
            }
            'u' => {
                let wday = (tm.tm_wday + 6) % 7 + 1;
                output.pad_number(wday, 1, pad_char, false)?;
            }
            'U' => {
                let week = (tm.tm_yday - tm.tm_wday + 7) / 7;
                output.pad_number(week, 2, pad_char, false)?;
            }
            'V' => {
                let year = tm.tm_year + (if tm.tm_year < 0 { 1 } else { 0 });
                let days = iso_week_days(tm.tm_yday, tm.tm_wday);
                let week = days / 7 + 1;
                output.pad_number(week, 2, pad_char, false)?;
            }
            'w' => output.pad_number(tm.tm_wday, 1, pad_char, false)?,
            'W' => {
                let week = (tm.tm_yday - (tm.tm_wday + 6) % 7 + 7) / 7;
                output.pad_number(week, 2, pad_char, false)?;
            }
            'x' => {
                let dt = chrono::NaiveDate::from_ymd(
                    tm.tm_year + TM_YEAR_BASE,
                    tm.tm_mon as u32 + 1,
                    tm.tm_mday as u32,
                );
                output.add_str(&dt.format("%m/%d/%y").to_string())?;
            }
            'X' => {
                let dt = chrono::NaiveTime::from_hms(
                    tm.tm_hour as u32,
                    tm.tm_min as u32,
                    tm.tm_sec as u32,
                );
                output.add_str(&dt.format("%H:%M:%S").to_string())?;
            }
            'y' => {
                let yy = (tm.tm_year % 100 + 100) % 100;
                output.pad_number(yy, 2, pad_char, false)?;
            }
            'Y' => {
                let year = tm.tm_year + TM_YEAR_BASE;
                output.pad_number(year, 4, pad_char, false)?;
            }
            'Z' => {
                // Timezone name - simplified for example
                output.add_str("UTC")?;
            }
            'z' => {
                // Timezone offset
                let offset = tm_diff(tm, &gmtime(mktime(tm)));
                let sign = if offset < 0 { '-' } else { '+' };
                let hours = offset.abs() / 3600;
                let minutes = (offset.abs() % 3600) / 60;
                output.add_char(sign)?;
                output.pad_number(hours, 2, '0', false)?;
                output.pad_number(minutes, 2, '0', false)?;
            }
            _ => {
                // Unknown format specifier - output as-is
                output.add_char('%')?;
                if modifier != '\0' {
                    output.add_char(modifier)?;
                }
                output.add_char(format_char)?;
            }
        }
    }

    Ok(())
}

fn mktime(tm: &tm) -> time_t {
    // Simplified implementation - would use system mktime in real code
    unsafe { libc::mktime(tm as *const tm as *mut tm) }
}

fn gmtime(t: time_t) -> tm {
    // Simplified implementation - would use system gmtime in real code
    unsafe { *libc::gmtime(&t) }
}

pub fn strftime(
    s: &mut String,
    maxsize: usize,
    format: &str,
    tm: &tm,
) -> Result<usize, fmt::Error> {
    let mut output = StrftimeOutput::new(maxsize, Locale::current());
    strftime_internal(&mut output, format, tm, false, 0, -1)?;
    *s = output.buf;
    Ok(s.len())
}