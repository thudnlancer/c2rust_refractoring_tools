use std::str::{FromStr, from_utf8};
use std::num::ParseIntError;
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
        match *self {
            StrToLongError::Ok => write!(f, "No error"),
            StrToLongError::Overflow => write!(f, "Overflow occurred"),
            StrToLongError::InvalidSuffixChar => write!(f, "Invalid suffix character"),
            StrToLongError::InvalidSuffixCharWithOverflow => 
                write!(f, "Both overflow and invalid suffix occurred"),
            StrToLongError::Invalid => write!(f, "Invalid input"),
        }
    }
}

impl std::ops::BitOr for StrToLongError {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (StrToLongError::Overflow, StrToLongError::InvalidSuffixChar) |
            (StrToLongError::InvalidSuffixChar, StrToLongError::Overflow) =>
                StrToLongError::InvalidSuffixCharWithOverflow,
            (StrToLongError::Ok, other) | (other, StrToLongError::Ok) => other,
            _ => self,
        }
    }
}

pub fn xstrtol<T>(s: &str, strtol_base: u32, val: &mut T, valid_suffixes: Option<&str>) 
    -> Result<(), StrToLongError>
where
    T: FromStr + Copy + PartialOrd + std::ops::MulAssign + std::ops::Neg<Output = T>,
    <T as FromStr>::Err: std::fmt::Debug,
    T: std::ops::Mul<Output = T> + From<u32>,
{
    let s = s.trim();
    if s.is_empty() {
        return Err(StrToLongError::Invalid);
    }

    // Check for negative numbers in unsigned types
    if !std::mem::size_of::<T>().is_power_of_two() && s.starts_with('-') {
        return Err(StrToLongError::Invalid);
    }

    let (num_str, rest) = split_at_first_non_digit(s, strtol_base);
    let mut tmp = if num_str.is_empty() {
        if let Some(suffixes) = valid_suffixes {
            if !rest.is_empty() && suffixes.contains(rest.chars().next().unwrap()) {
                T::from_str("1").unwrap()
            } else {
                return Err(StrToLongError::Invalid);
            }
        } else {
            return Err(StrToLongError::Invalid);
        }
    } else {
        match T::from_str(num_str) {
            Ok(n) => n,
            Err(_) => return Err(StrToLongError::Overflow),
        }
    };

    let mut err = StrToLongError::Ok;

    if let Some(suffixes) = valid_suffixes {
        if !rest.is_empty() {
            let mut chars = rest.chars();
            let first_char = chars.next().unwrap();
            if !suffixes.contains(first_char) {
                *val = tmp;
                return Ok(err);
            }

            let (base, power) = match first_char {
                'E' | 'G' | 'g' | 'k' | 'K' | 'M' | 'm' | 'P' | 'T' | 't' | 'Y' | 'Z' => {
                    if suffixes.contains('0') {
                        match chars.next() {
                            Some('i') if chars.next() == Some('B') => (1024, 1),
                            Some('B') | Some('D') => (1000, 1),
                            _ => (1024, 0),
                        }
                    } else {
                        (1024, 0)
                    }
                }
                _ => (1024, 0),
            };

            let overflow = match first_char {
                'b' => bkm_scale(&mut tmp, 512),
                'B' => bkm_scale(&mut tmp, 1024),
                'c' => Ok(()),
                'E' => bkm_scale_by_power(&mut tmp, base, 6),
                'G' | 'g' => bkm_scale_by_power(&mut tmp, base, 3),
                'k' | 'K' => bkm_scale_by_power(&mut tmp, base, 1),
                'M' | 'm' => bkm_scale_by_power(&mut tmp, base, 2),
                'P' => bkm_scale_by_power(&mut tmp, base, 5),
                'T' | 't' => bkm_scale_by_power(&mut tmp, base, 4),
                'w' => bkm_scale(&mut tmp, 2),
                'Y' => bkm_scale_by_power(&mut tmp, base, 8),
                'Z' => bkm_scale_by_power(&mut tmp, base, 7),
                _ => {
                    *val = tmp;
                    return Err(err | StrToLongError::InvalidSuffixChar);
                }
            };

            if overflow.is_err() {
                err = err | StrToLongError::Overflow;
            }
        }
    }

    *val = tmp;
    Ok(err)
}

fn split_at_first_non_digit(s: &str, base: u32) -> (&str, &str) {
    let pos = s.find(|c: char| !c.is_digit(base)).unwrap_or(s.len());
    s.split_at(pos)
}

fn bkm_scale<T>(x: &mut T, scale_factor: T) -> Result<(), StrToLongError>
where
    T: PartialOrd + std::ops::MulAssign + Copy,
{
    *x *= scale_factor;
    Ok(())
}

fn bkm_scale_by_power<T>(x: &mut T, base: T, power: u32) -> Result<(), StrToLongError>
where
    T: From<u32> + PartialOrd + std::ops::MulAssign + Copy,
{
    for _ in 0..power {
        bkm_scale(x, base)?;
    }
    Ok(())
}

// Implementations for specific types
pub fn xstrtoul(s: &str, strtol_base: u32, val: &mut u64, valid_suffixes: Option<&str>) 
    -> Result<(), StrToLongError> {
    xstrtol(s, strtol_base, val, valid_suffixes)
}

pub fn xstrtol(s: &str, strtol_base: u32, val: &mut i64, valid_suffixes: Option<&str>) 
    -> Result<(), StrToLongError> {
    xstrtol(s, strtol_base, val, valid_suffixes)
}

pub fn xstrtoll(s: &str, strtol_base: u32, val: &mut i128, valid_suffixes: Option<&str>) 
    -> Result<(), StrToLongError> {
    xstrtol(s, strtol_base, val, valid_suffixes)
}

pub fn xstrtoull(s: &str, strtol_base: u32, val: &mut u128, valid_suffixes: Option<&str>) 
    -> Result<(), StrToLongError> {
    xstrtol(s, strtol_base, val, valid_suffixes)
}

pub fn xstrtoimax(s: &str, strtol_base: u32, val: &mut i128, valid_suffixes: Option<&str>) 
    -> Result<(), StrToLongError> {
    xstrtol(s, strtol_base, val, valid_suffixes)
}

pub fn xstrtoumax(s: &str, strtol_base: u32, val: &mut u128, valid_suffixes: Option<&str>) 
    -> Result<(), StrToLongError> {
    xstrtol(s, strtol_base, val, valid_suffixes)
}