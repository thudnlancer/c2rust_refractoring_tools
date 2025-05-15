use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
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

pub fn xstrtoul(
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

    let (num_str, rest) = split_at_first_non_digit(s, strtol_base);
    if num_str.is_empty() {
        if let Some(suffixes) = valid_suffixes {
            if let Some(c) = rest.chars().next() {
                if suffixes.contains(c) {
                    return Ok((1, Some(rest)));
                }
            }
        }
        return Err(StrToLongError::Invalid);
    }

    let val = u64::from_str_radix(num_str, strtol_base)?;
    let mut rest = rest;
    let mut err = StrToLongError::Ok;

    if let Some(suffixes) = valid_suffixes {
        if let Some(c) = rest.chars().next() {
            if !suffixes.contains(c) {
                return Err(if err == StrToLongError::Overflow {
                    StrToLongError::InvalidSuffixCharWithOverflow
                } else {
                    StrToLongError::InvalidSuffixChar
                });
            }

            let base = if suffixes.contains('0') {
                match rest.chars().nth(1) {
                    Some('i') if rest.chars().nth(2) == Some('B') => {
                        rest = &rest[3..];
                        1024
                    }
                    Some('B') | Some('D') => {
                        rest = &rest[2..];
                        1000
                    }
                    _ => 1024,
                }
            } else {
                1024
            };

            let power = match c {
                'b' => 512,
                'B' => 1024,
                'c' => 1,
                'E' => 6,
                'G' | 'g' => 3,
                'k' | 'K' => 1,
                'M' | 'm' => 2,
                'P' => 5,
                'Q' => 10,
                'R' => 9,
                'T' | 't' => 4,
                'w' => 2,
                'Y' => 8,
                'Z' => 7,
                _ => {
                    return Err(if err == StrToLongError::Overflow {
                        StrToLongError::InvalidSuffixCharWithOverflow
                    } else {
                        StrToLongError::InvalidSuffixChar
                    });
                }
            };

            let scaled_val = match scale_by_power(val, base, power) {
                Ok(v) => v,
                Err(e) => {
                    err = e;
                    u64::MAX
                }
            };

            if !rest.is_empty() {
                err = StrToLongError::InvalidSuffixChar;
            }

            return if err == StrToLongError::Ok {
                Ok((scaled_val, None))
            } else {
                Err(err)
            };
        }
    }

    Ok((val, if rest.is_empty() { None } else { Some(rest) }))
}

fn split_at_first_non_digit(s: &str, radix: u32) -> (&str, &str) {
    let radix = radix as u8;
    let pos = s
        .chars()
        .position(|c| !c.is_digit(radix.into()))
        .unwrap_or(s.len());
    s.split_at(pos)
}

fn scale_by_power(mut x: u64, base: u64, power: u32) -> Result<u64, StrToLongError> {
    for _ in 0..power {
        x = scale(x, base)?;
    }
    Ok(x)
}

fn scale(x: u64, scale_factor: u64) -> Result<u64, StrToLongError> {
    x.checked_mul(scale_factor)
        .ok_or(StrToLongError::Overflow)
}