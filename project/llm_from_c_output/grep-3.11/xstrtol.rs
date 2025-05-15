use std::num::{IntErrorKind, ParseIntError};
use std::str::FromStr;
use std::convert::TryFrom;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StrToLongError {
    Ok = 0,
    Overflow = 1,
    InvalidSuffixChar = 2,
    InvalidSuffixCharWithOverflow = 3,
    Invalid = 4,
}

impl StrToLongError {
    pub fn combine(self, other: Self) -> Self {
        match (self, other) {
            (Self::Ok, e) | (e, Self::Ok) => e,
            (Self::Overflow, Self::InvalidSuffixChar) | (Self::InvalidSuffixChar, Self::Overflow) => {
                Self::InvalidSuffixCharWithOverflow
            }
            _ => Self::Invalid,
        }
    }
}

pub fn xstrtol<T>(s: &str, base: u32, valid_suffixes: Option<&str>) -> Result<(T, StrToLongError), StrToLongError>
where
    T: FromStr<Err = ParseIntError>
        + TryFrom<i128>
        + TryFrom<u128>
        + std::cmp::PartialOrd
        + std::fmt::Debug
        + Copy,
    <T as TryFrom<i128>>::Error: std::fmt::Debug,
    <T as TryFrom<u128>>::Error: std::fmt::Debug,
{
    let s = s.trim();
    if s.is_empty() {
        return Err(StrToLongError::Invalid);
    }

    // Check for negative numbers in unsigned types
    if !T::from_str("-1").is_ok() && s.starts_with('-') {
        return Err(StrToLongError::Invalid);
    }

    let (num_str, rest) = split_at_first_non_digit(s, base);
    let mut err = StrToLongError::Ok;

    let value = if num_str.is_empty() {
        // No number found, check if valid suffix exists
        if let Some(suffixes) = valid_suffixes {
            if let Some(c) = rest.chars().next() {
                if suffixes.contains(c) {
                    1
                } else {
                    return Err(StrToLongError::Invalid);
                }
            } else {
                return Err(StrToLongError::Invalid);
            }
        } else {
            return Err(StrToLongError::Invalid);
        }
    } else {
        match T::from_str_radix(num_str, base) {
            Ok(v) => v,
            Err(e) => match e.kind() {
                IntErrorKind::PosOverflow => {
                    err = err.combine(StrToLongError::Overflow);
                    if num_str.starts_with('-') {
                        if let Ok(v) = i128::from_str_radix(num_str, base) {
                            if v < 0 {
                                T::try_from(i128::MIN).unwrap()
                            } else {
                                T::try_from(i128::MAX).unwrap()
                            }
                        } else {
                            return Err(StrToLongError::Invalid);
                        }
                    } else {
                        T::try_from(u128::MAX).unwrap()
                    }
                }
                IntErrorKind::NegOverflow => {
                    err = err.combine(StrToLongError::Overflow);
                    T::try_from(i128::MIN).unwrap()
                }
                _ => return Err(StrToLongError::Invalid),
            },
        }
    };

    if rest.is_empty() || valid_suffixes.is_none() {
        return Ok((value, err));
    }

    let suffixes = valid_suffixes.unwrap();
    let mut chars = rest.chars();
    let first_char = chars.next().unwrap();

    if !suffixes.contains(first_char) {
        return Ok((value, err.combine(StrToLongError::InvalidSuffixChar)));
    }

    let (base, consumed) = match first_char {
        'E' | 'G' | 'g' | 'k' | 'K' | 'M' | 'm' | 'P' | 'Q' | 'R' | 'T' | 't' | 'Y' | 'Z' => {
            if suffixes.contains('0') {
                let mut iter = rest.chars();
                iter.next(); // skip first char
                match (iter.next(), iter.next()) {
                    (Some('i'), Some('B')) => (1024, 3),
                    (Some('B' | 'D'), _) => (1000, 2),
                    _ => (1024, 1),
                }
            } else {
                (1024, 1)
            }
        }
        _ => (1024, 1),
    };

    let scaled_value = match first_char {
        'b' => scale(value, 512),
        'B' => scale(value, 1024),
        'c' => Ok(value),
        'E' => scale_by_power(value, base, 6),
        'G' | 'g' => scale_by_power(value, base, 3),
        'k' | 'K' => scale_by_power(value, base, 1),
        'M' | 'm' => scale_by_power(value, base, 2),
        'P' => scale_by_power(value, base, 5),
        'Q' => scale_by_power(value, base, 10),
        'R' => scale_by_power(value, base, 9),
        'T' | 't' => scale_by_power(value, base, 4),
        'w' => scale(value, 2),
        'Y' => scale_by_power(value, base, 8),
        'Z' => scale_by_power(value, base, 7),
        _ => return Ok((value, err.combine(StrToLongError::InvalidSuffixChar))),
    };

    let scaled_value = match scaled_value {
        Ok(v) => v,
        Err(e) => {
            err = err.combine(e);
            if value < T::try_from(0).unwrap_or_else(|_| panic!("Negative value for unsigned type")) {
                T::try_from(i128::MIN).unwrap()
            } else {
                T::try_from(i128::MAX).unwrap()
            }
        }
    };

    if rest.chars().count() > consumed {
        err = err.combine(StrToLongError::InvalidSuffixChar);
    }

    Ok((scaled_value, err))
}

fn split_at_first_non_digit(s: &str, base: u32) -> (&str, &str) {
    let mut digit_end = 0;
    for c in s.chars() {
        if c.is_digit(base) || (digit_end == 0 && (c == '+' || c == '-')) {
            digit_end += c.len_utf8();
        } else {
            break;
        }
    }
    s.split_at(digit_end)
}

fn scale<T>(value: T, factor: T) -> Result<T, StrToLongError>
where
    T: std::ops::Mul<Output = T> + std::cmp::Ord + Copy,
{
    let max = T::max_value();
    if factor == T::try_from(0).unwrap() {
        return Ok(T::try_from(0).unwrap());
    }
    if value > max / factor {
        Err(StrToLongError::Overflow)
    } else {
        Ok(value * factor)
    }
}

fn scale_by_power<T>(value: T, base: T, power: u32) -> Result<T, StrToLongError>
where
    T: std::ops::Mul<Output = T> + std::cmp::Ord + Copy + From<u32>,
{
    let mut result = value;
    for _ in 0..power {
        result = scale(result, base)?;
    }
    Ok(result)
}

trait MaxValue {
    fn max_value() -> Self;
}

impl MaxValue for i128 {
    fn max_value() -> Self {
        i128::MAX
    }
}

impl MaxValue for u128 {
    fn max_value() -> Self {
        u128::MAX
    }
}

impl MaxValue for i64 {
    fn max_value() -> Self {
        i64::MAX
    }
}

impl MaxValue for u64 {
    fn max_value() -> Self {
        u64::MAX
    }
}

impl MaxValue for i32 {
    fn max_value() -> Self {
        i32::MAX
    }
}

impl MaxValue for u32 {
    fn max_value() -> Self {
        u32::MAX
    }
}

impl MaxValue for i16 {
    fn max_value() -> Self {
        i16::MAX
    }
}

impl MaxValue for u16 {
    fn max_value() -> Self {
        u16::MAX
    }
}

impl MaxValue for i8 {
    fn max_value() -> Self {
        i8::MAX
    }
}

impl MaxValue for u8 {
    fn max_value() -> Self {
        u8::MAX
    }
}