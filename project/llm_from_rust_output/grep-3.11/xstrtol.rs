use std::cmp::Ordering;
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

fn bkm_scale(x: &mut i64, scale_factor: i32) -> StrToLongError {
    let scale_factor = scale_factor as i64;
    let scaled = x.checked_mul(scale_factor);
    
    match scaled {
        Some(v) => {
            *x = v;
            StrToLongError::Ok
        }
        None => {
            *x = if *x < 0 {
                i64::MIN
            } else {
                i64::MAX
            };
            StrToLongError::Overflow
        }
    }
}

fn bkm_scale_by_power(x: &mut i64, base: i32, power: i32) -> StrToLongError {
    let mut err = StrToLongError::Ok;
    for _ in 0..power {
        err = match (err, bkm_scale(x, base)) {
            (StrToLongError::Ok, e) => e,
            (e, StrToLongError::Ok) => e,
            (_, e) => e,
        };
    }
    err
}

pub fn xstrtol(
    s: &str,
    strtol_base: u32,
    valid_suffixes: Option<&str>,
) -> Result<(i64, StrToLongError), StrToLongError> {
    if !(0..=36).contains(&strtol_base) {
        return Err(StrToLongError::Invalid);
    }

    let s = s.trim();
    if s.is_empty() {
        return Err(StrToLongError::Invalid);
    }

    if s.starts_with('-') && (i64::MIN..0).contains(&0) {
        return Err(StrToLongError::Invalid);
    }

    let (num_str, suffix) = if let Some(suffixes) = valid_suffixes {
        let mut split_pos = 0;
        for (i, c) in s.char_indices() {
            if !suffixes.contains(c) {
                split_pos = i;
                break;
            }
        }
        s.split_at(split_pos)
    } else {
        (s, "")
    };

    let mut val: i64 = if num_str.is_empty() {
        if suffix.is_empty() {
            return Err(StrToLongError::Invalid);
        }
        1
    } else {
        i64::from_str_radix(num_str, strtol_base)?
    };

    let mut err = StrToLongError::Ok;

    if !suffix.is_empty() {
        let mut chars = suffix.chars();
        let first_char = chars.next().unwrap();
        
        let (base, power) = match first_char {
            'b' => (1024, 0),
            'B' => (1024, 0),
            'c' => (1, 0),
            'E' => (1000, 6),
            'G' | 'g' => (1000, 3),
            'k' | 'K' => (1000, 1),
            'M' | 'm' => (1000, 2),
            'P' => (1000, 5),
            'Q' => (1000, 10),
            'R' => (1000, 9),
            'T' | 't' => (1000, 4),
            'w' => (2, 0),
            'Y' => (1000, 8),
            'Z' => (1000, 7),
            _ => return Err(StrToLongError::InvalidSuffixChar),
        };

        err = bkm_scale_by_power(&mut val, base, power);

        if chars.next().is_some() {
            err = match err {
                StrToLongError::Ok => StrToLongError::InvalidSuffixChar,
                StrToLongError::Overflow => StrToLongError::InvalidSuffixCharWithOverflow,
                e => e,
            };
        }
    }

    Ok((val, err))
}