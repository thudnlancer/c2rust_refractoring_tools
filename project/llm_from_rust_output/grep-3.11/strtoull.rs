use std::str::{self, FromStr};
use std::num::ParseIntError;
use std::ptr;

pub fn rpl_strtoull(nptr: &str, endptr: Option<&mut *const u8>, base: i32) -> Result<u64, ParseIntError> {
    let mut s = nptr.trim_start();
    let original_len = s.len();
    
    if base < 0 || base == 1 || base > 36 {
        return Err(ParseIntError { kind: std::num::IntErrorKind::InvalidDigit });
    }

    let negative = if s.starts_with('-') {
        s = &s[1..];
        true
    } else if s.starts_with('+') {
        s = &s[1..];
        false
    } else {
        false
    };

    let (mut s, base) = if s.starts_with('0') {
        if (base == 0 || base == 16) && s.len() > 1 && s[1..].starts_with(|c| c == 'X' || c == 'x') {
            (&s[2..], 16)
        } else if (base == 0 || base == 2) && s.len() > 1 && s[1..].starts_with(|c| c == 'B' || c == 'b') {
            (&s[2..], 2)
        } else if base == 0 {
            (s, 8)
        } else {
            (s, base)
        }
    } else if base == 0 {
        (s, 10)
    } else {
        (s, base)
    };

    let mut result: u64 = 0;
    let mut overflow = false;
    let cutoff = u64::MAX / base as u64;
    let cutlim = (u64::MAX % base as u64) as u32;

    let mut valid_chars = 0;
    for c in s.chars() {
        let digit = match c.to_digit(base) {
            Some(d) => d,
            None => break,
        };

        if result > cutoff || (result == cutoff && digit > cutlim) {
            overflow = true;
        }

        result = match result.checked_mul(base as u64) {
            Some(v) => v,
            None => {
                overflow = true;
                break;
            }
        };

        result = match result.checked_add(digit as u64) {
            Some(v) => v,
            None => {
                overflow = true;
                break;
            }
        };

        valid_chars += 1;
    }

    if valid_chars == 0 {
        if let Some(ptr) = endptr {
            *ptr = nptr.as_ptr();
        }
        return Err(ParseIntError { kind: std::num::IntErrorKind::InvalidDigit });
    }

    s = &s[..valid_chars];
    let bytes_consumed = original_len - s.len();

    if let Some(ptr) = endptr {
        *ptr = unsafe { nptr.as_ptr().add(bytes_consumed) };
    }

    if overflow {
        return Err(ParseIntError { kind: std::num::IntErrorKind::PosOverflow });
    }

    if negative {
        Ok(result.wrapping_neg() as u64)
    } else {
        Ok(result)
    }
}