use std::str::FromStr;
use std::num::ParseIntError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum StrToIntError {
    Invalid,
    InvalidSuffixChar,
    Overflow,
    InvalidSuffixCharWithOverflow,
    Ok,
}

impl fmt::Display for StrToIntError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StrToIntError::Invalid => write!(f, "Invalid input"),
            StrToIntError::InvalidSuffixChar => write!(f, "Invalid suffix character"),
            StrToIntError::Overflow => write!(f, "Overflow"),
            StrToIntError::InvalidSuffixCharWithOverflow => write!(f, "Invalid suffix character with overflow"),
            StrToIntError::Ok => write!(f, "No error"),
        }
    }
}

pub fn xstrtoimax(
    s: &str,
    base: i32,
    valid_suffixes: Option<&str>,
) -> Result<i64, StrToIntError> {
    if !(0..=36).contains(&base) {
        return Err(StrToIntError::Invalid);
    }

    let s = s.trim_start();
    if s.starts_with('-') {
        return Err(StrToIntError::Invalid);
    }

    let (num_str, suffix) = if let Some(pos) = s.find(|c: char| !c.is_ascii_digit()) {
        s.split_at(pos)
    } else {
        (s, "")
    };

    if num_str.is_empty() {
        if let Some(suffixes) = valid_suffixes {
            if !suffix.is_empty() && suffixes.contains(suffix.chars().next().unwrap()) {
                return Ok(1);
            }
        }
        return Err(StrToIntError::Invalid);
    }

    let value = match i64::from_str_radix(num_str, base as u32) {
        Ok(v) => v,
        Err(e) => match e {
            ParseIntError::PosOverflow => return Err(StrToIntError::Overflow),
            _ => return Err(StrToIntError::Invalid),
        },
    };

    let Some(suffixes) = valid_suffixes else {
        return Ok(value);
    };

    if suffix.is_empty() {
        return Ok(value);
    }

    let mut chars = suffix.chars();
    let Some(first_char) = chars.next() else {
        return Ok(value);
    };

    if !suffixes.contains(first_char) {
        return Err(StrToIntError::InvalidSuffixChar);
    }

    let mut scale_base = 1024;
    let mut suffix_len = 1;

    if suffixes.contains('0') {
        match first_char {
            'E' | 'G' | 'g' | 'k' | 'K' | 'M' | 'm' | 'P' | 'Q' | 'R' | 'T' | 't' | 'Y' | 'Z' => {
                if let Some(next_char) = chars.next() {
                    match next_char {
                        'i' => {
                            if chars.next() == Some('B') {
                                suffix_len += 2;
                            }
                        }
                        'B' | 'D' => {
                            scale_base = 1000;
                            suffix_len += 1;
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    let mut scaled_value = value;
    let mut overflow = StrToIntError::Ok;

    match first_char {
        'b' => overflow = bkm_scale(&mut scaled_value, 512),
        'B' => overflow = bkm_scale(&mut scaled_value, 1024),
        'c' => {}
        'E' => overflow = bkm_scale_by_power(&mut scaled_value, scale_base, 6),
        'G' | 'g' => overflow = bkm_scale_by_power(&mut scaled_value, scale_base, 3),
        'k' | 'K' => overflow = bkm_scale_by_power(&mut scaled_value, scale_base, 1),
        'M' | 'm' => overflow = bkm_scale_by_power(&mut scaled_value, scale_base, 2),
        'P' => overflow = bkm_scale_by_power(&mut scaled_value, scale_base, 5),
        'Q' => overflow = bkm_scale_by_power(&mut scaled_value, scale_base, 10),
        'R' => overflow = bkm_scale_by_power(&mut scaled_value, scale_base, 9),
        'T' | 't' => overflow = bkm_scale_by_power(&mut scaled_value, scale_base, 4),
        'w' => overflow = bkm_scale(&mut scaled_value, 2),
        'Y' => overflow = bkm_scale_by_power(&mut scaled_value, scale_base, 8),
        'Z' => overflow = bkm_scale_by_power(&mut scaled_value, scale_base, 7),
        _ => return Err(StrToIntError::InvalidSuffixChar),
    }

    if chars.next().is_some() {
        return Err(StrToIntError::InvalidSuffixChar);
    }

    if overflow != StrToIntError::Ok {
        return Err(overflow);
    }

    Ok(scaled_value)
}

fn bkm_scale_by_power(x: &mut i64, base: i32, power: i32) -> StrToIntError {
    let mut err = StrToIntError::Ok;
    for _ in 0..power {
        err = combine_errors(err, bkm_scale(x, base));
    }
    err
}

fn bkm_scale(x: &mut i64, scale_factor: i32) -> StrToIntError {
    let scaled = match x.checked_mul(scale_factor as i64) {
        Some(v) => v,
        None => return StrToIntError::Overflow,
    };
    *x = scaled;
    StrToIntError::Ok
}

fn combine_errors(a: StrToIntError, b: StrToIntError) -> StrToIntError {
    match (a, b) {
        (StrToIntError::Ok, e) => e,
        (e, StrToIntError::Ok) => e,
        (StrToIntError::InvalidSuffixChar, StrToIntError::Overflow) => StrToIntError::InvalidSuffixCharWithOverflow,
        (StrToIntError::Overflow, StrToIntError::InvalidSuffixChar) => StrToIntError::InvalidSuffixCharWithOverflow,
        (e, _) => e,
    }
}