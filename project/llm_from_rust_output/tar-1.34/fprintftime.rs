use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{Datelike, Timelike, Weekday};

pub fn fprintftime(
    s: &mut dyn Write,
    format: &str,
    tp: &SystemTime,
    tz: Option<chrono_tz::Tz>,
    ns: i32,
) -> Result<usize, std::io::Error> {
    __strftime_internal(s, format, tp, false, 0, -1, false, tz, ns)
}

fn __strftime_internal(
    s: &mut dyn Write,
    format: &str,
    tp: &SystemTime,
    upcase: bool,
    yr_spec: i32,
    width: i32,
    tzset_called: bool,
    tz: Option<chrono_tz::Tz>,
    ns: i32,
) -> Result<usize, std::io::Error> {
    let mut i = 0;
    let mut maxsize = usize::MAX;
    let saved_errno = std::io::Error::last_os_error();
    
    let datetime = match tz {
        Some(tz) => tp.with_timezone(&tz),
        None => tp.with_timezone(&chrono::Local),
    };
    
    let mut hour12 = datetime.hour();
    if hour12 > 12 {
        hour12 -= 12;
    } else if hour12 == 0 {
        hour12 = 12;
    }
    
    let zone = datetime.offset().to_string();
    
    let mut f = format.chars().peekable();
    while let Some(c) = f.next() {
        if c != '%' {
            write!(s, "{}", c)?;
            i += 1;
            continue;
        }
        
        let mut pad = 0;
        let mut modifier = 0;
        let mut digits = 0;
        let mut number_value = 0;
        let mut u_number_value = 0;
        let mut negative_number = false;
        let mut always_output_a_sign = false;
        let mut tz_colon_mask = 0;
        let mut subfmt = "";
        let mut buf = [0; 23];
        let mut to_lowcase = false;
        let mut to_uppcase = upcase;
        let mut colons = 0;
        let mut change_case = false;
        let mut format_char = '\0';
        let mut subwidth = 0;
        
        // Parse flags
        loop {
            match f.peek() {
                Some('_') | Some('-') | Some('+') | Some('0') => {
                    pad = f.next().unwrap() as i32;
                }
                Some('^') => {
                    f.next();
                    to_uppcase = true;
                }
                Some('#') => {
                    f.next();
                    change_case = true;
                }
                _ => break,
            }
        }
        
        // Parse width
        let mut width = width;
        while let Some(c) = f.peek() {
            if c.is_digit(10) {
                width = width * 10 + c.to_digit(10).unwrap() as i32;
                f.next();
            } else {
                break;
            }
        }
        
        // Parse modifier
        match f.peek() {
            Some('E') | Some('O') => {
                modifier = f.next().unwrap() as i32;
            }
            _ => {}
        }
        
        format_char = f.next().unwrap();
        
        match format_char {
            '%' => {
                if modifier != 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                write!(s, "%")?;
                i += 1;
            }
            'a' | 'A' => {
                if modifier != 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                if change_case {
                    to_uppcase = true;
                    to_lowcase = false;
                }
                let weekday = datetime.weekday();
                let s = match format_char {
                    'a' => match weekday {
                        Weekday::Mon => "Mon",
                        Weekday::Tue => "Tue",
                        Weekday::Wed => "Wed",
                        Weekday::Thu => "Thu",
                        Weekday::Fri => "Fri",
                        Weekday::Sat => "Sat",
                        Weekday::Sun => "Sun",
                    },
                    'A' => match weekday {
                        Weekday::Mon => "Monday",
                        Weekday::Tue => "Tuesday",
                        Weekday::Wed => "Wednesday",
                        Weekday::Thu => "Thursday",
                        Weekday::Fri => "Friday",
                        Weekday::Sat => "Saturday",
                        Weekday::Sun => "Sunday",
                    },
                    _ => unreachable!(),
                };
                if to_uppcase {
                    write!(s, "{}", s.to_uppercase())?;
                } else if to_lowcase {
                    write!(s, "{}", s.to_lowercase())?;
                } else {
                    write!(s, "{}", s)?;
                }
                i += s.len();
            }
            'b' | 'h' | 'B' => {
                if change_case {
                    to_uppcase = true;
                    to_lowcase = false;
                }
                if modifier == 'E' as i32 && format_char != 'B' {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                let month = datetime.month();
                let s = match format_char {
                    'b' | 'h' => match month {
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
                        _ => unreachable!(),
                    },
                    'B' => match month {
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
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                };
                if to_uppcase {
                    write!(s, "{}", s.to_uppercase())?;
                } else if to_lowcase {
                    write!(s, "{}", s.to_lowercase())?;
                } else {
                    write!(s, "{}", s)?;
                }
                i += s.len();
            }
            'c' => {
                if modifier == 'O' as i32 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                let len = __strftime_internal(
                    s,
                    "%a %b %e %H:%M:%S %Y",
                    tp,
                    to_uppcase,
                    pad,
                    subwidth,
                    tzset_called,
                    tz,
                    ns,
                )?;
                i += len;
            }
            'C' => {
                if modifier == 'E' as i32 {
                    let len = __strftime_internal(
                        s,
                        "%EC",
                        tp,
                        to_uppcase,
                        pad,
                        subwidth,
                        tzset_called,
                        tz,
                        ns,
                    )?;
                    i += len;
                } else {
                    let century = (datetime.year() / 100) % 100;
                    digits = 2;
                    negative_number = datetime.year() < 0;
                    u_number_value = century as u32;
                    always_output_a_sign = pad == '+' as i32 && (99 < u_number_value || digits < width);
                    write_number(
                        s,
                        u_number_value,
                        digits,
                        width,
                        pad,
                        negative_number,
                        always_output_a_sign,
                        to_uppcase,
                        to_lowcase,
                    )?;
                    i += digits as usize;
                }
            }
            'd' | 'e' => {
                if modifier == 'E' as i32 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                digits = 2;
                number_value = datetime.day() as i32;
                if format_char == 'e' && pad == 0 {
                    pad = '_' as i32;
                }
                negative_number = number_value < 0;
                u_number_value = number_value as u32;
                write_number(
                    s,
                    u_number_value,
                    digits,
                    width,
                    pad,
                    negative_number,
                    always_output_a_sign,
                    to_uppcase,
                    to_lowcase,
                )?;
                i += digits as usize;
            }
            'D' => {
                if modifier != 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                let len = __strftime_internal(
                    s,
                    "%m/%d/%y",
                    tp,
                    to_uppcase,
                    pad,
                    subwidth,
                    tzset_called,
                    tz,
                    ns,
                )?;
                i += len;
            }
            'F' => {
                if modifier != 0 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                if pad == 0 && width < 0 {
                    pad = '+' as i32;
                    subwidth = 4;
                } else {
                    subwidth = width - 6;
                    if subwidth < 0 {
                        subwidth = 0;
                    }
                }
                let len = __strftime_internal(
                    s,
                    "%Y-%m-%d",
                    tp,
                    to_uppcase,
                    pad,
                    subwidth,
                    tzset_called,
                    tz,
                    ns,
                )?;
                i += len;
            }
            'g' | 'G' | 'V' => {
                if modifier == 'E' as i32 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                let (iso_year, iso_week, _) = datetime.iso_week();
                match format_char {
                    'g' => {
                        digits = 2;
                        negative_number = false;
                        u_number_value = (iso_year % 100) as u32;
                        write_number(
                            s,
                            u_number_value,
                            digits,
                            width,
                            pad,
                            negative_number,
                            always_output_a_sign,
                            to_uppcase,
                            to_lowcase,
                        )?;
                        i += digits as usize;
                    }
                    'G' => {
                        digits = 4;
                        negative_number = iso_year < 0;
                        u_number_value = iso_year as u32;
                        write_number(
                            s,
                            u_number_value,
                            digits,
                            width,
                            pad,
                            negative_number,
                            always_output_a_sign,
                            to_uppcase,
                            to_lowcase,
                        )?;
                        i += digits as usize;
                    }
                    'V' => {
                        digits = 2;
                        number_value = iso_week as i32;
                        write_number(
                            s,
                            number_value as u32,
                            digits,
                            width,
                            pad,
                            false,
                            always_output_a_sign,
                            to_uppcase,
                            to_lowcase,
                        )?;
                        i += digits as usize;
                    }
                    _ => unreachable!(),
                }
            }
            'H' | 'I' | 'k' | 'l' => {
                if modifier == 'E' as i32 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                digits = 2;
                number_value = match format_char {
                    'H' | 'k' => datetime.hour() as i32,
                    'I' | 'l' => hour12 as i32,
                    _ => unreachable!(),
                };
                if (format_char == 'k' || format_char == 'l') && pad == 0 {
                    pad = '_' as i32;
                }
                negative_number = number_value < 0;
                u_number_value = number_value as u32;
                write_number(
                    s,
                    u_number_value,
                    digits,
                    width,
                    pad,
                    negative_number,
                    always_output_a_sign,
                    to_uppcase,
                    to_lowcase,
                )?;
                i += digits as usize;
            }
            'j' => {
                if modifier == 'E' as i32 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                digits = 3;
                negative_number = datetime.ordinal() < 1;
                u_number_value = datetime.ordinal() as u32;
                write_number(
                    s,
                    u_number_value,
                    digits,
                    width,
                    pad,
                    negative_number,
                    always_output_a_sign,
                    to_uppcase,
                    to_lowcase,
                )?;
                i += digits as usize;
            }
            'm' => {
                if modifier == 'E' as i32 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                digits = 2;
                negative_number = datetime.month() < 1;
                u_number_value = datetime.month() as u32;
                write_number(
                    s,
                    u_number_value,
                    digits,
                    width,
                    pad,
                    negative_number,
                    always_output_a_sign,
                    to_uppcase,
                    to_lowcase,
                )?;
                i += digits as usize;
            }
            'M' => {
                if modifier == 'E' as i32 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                digits = 2;
                number_value = datetime.minute() as i32;
                negative_number = number_value < 0;
                u_number_value = number_value as u32;
                write_number(
                    s,
                    u_number_value,
                    digits,
                    width,
                    pad,
                    negative_number,
                    always_output_a_sign,
                    to_uppcase,
                    to_lowcase,
                )?;
                i += digits as usize;
            }
            'n' => {
                write!(s, "\n")?;
                i += 1;
            }
            'p' | 'P' => {
                if change_case {
                    to_uppcase = false;
                    to_lowcase = true;
                }
                let s = if datetime.hour() < 12 { "AM" } else { "PM" };
                if format_char == 'P' {
                    write!(s, "{}", s.to_lowercase())?;
                } else {
                    write!(s, "{}", s)?;
                }
                i += s.len();
            }
            'r' => {
                let len = __strftime_internal(
                    s,
                    "%I:%M:%S %p",
                    tp,
                    to_uppcase,
                    pad,
                    subwidth,
                    tzset_called,
                    tz,
                    ns,
                )?;
                i += len;
            }
            'R' => {
                let len = __strftime_internal(
                    s,
                    "%H:%M",
                    tp,
                    to_uppcase,
                    pad,
                    subwidth,
                    tzset_called,
                    tz,
                    ns,
                )?;
                i += len;
            }
            'S' => {
                if modifier == 'E' as i32 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                digits = 2;
                number_value = datetime.second() as i32;
                negative_number = number_value < 0;
                u_number_value = number_value as u32;
                write_number(
                    s,
                    u_number_value,
                    digits,
                    width,
                    pad,
                    negative_number,
                    always_output_a_sign,
                    to_uppcase,
                    to_lowcase,
                )?;
                i += digits as usize;
            }
            't' => {
                write!(s, "\t")?;
                i += 1;
            }
            'T' => {
                let len = __strftime_internal(
                    s,
                    "%H:%M:%S",
                    tp,
                    to_uppcase,
                    pad,
                    subwidth,
                    tzset_called,
                    tz,
                    ns,
                )?;
                i += len;
            }
            'u' => {
                digits = 1;
                number_value = datetime.weekday().num_days_from_monday() as i32 + 1;
                negative_number = number_value < 0;
                u_number_value = number_value as u32;
                write_number(
                    s,
                    u_number_value,
                    digits,
                    width,
                    pad,
                    negative_number,
                    always_output_a_sign,
                    to_uppcase,
                    to_lowcase,
                )?;
                i += digits as usize;
            }
            'U' | 'W' => {
                if modifier == 'E' as i32 {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidInput,
                        "invalid modifier",
                    ));
                }
                digits = 2;
                number_value = match format_char {
                    'U' => (datetime.ordinal() - datetime.weekday().num_days_from_sunday() as u32 + 7) / 7,
                    'W' => (datetime.ordinal() - (datetime.weekday().num_days_from_monday() as u32) + 7) / 7,
                    _ => unreachable!(),
                } as i32;
                negative_number = number_value < 0;
                u_number_value = number_value as u32;
                write_number(
                    s,
                    u_number_value,
                    digits,
                    width,
                    pad,
                    negative_number,
                    always_output