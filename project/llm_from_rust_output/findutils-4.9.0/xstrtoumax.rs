use std::str::FromStr;
use std::num::ParseIntError;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrToLongError {
    Invalid,
    InvalidSuffixChar,
    Overflow,
    InvalidSuffixCharWithOverflow,
    Ok,
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

pub fn xstrtoumax(
    s: &str,
    base: u32,
    valid_suffixes: Option<&str>,
) -> Result<(u64, StrToLongError), StrToLongError> {
    if base > 36 {
        return Err(StrToLongError::Invalid);
    }

    let s = s.trim_start();
    if s.starts_with('-') {
        return Err(StrToLongError::Invalid);
    }

    let (num_str, rest) = split_at_first_non_digit(s, base);
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

    let value = match u64::from_str_radix(num_str, base) {
        Ok(v) => v,
        Err(e) => return Err(e.into()),
    };

    if rest.is_empty() {
        return Ok((value, StrToLongError::Ok));
    }

    let mut err = StrToLongError::Ok;
    let mut scaled_value = value;
    let mut suffix_iter = rest.chars();
    let first_char = suffix_iter.next().unwrap();

    if let Some(suffixes) = valid_suffixes {
        if !suffixes.contains(first_char) {
            return Ok((value, StrToLongError::InvalidSuffixChar));
        }

        let (scale_factor, power) = match first_char {
            'b' => (512, 1),
            'B' => (1024, 1),
            'c' => (1, 1),
            'E' => (1024, 6),
            'G' | 'g' => (1024, 3),
            'k' | 'K' => (1024, 1),
            'M' | 'm' => (1024, 2),
            'P' => (1024, 5),
            'T' | 't' => (1024, 4),
            'w' => (2, 1),
            'Y' => (1024, 8),
            'Z' => (1024, 7),
            _ => return Ok((value, StrToLongError::InvalidSuffixChar)),
        };

        for _ in 0..power {
            scaled_value = match scaled_value.checked_mul(scale_factor as u64) {
                Some(v) => v,
                None => {
                    err = StrToLongError::Overflow;
                    u64::MAX
                }
            };
        }

        // Handle multi-character suffixes
        if let Some(next_char) = suffix_iter.next() {
            match (first_char, next_char) {
                ('E' | 'G' | 'g' | 'k' | 'K' | 'M' | 'm' | 'P' | 'T' | 't' | 'Y' | 'Z', 'i') => {
                    if suffix_iter.next() == Some('B') {
                        // Valid KiB-style suffix
                    } else {
                        err = StrToLongError::InvalidSuffixChar;
                    }
                }
                ('E' | 'G' | 'g' | 'k' | 'K' | 'M' | 'm' | 'P' | 'T' | 't' | 'Y' | 'Z', 'B' | 'D') => {
                    // Valid kB-style suffix (base 1000)
                    if scale_factor == 1024 {
                        // Need to adjust scaling
                    }
                }
                _ => {
                    err = StrToLongError::InvalidSuffixChar;
                }
            }
        }
    }

    Ok((scaled_value, err))
}

fn split_at_first_non_digit(s: &str, base: u32) -> (&str, &str) {
    let mut digit_end = 0;
    for c in s.chars() {
        if c.is_digit(base) {
            digit_end += c.len_utf8();
        } else {
            break;
        }
    }
    s.split_at(digit_end)
}

fn bkm_scale(x: &mut u64, scale_factor: u64) -> StrToLongError {
    match x.checked_mul(scale_factor) {
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

fn bkm_scale_by_power(x: &mut u64, base: u64, power: u32) -> StrToLongError {
    let mut err = StrToLongError::Ok;
    for _ in 0..power {
        let current_err = bkm_scale(x, base);
        if current_err != StrToLongError::Ok {
            err = current_err;
        }
    }
    err
}