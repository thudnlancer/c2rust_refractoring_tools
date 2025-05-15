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

pub fn xstrtol(
    s: &str,
    strtol_base: u32,
    valid_suffixes: Option<&str>,
) -> Result<(i64, StrToLongError), StrToLongError> {
    assert!(strtol_base <= 36, "base must be between 0 and 36");

    let s = s.trim();
    if s.starts_with('-') {
        return Err(StrToLongError::Invalid);
    }

    let (num_str, rest) = split_at_first_non_digit(s, strtol_base);
    if num_str.is_empty() {
        if let Some(suffixes) = valid_suffixes {
            if let Some(c) = rest.chars().next() {
                if suffixes.contains(c) {
                    return Ok((1, StrToLongError::Ok));
                }
            }
        }
        return Err(StrToLongError::Invalid);
    }

    let mut value = match i64::from_str_radix(num_str, strtol_base) {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    let mut err = StrToLongError::Ok;
    if let Some(suffixes) = valid_suffixes {
        if let Some(c) = rest.chars().next() {
            if !suffixes.contains(c) {
                return Ok((value, err));
            }

            let (base, power) = match c {
                'b' => (1024, 0),
                'B' => (1024, 1),
                'c' => (1, 0),
                'E' => (1000, 6),
                'G' | 'g' => (1000, 3),
                'k' | 'K' => (1000, 1),
                'M' | 'm' => (1000, 2),
                'P' => (1000, 5),
                'T' | 't' => (1000, 4),
                'w' => (2, 0),
                'Y' => (1000, 8),
                'Z' => (1000, 7),
                _ => return Ok((value, StrToLongError::InvalidSuffixChar)),
            };

            err = bkm_scale_by_power(&mut value, base, power);
            if rest.len() > 1 {
                err = match err {
                    StrToLongError::Ok => StrToLongError::InvalidSuffixChar,
                    _ => StrToLongError::InvalidSuffixCharWithOverflow,
                };
            }
        }
    }

    Ok((value, err))
}

fn split_at_first_non_digit(s: &str, base: u32) -> (&str, &str) {
    let pos = s
        .chars()
        .position(|c| !c.is_digit(base))
        .unwrap_or(s.len());
    s.split_at(pos)
}

fn bkm_scale(x: &mut i64, scale_factor: i64) -> StrToLongError {
    if *x < i64::MIN / scale_factor {
        *x = i64::MIN;
        StrToLongError::Overflow
    } else if *x > i64::MAX / scale_factor {
        *x = i64::MAX;
        StrToLongError::Overflow
    } else {
        *x *= scale_factor;
        StrToLongError::Ok
    }
}

fn bkm_scale_by_power(x: &mut i64, base: i64, power: i32) -> StrToLongError {
    let mut err = StrToLongError::Ok;
    for _ in 0..power {
        err = match (err, bkm_scale(x, base)) {
            (StrToLongError::Ok, e) => e,
            (_, StrToLongError::Overflow) => StrToLongError::Overflow,
            (e, _) => e,
        };
    }
    err
}