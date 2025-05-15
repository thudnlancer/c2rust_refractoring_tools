use std::num::IntErrorKind;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Base {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl Base {
    fn from_prefix(s: &str) -> Option<(Base, &str)> {
        if s.len() >= 2 && &s[0..1] == "0" {
            match s[1..2].to_ascii_uppercase().as_str() {
                "X" => Some((Base::Hexadecimal, &s[2..])),
                "B" => Some((Base::Binary, &s[2..])),
                _ => Some((Base::Octal, &s[1..])),
            }
        } else {
            None
        }
    }

    fn radix(&self) -> u32 {
        match self {
            Base::Binary => 2,
            Base::Octal => 8,
            Base::Decimal => 10,
            Base::Hexadecimal => 16,
        }
    }
}

fn rpl_strtol(nptr: &str, base: i32) -> Result<i64, (IntErrorKind, &str)> {
    let mut s = nptr.trim_start();
    let mut negative = false;

    if let Some(rest) = s.strip_prefix('-') {
        negative = true;
        s = rest;
    } else if let Some(rest) = s.strip_prefix('+') {
        s = rest;
    }

    let (base, s) = match base {
        0 => {
            if let Some((detected_base, rest)) = Base::from_prefix(s) {
                (detected_base.radix(), rest)
            } else {
                (10, s)
            }
        }
        2..=36 => (base as u32, s),
        _ => return Err((IntErrorKind::InvalidDigit, nptr)),
    };

    if s.is_empty() {
        return Err((IntErrorKind::Empty, nptr));
    }

    let mut value: u64 = 0;
    let cutoff = u64::MAX / base as u64;
    let cutlim = (u64::MAX % base as u64) as u32;

    for (i, c) in s.char_indices() {
        let digit = match c.to_digit(base) {
            Some(d) => d,
            None => {
                if i == 0 {
                    return Err((IntErrorKind::InvalidDigit, nptr));
                } else {
                    break;
                }
            }
        };

        if value > cutoff || (value == cutoff && digit > cutlim) {
            return Err((IntErrorKind::PosOverflow, nptr));
        }

        value = value * base as u64 + digit as u64;
    }

    let value = if negative {
        let abs_value = value as i64;
        if abs_value == i64::MIN {
            abs_value
        } else {
            -abs_value
        }
    } else {
        value as i64
    };

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn test_base_detection() {
        assert_eq!(Base::from_prefix("0x123"), Some((Base::Hexadecimal, "123")));
        assert_eq!(Base::from_prefix("0b101"), Some((Base::Binary, "101")));
        assert_eq!(Base::from_prefix("0123"), Some((Base::Octal, "123")));
        assert_eq!(Base::from_prefix("123"), None);
    }

    #[test]
    fn test_strtol() {
        assert_eq!(rpl_strtol("123", 10), Ok(123));
        assert_eq!(rpl_strtol("-123", 10), Ok(-123));
        assert_eq!(rpl_strtol("0x123", 0), Ok(0x123));
        assert_eq!(rpl_strtol("0b101", 0), Ok(0b101));
        assert_eq!(rpl_strtol("0123", 0), Ok(0o123));
        assert_eq!(rpl_strtol("123", 0), Ok(123));
        assert_eq!(rpl_strtol("99999999999999999999", 10), Err((IntErrorKind::PosOverflow, "99999999999999999999")));
        assert_eq!(rpl_strtol("-99999999999999999999", 10), Err((IntErrorKind::PosOverflow, "-99999999999999999999")));
        assert_eq!(rpl_strtol("invalid", 10), Err((IntErrorKind::InvalidDigit, "invalid")));
        assert_eq!(rpl_strtol("", 10), Err((IntErrorKind::Empty, "")));
    }
}