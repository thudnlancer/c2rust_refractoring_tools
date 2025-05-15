use std::str::FromStr;
use std::num::{IntErrorKind, ParseIntError};
use std::convert::TryFrom;

pub fn strtol(nptr: &str, base: u32) -> Result<i64, ParseIntError> {
    // Handle base validation
    if base < 2 || base > 36 {
        return Err(ParseIntError {
            kind: IntErrorKind::InvalidDigit,
        });
    }

    // Skip whitespace
    let s = nptr.trim_start();

    // Handle empty string after trimming
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

    // Handle base prefix
    let (base, s) = if s.starts_with("0x") || s.starts_with("0X") {
        (16, &s[2..])
    } else if s.starts_with("0b") || s.starts_with("0B") {
        (2, &s[2..])
    } else if s.starts_with('0') {
        (8, &s[1..])
    } else {
        (base, s)
    };

    // Parse the number
    let mut value: u64 = 0;
    let cutoff = u64::MAX / base as u64;
    let cutlim = u64::MAX % base as u64;

    for c in s.chars() {
        let digit = match c {
            '0'..='9' => c as u64 - '0' as u64,
            'a'..='z' => c as u64 - 'a' as u64 + 10,
            'A'..='Z' => c as u64 - 'A' as u64 + 10,
            _ => break,
        };

        if digit >= base as u64 {
            break;
        }

        // Check for overflow
        if value > cutoff || (value == cutoff && digit > cutlim) {
            return Err(ParseIntError {
                kind: IntErrorKind::PosOverflow,
            });
        }

        value = value * base as u64 + digit;
    }

    // Apply sign and check bounds
    if negative {
        if value > (i64::MAX as u64) + 1 {
            Err(ParseIntError {
                kind: IntErrorKind::NegOverflow,
            })
        } else if value == (i64::MAX as u64) + 1 {
            Ok(i64::MIN)
        } else {
            Ok(-(value as i64))
        }
    } else {
        if value > i64::MAX as u64 {
            Err(ParseIntError {
                kind: IntErrorKind::PosOverflow,
            })
        } else {
            Ok(value as i64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decimal() {
        assert_eq!(strtol("123", 10), Ok(123));
        assert_eq!(strtol("-123", 10), Ok(-123));
        assert_eq!(strtol("+123", 10), Ok(123));
    }

    #[test]
    fn test_hexadecimal() {
        assert_eq!(strtol("0x1a", 16), Ok(0x1a));
        assert_eq!(strtol("0X1A", 16), Ok(0x1A));
        assert_eq!(strtol("-0x1a", 16), Ok(-0x1a));
    }

    #[test]
    fn test_octal() {
        assert_eq!(strtol("0123", 8), Ok(0o123));
        assert_eq!(strtol("-0123", 8), Ok(-0o123));
    }

    #[test]
    fn test_binary() {
        assert_eq!(strtol("0b1010", 2), Ok(0b1010));
        assert_eq!(strtol("-0B1010", 2), Ok(-0b1010));
    }

    #[test]
    fn test_auto_base() {
        assert_eq!(strtol("123", 0), Ok(123));
        assert_eq!(strtol("0x123", 0), Ok(0x123));
        assert_eq!(strtol("0123", 0), Ok(0o123));
        assert_eq!(strtol("0b1010", 0), Ok(0b1010));
    }

    #[test]
    fn test_overflow() {
        assert!(strtol(&i64::MAX.to_string(), 10).is_ok());
        assert!(strtol(&(i64::MAX as u64 + 1).to_string(), 10).is_err());
        assert!(strtol(&format!("-{}", (i64::MAX as u64 + 1)), 10).is_ok()); // i64::MIN
        assert!(strtol(&format!("-{}", (i64::MAX as u64 + 2)), 10).is_err());
    }

    #[test]
    fn test_invalid() {
        assert!(strtol("", 10).is_err());
        assert!(strtol("abc", 10).is_err());
        assert!(strtol("123", 1).is_err());
        assert!(strtol("123", 37).is_err());
    }
}