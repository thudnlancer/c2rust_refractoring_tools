use std::num::IntErrorKind;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Radix {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

impl Radix {
    fn from_prefix(s: &str) -> Option<(Self, &str)> {
        if s.len() >= 2 && &s[..2] == "0x" {
            Some((Radix::Hexadecimal, &s[2..]))
        } else if s.len() >= 2 && &s[..2] == "0b" {
            Some((Radix::Binary, &s[2..]))
        } else if s.len() >= 1 && &s[..1] == "0" {
            Some((Radix::Octal, &s[1..]))
        } else {
            None
        }
    }

    fn as_u32(&self) -> u32 {
        match self {
            Radix::Binary => 2,
            Radix::Octal => 8,
            Radix::Decimal => 10,
            Radix::Hexadecimal => 16,
        }
    }
}

pub fn rpl_strtol(
    nptr: &str,
    endptr: Option<&mut usize>,
    mut base: i32,
) -> Result<i64, IntErrorKind> {
    if base < 0 || base == 1 || base > 36 {
        return Err(IntErrorKind::InvalidDigit);
    }

    let s = nptr.trim_start();
    if s.is_empty() {
        if let Some(ptr) = endptr {
            *ptr = 0;
        }
        return Ok(0);
    }

    let (negative, s) = match s.chars().next() {
        Some('-') => (true, &s[1..]),
        Some('+') => (false, &s[1..]),
        _ => (false, s),
    };

    let (radix, s) = if base == 0 {
        if let Some((r, s)) = Radix::from_prefix(s) {
            (r.as_u32() as i32, s)
        } else {
            (10, s)
        }
    } else {
        (base, s)
    };

    let mut result: u64 = 0;
    let cutoff = if negative {
        (i64::MAX as u64 + 1) / radix as u64
    } else {
        i64::MAX as u64 / radix as u64
    };
    let cutlim = if negative {
        ((i64::MAX as u64 + 1) % radix as u64) as u32
    } else {
        (i64::MAX as u64 % radix as u64) as u32
    };

    let mut overflow = false;
    let mut valid_digits = false;
    let mut processed_len = 0;

    for (i, c) in s.char_indices() {
        let digit = match c.to_digit(radix as u32) {
            Some(d) => d,
            None => break,
        };

        valid_digits = true;

        if result > cutoff || (result == cutoff && digit > cutlim) {
            overflow = true;
        }

        if !overflow {
            result = result * radix as u64 + digit as u64;
        }

        processed_len = i + c.len_utf8();
    }

    if !valid_digits {
        if let Some(ptr) = endptr {
            *ptr = nptr.len() - s.len();
        }
        return Ok(0);
    }

    if let Some(ptr) = endptr {
        *ptr = nptr.len() - s.len() + processed_len;
    }

    if overflow {
        return Err(IntErrorKind::PosOverflow);
    }

    let result = if negative {
        -(result as i64)
    } else {
        result as i64
    };

    Ok(result)
}