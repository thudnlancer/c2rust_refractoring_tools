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

    let mut val = match i64::from_str_radix(num_str, strtol_base) {
        Ok(v) => v,
        Err(e) => match e {
            ParseIntError { .. } if e.kind() == &std::num::IntErrorKind::PosOverflow => {
                return Err(StrToLongError::Overflow)
            }
            _ => return Err(StrToLongError::Invalid),
        },
    };

    let mut err = StrToLongError::Ok;
    if let Some(suffix) = suffix {
        let (base, power) = match suffix.chars().next().unwrap() {
            'b' => (1024, 0.5),
            'B' => (1024, 1),
            'c' => (1, 1),
            'E' => (1000, 6),
            'G' | 'g' => (1000, 3),
            'k' | 'K' => (1000, 1),
            'M' | 'm' => (1000, 2),
            'P' => (1000, 5),
            'T' | 't' => (1000, 4),
            'w' => (2, 1),
            'Y' => (1000, 8),
            'Z' => (1000, 7),
            _ => return Err(StrToLongError::InvalidSuffixChar),
        };

        if let Some(rest) = suffix.get(1..) {
            if !rest.is_empty() {
                if rest == "iB" && suffix.starts_with('E') {
                    // Handle EiB case
                } else if rest == "B" || rest == "D" {
                    // Handle EB/ED case
                } else {
                    return Err(StrToLongError::InvalidSuffixChar);
                }
            }
        }

        let scaled = match base.checked_pow(power as u32) {
            Some(scaling) => match val.checked_mul(scaling) {
                Some(v) => v,
                None => {
                    err = StrToLongError::Overflow;
                    i64::MAX
                }
            },
            None => {
                err = StrToLongError::Overflow;
                i64::MAX
            }
        };
        val = scaled;
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
        if valid.contains(suffix.chars().next().unwrap()) {
            return (num, Some(suffix));
        }
    }

    (num, None)
}