use std::num::Wrapping;
use std::str;

#[derive(Debug, PartialEq)]
enum ParseError {
    InvalidBase,
    Overflow,
    NoDigits,
}

fn rpl_strtoull(nptr: &str, endptr: Option<&mut &str>, base: i32) -> Result<u64, ParseError> {
    if base < 0 || base == 1 || base > 36 {
        return Err(ParseError::InvalidBase);
    }

    let mut s = nptr.trim_start();
    let original = s;
    let negative = if s.starts_with('-') {
        s = &s[1..];
        true
    } else if s.starts_with('+') {
        s = &s[1..];
        false
    } else {
        false
    };

    let (mut base, mut s) = if s.starts_with('0') {
        if (base == 0 || base == 16) && s.len() > 1 && s[1..].starts_with(|c| c == 'X' || c == 'x') {
            (16, &s[2..])
        } else if (base == 0 || base == 2) && s.len() > 1 && s[1..].starts_with(|c| c == 'B' || c == 'b') {
            (2, &s[2..])
        } else if base == 0 {
            (8, &s[1..])
        } else {
            (base, s)
        }
    } else if base == 0 {
        (10, s)
    } else {
        (base, s)
    };

    if s.is_empty() {
        if let Some(ptr) = endptr {
            *ptr = original;
        }
        return Err(ParseError::NoDigits);
    }

    let cutoff = u64::MAX / base as u64;
    let cutlim = (u64::MAX % base as u64) as u32;
    let mut overflow = false;
    let mut result = Wrapping(0u64);

    for c in s.chars() {
        let digit = match c {
            '0'..='9' => c as u32 - '0' as u32,
            'a'..='z' => c as u32 - 'a' as u32 + 10,
            'A'..='Z' => c as u32 - 'A' as u32 + 10,
            _ => break,
        };

        if digit >= base as u32 {
            break;
        }

        if result.0 > cutoff || (result.0 == cutoff && digit > cutlim) {
            overflow = true;
        }

        result = result * Wrapping(base as u64) + Wrapping(digit as u64);
        s = &s[c.len_utf8()..];
    }

    if s == nptr {
        if let Some(ptr) = endptr {
            *ptr = original;
        }
        return Err(ParseError::NoDigits);
    }

    if let Some(ptr) = endptr {
        *ptr = s;
    }

    if overflow {
        Err(ParseError::Overflow)
    } else {
        Ok(if negative {
            result.0.wrapping_neg()
        } else {
            result.0
        })
    }
}