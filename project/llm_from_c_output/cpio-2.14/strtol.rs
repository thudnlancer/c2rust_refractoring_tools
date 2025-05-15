use std::error::Error;
use std::num::{IntErrorKind, ParseIntError};
use std::str::FromStr;

pub fn strtol<T: FromStr>(
    nptr: &str,
    base: u32,
) -> Result<(T, Option<&str>), ParseIntError> {
    // Skip whitespace
    let s = nptr.trim_start();
    
    if s.is_empty() {
        return Err(ParseIntError {
            kind: IntErrorKind::Empty,
        });
    }

    // Check for sign
    let (negative, s) = match s.chars().next() {
        Some('-') => (true, &s[1..]),
        Some('+') => (false, &s[1..]),
        _ => (false, s),
    };

    // Determine base if 0
    let (base, s) = if base == 0 {
        if s.starts_with("0x") || s.starts_with("0X") {
            (16, &s[2..])
        } else if s.starts_with("0b") || s.starts_with("0B") {
            (2, &s[2..])
        } else if s.starts_with('0') {
            (8, &s[1..])
        } else {
            (10, s)
        }
    } else {
        (base, s)
    };

    // Validate base
    if base < 2 || base > 36 {
        return Err(ParseIntError {
            kind: IntErrorKind::InvalidDigit,
        });
    }

    // Parse digits
    let mut value: T = T::from_str("0").unwrap();
    let mut overflow = false;
    
    for c in s.chars() {
        let digit = match c.to_digit(base) {
            Some(d) => d,
            None => break,
        };
        
        // Check for overflow
        if let Some(new_value) = value.checked_mul(base.into())
            .and_then(|v| v.checked_add(digit.into())) {
            value = new_value;
        } else {
            overflow = true;
        }
    }

    if overflow {
        return Err(ParseIntError {
            kind: IntErrorKind::PosOverflow,
        });
    }

    // Apply sign
    let value = if negative {
        if let Some(v) = value.checked_neg() {
            v
        } else {
            return Err(ParseIntError {
                kind: IntErrorKind::NegOverflow,
            });
        }
    } else {
        value
    };

    Ok((value, Some(s)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        assert_eq!(strtol::<i32>("123", 10), Ok((123, None)));
        assert_eq!(strtol::<i32>("+123", 10), Ok((123, None)));
        assert_eq!(strtol::<i32>("-123", 10), Ok((-123, None)));
    }

    #[test]
    fn test_hexadecimal() {
        assert_eq!(strtol::<i32>("0x1a", 0), Ok((26, None)));
        assert_eq!(strtol::<i32>("0X1A", 0), Ok((26, None)));
    }

    #[test]
    fn test_octal() {
        assert_eq!(strtol::<i32>("0755", 0), Ok((493, None)));
    }

    #[test]
    fn test_binary() {
        assert_eq!(strtol::<i32>("0b1010", 0), Ok((10, None)));
    }

    #[test]
    fn test_invalid() {
        assert!(strtol::<i32>("abc", 10).is_err());
        assert!(strtol::<i32>("", 10).is_err());
    }

    #[test]
    fn test_overflow() {
        assert!(strtol::<i32>("99999999999999999999", 10).is_err());
    }
}