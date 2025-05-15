use std::num::ParseIntError;
use std::str::FromStr;
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StrToLongError {
    Ok,
    Overflow,
    InvalidSuffixChar,
    InvalidSuffixCharWithOverflow,
    Invalid,
}

impl fmt::Display for StrToLongError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StrToLongError::Ok => write!(f, "No error"),
            StrToLongError::Overflow => write!(f, "Value overflow"),
            StrToLongError::InvalidSuffixChar => write!(f, "Invalid suffix character"),
            StrToLongError::InvalidSuffixCharWithOverflow => write!(f, "Invalid suffix character with overflow"),
            StrToLongError::Invalid => write!(f, "Invalid input"),
        }
    }
}

impl Error for StrToLongError {}

fn is_space(c: char) -> bool {
    c.is_whitespace()
}

pub fn xstrtoul(
    s: &str,
    strtol_base: u32,
    valid_suffixes: Option<&str>,
) -> Result<(u64, Option<&str>), StrToLongError> {
    if strtol_base > 36 {
        return Err(StrToLongError::Invalid);
    }

    let s = s.trim_start_matches(is_space);
    if s.starts_with('-') {
        return Err(StrToLongError::Invalid);
    }

    let parse_result = if strtol_base == 0 {
        u64::from_str_radix(s, 10)
    } else {
        u64::from_str_radix(s, strtol_base)
    };

    match parse_result {
        Ok(mut val) => {
            let remaining = s.trim_start_matches(|c: char| c.is_digit(strtol_base));
            if remaining.is_empty() {
                return Ok((val, None));
            }

            if let Some(suffixes) = valid_suffixes {
                if let Some(c) = remaining.chars().next() {
                    if !suffixes.contains(c) {
                        return Err(StrToLongError::InvalidSuffixChar);
                    }

                    let (new_val, overflow) = apply_suffix(val, c, strtol_base)?;
                    val = new_val;
                    let new_remaining = &remaining[1..];

                    if !new_remaining.is_empty() {
                        return Err(if overflow {
                            StrToLongError::InvalidSuffixCharWithOverflow
                        } else {
                            StrToLongError::InvalidSuffixChar
                        });
                    }

                    Ok((val, None))
                } else {
                    Ok((val, None))
                }
            } else {
                Err(StrToLongError::InvalidSuffixChar)
            }
        }
        Err(e) => match e.kind() {
            std::num::IntErrorKind::PosOverflow => Err(StrToLongError::Overflow),
            _ => Err(StrToLongError::Invalid),
        },
    }
}

fn apply_suffix(val: u64, suffix: char, base: u32) -> Result<(u64, bool), StrToLongError> {
    let (scale, power) = match suffix {
        'b' => (512, 1),
        'B' => (1024, 1),
        'c' => (1, 1),
        'E' => (base as u64, 6),
        'G' | 'g' => (base as u64, 3),
        'K' | 'k' => (base as u64, 1),
        'M' | 'm' => (base as u64, 2),
        'P' => (base as u64, 5),
        'T' | 't' => (base as u64, 4),
        'w' => (2, 1),
        'Y' => (base as u64, 8),
        'Z' => (base as u64, 7),
        _ => return Err(StrToLongError::InvalidSuffixChar),
    };

    let mut result = val;
    let mut overflow = false;
    for _ in 0..power {
        if let Some(new_result) = result.checked_mul(scale) {
            result = new_result;
        } else {
            result = u64::MAX;
            overflow = true;
        }
    }

    Ok((result, overflow))
}

fn bkm_scale(x: &mut u64, scale_factor: u64) -> StrToLongError {
    if let Some(new_val) = x.checked_mul(scale_factor) {
        *x = new_val;
        StrToLongError::Ok
    } else {
        *x = u64::MAX;
        StrToLongError::Overflow
    }
}

fn bkm_scale_by_power(x: &mut u64, base: u64, power: u32) -> StrToLongError {
    let mut err = StrToLongError::Ok;
    for _ in 0..power {
        err = match (err, bkm_scale(x, base)) {
            (StrToLongError::Ok, e) => e,
            (e, StrToLongError::Ok) => e,
            _ => StrToLongError::Overflow,
        };
    }
    err
}