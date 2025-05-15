use std::char;
use std::cmp;
use std::i64;
use std::num::Wrapping;

const _ISspace: u16 = 8192;
const _ISalpha: u16 = 1024;

#[derive(Debug, PartialEq)]
enum Sign {
    Positive,
    Negative,
}

pub fn rpl_strtoll(
    nptr: &str,
    endptr: Option<&mut *const u8>,
    mut base: i32,
) -> Result<i64, (i64, &'static str)> {
    if base < 0 || base == 1 || base > 36 {
        return Err((0, "Invalid base"));
    }

    let mut chars = nptr.chars().peekable();
    
    // Skip whitespace
    while let Some(&c) = chars.peek() {
        if !c.is_whitespace() {
            break;
        }
        chars.next();
    }

    // Parse sign
    let sign = match chars.peek() {
        Some('+') => {
            chars.next();
            Sign::Positive
        }
        Some('-') => {
            chars.next();
            Sign::Negative
        }
        _ => Sign::Positive,
    };

    // Determine base if 0
    let mut digits_start = chars.clone();
    if let Some('0') = chars.peek() {
        chars.next();
        match chars.peek() {
            Some('x') | Some('X') if base == 0 || base == 16 => {
                chars.next();
                base = 16;
            }
            Some('b') | Some('B') if base == 0 || base == 2 => {
                chars.next();
                base = 2;
            }
            _ if base == 0 => {
                base = 8;
            }
            _ => {}
        }
    } else if base == 0 {
        base = 10;
    }

    // Parse digits
    let mut result: Wrapping<u64> = Wrapping(0);
    let cutoff = match sign {
        Sign::Positive => i64::MAX as u64,
        Sign::Negative => (i64::MIN as i128).unsigned_abs() as u64,
    } / base as u64;
    let cutlim = match sign {
        Sign::Positive => i64::MAX as u64,
        Sign::Negative => (i64::MIN as i128).unsigned_abs() as u64,
    } % base as u64;

    let mut overflow = false;
    let mut any_digits = false;
    let mut last_valid_pos = chars.clone();

    while let Some(&c) = chars.peek() {
        let digit = match c.to_digit(base as u32) {
            Some(d) => d as u8,
            None => break,
        };

        any_digits = true;
        last_valid_pos = chars.clone();

        if result.0 > cutoff || (result.0 == cutoff && digit as u64 > cutlim) {
            overflow = true;
        }

        result = result * Wrapping(base as u64) + Wrapping(digit as u64);
        chars.next();
    }

    if !any_digits {
        if let Some(endptr) = endptr {
            *endptr = nptr.as_ptr();
        }
        return Ok(0);
    }

    if let Some(endptr) = endptr {
        let pos = last_valid_pos
            .peek()
            .map(|c| nptr.len() - chars.as_str().len())
            .unwrap_or(nptr.len());
        *endptr = unsafe { nptr.as_ptr().add(pos) };
    }

    if overflow {
        return Err((
            match sign {
                Sign::Positive => i64::MAX,
                Sign::Negative => i64::MIN,
            },
            "Overflow",
        ));
    }

    let value = match sign {
        Sign::Positive => cmp::min(result.0, i64::MAX as u64) as i64,
        Sign::Negative => {
            let abs = cmp::min(result.0, (i64::MIN as i128).unsigned_abs() as u64);
            -(abs as i64)
        }
    };

    Ok(value)
}