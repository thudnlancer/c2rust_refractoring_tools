use std::num::Wrapping;
use std::str::FromStr;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
            StrToLongError::Overflow => write!(f, "Overflow occurred"),
            StrToLongError::InvalidSuffixChar => write!(f, "Invalid suffix character"),
            StrToLongError::InvalidSuffixCharWithOverflow => write!(f, "Invalid suffix character with overflow"),
            StrToLongError::Invalid => write!(f, "Invalid input"),
        }
    }
}

impl Error for StrToLongError {}

pub fn xstrtoumax(
    s: &str,
    strtol_base: u32,
    valid_suffixes: Option<&str>,
) -> Result<(u64, Option<&str>), StrToLongError> {
    if strtol_base > 36 {
        return Err(StrToLongError::Invalid);
    }

    let s = s.trim_start();
    if s.starts_with('-') {
        return Err(StrToLongError::Invalid);
    }

    let (num_str, remaining) = split_at_first_non_digit(s, strtol_base);
    if num_str.is_empty() {
        if let Some(suffixes) = valid_suffixes {
            if let Some(c) = s.chars().next() {
                if suffixes.contains(c) {
                    return Ok((1, Some(&s[1..])));
                }
            }
        }
        return Err(StrToLongError::Invalid);
    }

    let num = match u64::from_str_radix(num_str, strtol_base) {
        Ok(n) => n,
        Err(_) => return Err(StrToLongError::Overflow),
    };

    let remaining = if remaining.is_empty() {
        None
    } else {
        Some(remaining)
    };

    if let Some(suffixes) = valid_suffixes {
        if let Some(rem) = remaining {
            if let Some(c) = rem.chars().next() {
                if !suffixes.contains(c) {
                    return Ok((num, remaining));
                }

                let (scaled_num, scale_err) = apply_suffix(num, c, strtol_base);
                let new_remaining = &rem[1..];

                if !new_remaining.is_empty() {
                    return Err(if scale_err {
                        StrToLongError::InvalidSuffixCharWithOverflow
                    } else {
                        StrToLongError::InvalidSuffixChar
                    });
                }

                return Ok((scaled_num, None));
            }
        }
    }

    Ok((num, remaining))
}

fn split_at_first_non_digit(s: &str, base: u32) -> (&str, &str) {
    let mut split_pos = 0;
    for c in s.chars() {
        if !c.is_digit(base) {
            break;
        }
        split_pos += c.len_utf8();
    }
    s.split_at(split_pos)
}

fn apply_suffix(mut num: u64, suffix: char, base: u32) -> (u64, bool) {
    let (scale_factor, power) = match suffix {
        'b' => (512, 1),
        'B' => (1024, 1),
        'c' => (1, 1),
        'E' => (base as u64, 6),
        'G' | 'g' => (base as u64, 3),
        'k' | 'K' => (base as u64, 1),
        'M' | 'm' => (base as u64, 2),
        'P' => (base as u64, 5),
        'T' | 't' => (base as u64, 4),
        'w' => (2, 1),
        'Y' => (base as u64, 8),
        'Z' => (base as u64, 7),
        _ => return (num, false),
    };

    let mut overflow = false;
    for _ in 0..power {
        let (scaled, of) = scale_checked(num, scale_factor);
        num = scaled;
        overflow |= of;
    }
    (num, overflow)
}

fn scale_checked(num: u64, scale: u64) -> (u64, bool) {
    let max = u64::MAX;
    if num > max / scale {
        (max, true)
    } else {
        (num * scale, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        assert_eq!(xstrtoumax("123", 10, None), Ok((123, None)));
        assert_eq!(xstrtoumax("123abc", 10, None), Ok((123, Some("abc"))));
        assert_eq!(xstrtoumax("abc", 10, None), Err(StrToLongError::Invalid));
    }

    #[test]
    fn test_with_suffixes() {
        assert_eq!(
            xstrtoumax("123K", 10, Some("KMG")),
            Ok((123 * 1000, None))
        );
        assert_eq!(
            xstrtoumax("123Kb", 10, Some("KMG")),
            Err(StrToLongError::InvalidSuffixChar)
        );
    }

    #[test]
    fn test_overflow() {
        assert_eq!(
            xstrtoumax(&u64::MAX.to_string(), 10, None),
            Ok((u64::MAX, None))
        );
        assert_eq!(
            xstrtoumax(&format!("{}1", u64::MAX), 10, None),
            Err(StrToLongError::Overflow)
        );
    }
}