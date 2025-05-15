use std::str::{FromStr, from_utf8_unchecked};
use std::num::{ParseIntError, ParseFloatError};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrToLongError {
    Ok = 0,
    Overflow = 1,
    InvalidSuffixChar = 2,
    InvalidSuffixCharWithOverflow = 3,
    Invalid = 4,
}

impl fmt::Display for StrToLongError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StrToLongError::Ok => write!(f, "No error"),
            StrToLongError::Overflow => write!(f, "Overflow occurred"),
            StrToLongError::InvalidSuffixChar => write!(f, "Invalid suffix character"),
            StrToLongError::InvalidSuffixCharWithOverflow => 
                write!(f, "Both overflow and invalid suffix occurred"),
            StrToLongError::Invalid => write!(f, "Invalid input"),
        }
    }
}

impl Error for StrToLongError {}

impl From<ParseIntError> for StrToLongError {
    fn from(_: ParseIntError) -> Self {
        StrToLongError::Invalid
    }
}

impl From<ParseFloatError> for StrToLongError {
    fn from(_: ParseFloatError) -> Self {
        StrToLongError::Invalid
    }
}

pub trait StrToNum: Sized {
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;
    fn max_value() -> Self;
    fn min_value() -> Self;
    fn is_signed() -> bool;
}

macro_rules! impl_str_to_num {
    ($($t:ty)*) => ($(
        impl StrToNum for $t {
            fn from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
                <$t>::from_str_radix(src, radix)
            }
            fn max_value() -> Self {
                <$t>::MAX
            }
            fn min_value() -> Self {
                <$t>::MIN
            }
            fn is_signed() -> bool {
                <$t>::MIN != 0
            }
        }
    )*)
}

impl_str_to_num! { i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize }

fn bkm_scale<T: StrToNum>(x: &mut T, scale_factor: T) -> StrToLongError {
    match x.checked_mul(&scale_factor) {
        Some(scaled) => {
            *x = scaled;
            StrToLongError::Ok
        }
        None => {
            *x = if *x < T::from(0).unwrap() { T::min_value() } else { T::max_value() };
            StrToLongError::Overflow
        }
    }
}

fn bkm_scale_by_power<T: StrToNum>(x: &mut T, base: T, power: i32) -> StrToLongError {
    let mut err = StrToLongError::Ok;
    for _ in 0..power {
        err |= bkm_scale(x, base);
    }
    err
}

pub fn xstrtol<T: StrToNum>(
    s: &str,
    strtol_base: u32,
    valid_suffixes: Option<&str>,
) -> Result<(T, Option<&str>, StrToLongError), StrToLongError> {
    let mut s = s.trim_start();
    
    if T::is_signed() && s.starts_with('-') {
        return Err(StrToLongError::Invalid);
    }

    let num_str = s.split(|c: char| !c.is_ascii_digit()).next().unwrap_or("");
    if num_str.is_empty() {
        if let Some(suffixes) = valid_suffixes {
            if let Some(c) = s.chars().next() {
                if suffixes.contains(c) {
                    return Ok((T::from(1).unwrap(), Some(&s[1..]), StrToLongError::Ok);
                }
            }
        }
        return Err(StrToLongError::Invalid);
    }

    let val = match T::from_str_radix(num_str, strtol_base) {
        Ok(v) => v,
        Err(e) => {
            if e.kind() == &std::num::IntErrorKind::PosOverflow {
                return Ok((T::max_value(), None, StrToLongError::Overflow));
            } else if e.kind() == &std::num::IntErrorKind::NegOverflow {
                return Ok((T::min_value(), None, StrToLongError::Overflow));
            } else {
                return Err(StrToLongError::Invalid);
            }
        }
    };

    let remaining = &s[num_str.len()..];
    if remaining.is_empty() || valid_suffixes.is_none() {
        return Ok((val, None, StrToLongError::Ok));
    }

    let valid_suffixes = valid_suffixes.unwrap();
    let mut chars = remaining.chars();
    let first_char = chars.next().unwrap();
    
    if !valid_suffixes.contains(first_char) {
        return Ok((val, None, StrToLongError::InvalidSuffixChar));
    }

    let mut val = val;
    let mut err = StrToLongError::Ok;
    let mut suffix_len = 1;
    let base = if valid_suffixes.contains('0') {
        match chars.next() {
            Some('i') => {
                if chars.next() == Some('B') {
                    suffix_len += 2;
                    1024
                } else {
                    return Ok((val, None, StrToLongError::InvalidSuffixChar));
                }
            }
            Some('B') | Some('D') => {
                suffix_len += 1;
                1000
            }
            _ => 1024,
        }
    } else {
        1024
    };

    err |= match first_char {
        'b' => bkm_scale(&mut val, T::from(512).unwrap()),
        'B' => bkm_scale(&mut val, T::from(1024).unwrap()),
        'c' => StrToLongError::Ok,
        'E' => bkm_scale_by_power(&mut val, T::from(base).unwrap(), 6),
        'G' | 'g' => bkm_scale_by_power(&mut val, T::from(base).unwrap(), 3),
        'k' | 'K' => bkm_scale_by_power(&mut val, T::from(base).unwrap(), 1),
        'M' | 'm' => bkm_scale_by_power(&mut val, T::from(base).unwrap(), 2),
        'P' => bkm_scale_by_power(&mut val, T::from(base).unwrap(), 5),
        'T' | 't' => bkm_scale_by_power(&mut val, T::from(base).unwrap(), 4),
        'w' => bkm_scale(&mut val, T::from(2).unwrap()),
        'Y' => bkm_scale_by_power(&mut val, T::from(base).unwrap(), 8),
        'Z' => bkm_scale_by_power(&mut val, T::from(base).unwrap(), 7),
        _ => return Ok((val, None, StrToLongError::InvalidSuffixChar)),
    };

    let remaining = &remaining[suffix_len..];
    if !remaining.is_empty() {
        err |= StrToLongError::InvalidSuffixChar;
    }

    Ok((val, Some(remaining), err))
}