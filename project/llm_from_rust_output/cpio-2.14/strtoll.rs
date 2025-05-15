use std::char;
use std::num::IntErrorKind;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum ParseIntError {
    InvalidDigit,
    PosOverflow,
    NegOverflow,
    Empty,
    InvalidBase,
}

fn rpl_strtoll(nptr: &str, endptr: Option<&mut usize>, base: u32) -> Result<i64, ParseIntError> {
    if base < 2 || base > 36 {
        return Err(ParseIntError::InvalidBase);
    }

    let s = nptr.trim_start();
    if s.is_empty() {
        if let Some(ptr) = endptr {
            *ptr = 0;
        }
        return Err(ParseIntError::Empty);
    }

    let (sign, digits) = match s.chars().next() {
        Some('-') => (-1, &s[1..]),
        Some('+') => (1, &s[1..]),
        _ => (1, s),
    };

    let (digits, radix) = if digits.starts_with("0x") || digits.starts_with("0X") {
        if base == 0 || base == 16 {
            (&digits[2..], 16)
        } else {
            (digits, base)
        }
    } else if digits.starts_with("0b") || digits.starts_with("0B") {
        if base == 0 || base == 2 {
            (&digits[2..], 2)
        } else {
            (digits, base)
        }
    } else if digits.starts_with('0') {
        if base == 0 {
            (&digits[1..], 8)
        } else {
            (digits, base)
        }
    } else {
        if base == 0 {
            (digits, 10)
        } else {
            (digits, base)
        }
    };

    let mut result: u64 = 0;
    let cutoff = if sign == 1 {
        i64::MAX as u64 / radix as u64
    } else {
        (i64::MAX as u64 + 1) / radix as u64
    };
    let cutlim = if sign == 1 {
        i64::MAX as u64 % radix as u64
    } else {
        (i64::MAX as u64 + 1) % radix as u64
    };

    let mut overflow = false;
    let mut valid_digits = false;
    let mut processed = 0;

    for (i, c) in digits.chars().enumerate() {
        let digit = match c.to_digit(radix) {
            Some(d) => d as u64,
            None => break,
        };

        valid_digits = true;
        processed = i + 1;

        if result > cutoff || (result == cutoff && digit > cutlim) {
            overflow = true;
        } else {
            result = result * radix as u64 + digit;
        }
    }

    if !valid_digits {
        if let Some(ptr) = endptr {
            *ptr = nptr.len() - digits.len();
        }
        return Err(ParseIntError::InvalidDigit);
    }

    if let Some(ptr) = endptr {
        *ptr = nptr.len() - digits.len() + processed;
    }

    if overflow {
        return if sign == 1 {
            Err(ParseIntError::PosOverflow)
        } else {
            Err(ParseIntError::NegOverflow)
        };
    }

    Ok(sign * result as i64)
}

#[no_mangle]
pub extern "C" fn rpl_strtoll_c(
    nptr: *const libc::c_char,
    endptr: *mut *mut libc::c_char,
    base: libc::c_int,
) -> libc::c_longlong {
    let base = if base == 0 { 10 } else { base as u32 };
    let c_str = unsafe { std::ffi::CStr::from_ptr(nptr) };
    let s = match c_str.to_str() {
        Ok(s) => s,
        Err(_) => {
            if !endptr.is_null() {
                unsafe { *endptr = nptr as *mut libc::c_char };
            }
            return 0;
        }
    };

    let mut end = 0;
    let result = rpl_strtoll(s, Some(&mut end), base);

    if !endptr.is_null() {
        unsafe {
            *endptr = nptr.add(end) as *mut libc::c_char;
        }
    }

    match result {
        Ok(n) => n as libc::c_longlong,
        Err(ParseIntError::PosOverflow) => {
            unsafe { *libc::__errno_location() = libc::ERANGE };
            libc::LONG_MAX as libc::c_longlong
        }
        Err(ParseIntError::NegOverflow) => {
            unsafe { *libc::__errno_location() = libc::ERANGE };
            libc::LONG_MIN as libc::c_longlong
        }
        _ => 0,
    }
}