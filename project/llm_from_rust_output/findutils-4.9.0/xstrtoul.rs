use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum StrToLongError {
    Ok,
    Overflow,
    InvalidSuffixChar,
    InvalidSuffixCharWithOverflow,
    Invalid,
}

impl From<ParseIntError> for StrToLongError {
    fn from(err: ParseIntError) -> Self {
        if err.kind() == &std::num::IntErrorKind::PosOverflow {
            StrToLongError::Overflow
        } else {
            StrToLongError::Invalid
        }
    }
}

fn bkm_scale(x: &mut u64, scale_factor: i32) -> StrToLongError {
    let scaled = x.checked_mul(scale_factor as u64);
    match scaled {
        Some(v) => {
            *x = v;
            StrToLongError::Ok
        }
        None => {
            *x = u64::MAX;
            StrToLongError::Overflow
        }
    }
}

fn bkm_scale_by_power(x: &mut u64, base: i32, power: i32) -> StrToLongError {
    let mut err = StrToLongError::Ok;
    for _ in 0..power {
        err = combine_errors(err, bkm_scale(x, base));
    }
    err
}

fn combine_errors(a: StrToLongError, b: StrToLongError) -> StrToLongError {
    match (a, b) {
        (StrToLongError::Ok, e) | (e, StrToLongError::Ok) => e,
        (StrToLongError::Overflow, StrToLongError::InvalidSuffixChar) 
        | (StrToLongError::InvalidSuffixChar, StrToLongError::Overflow) => {
            StrToLongError::InvalidSuffixCharWithOverflow
        }
        (StrToLongError::Overflow, _) | (_, StrToLongError::Overflow) => StrToLongError::Overflow,
        (StrToLongError::InvalidSuffixChar, _) | (_, StrToLongError::InvalidSuffixChar) => {
            StrToLongError::InvalidSuffixChar
        }
        _ => StrToLongError::Invalid,
    }
}

pub fn xstrtoul(
    s: &str,
    strtol_base: u32,
    valid_suffixes: Option<&str>,
) -> Result<(u64, StrToLongError), StrToLongError> {
    if !(0..=36).contains(&strtol_base) {
        return Err(StrToLongError::Invalid);
    }

    let s = s.trim();
    if s.starts_with('-') {
        return Err(StrToLongError::Invalid);
    }

    let (num_str, suffix) = split_num_suffix(s, valid_suffixes);
    if num_str.is_empty() {
        if suffix.is_some() {
            return Ok((1, StrToLongError::Ok));
        }
        return Err(StrToLongError::Invalid);
    }

    let mut val = match u64::from_str_radix(num_str, strtol_base) {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    let mut err = StrToLongError::Ok;
    if let Some(suffix) = suffix {
        let (base, power) = match suffix.chars().next() {
            Some('E') => (1000, 6),
            Some('G') | Some('g') => (1000, 3),
            Some('k') | Some('K') => (1000, 1),
            Some('M') | Some('m') => (1000, 2),
            Some('P') => (1000, 5),
            Some('T') | Some('t') => (1000, 4),
            Some('Y') => (1000, 8),
            Some('Z') => (1000, 7),
            Some('b') => (1024, 0),
            Some('B') => (1024, 0),
            Some('c') => return Ok((val, err)),
            Some('w') => (2, 0),
            _ => return Err(StrToLongError::InvalidSuffixChar),
        };

        if power > 0 {
            err = bkm_scale_by_power(&mut val, base, power);
        } else {
            err = bkm_scale(&mut val, base);
        }

        if suffix.len() > 1 {
            if suffix.ends_with("iB") {
                // Valid suffix format
            } else if suffix.ends_with('B') || suffix.ends_with('D') {
                // Valid suffix format
            } else {
                err = combine_errors(err, StrToLongError::InvalidSuffixChar);
            }
        }
    }

    Ok((val, err))
}

fn split_num_suffix<'a>(s: &'a str, valid_suffixes: Option<&str>) -> (&'a str, Option<&'a str>) {
    let mut split_pos = s.len();
    for (i, c) in s.char_indices() {
        if !c.is_ascii_digit() {
            split_pos = i;
            break;
        }
    }

    if split_pos == s.len() {
        return (s, None);
    }

    let (num, suffix) = s.split_at(split_pos);
    if let Some(valid) = valid_suffixes {
        if valid.contains(suffix) {
            return (num, Some(suffix));
        }
    }

    (num, None)
}