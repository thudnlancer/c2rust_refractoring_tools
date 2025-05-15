use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum ParseNumberError {
    InvalidFormat,
    OutOfRange,
}

pub fn str2num(s: &str) -> Result<f64, ParseNumberError> {
    // Check for valid number format
    let mut chars = s.chars().peekable();
    
    // Optional sign
    if let Some(&c) = chars.peek() {
        if c == '+' || c == '-' {
            chars.next();
        }
    }
    
    // Check for digits before decimal point
    let mut has_digits = false;
    while let Some(&c) = chars.peek() {
        if c.is_ascii_digit() {
            has_digits = true;
            chars.next();
        } else {
            break;
        }
    }
    
    // Optional decimal point and digits after
    if let Some(&c) = chars.peek() {
        if c == '.' {
            chars.next();
            let mut has_decimal_digits = false;
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() {
                    has_decimal_digits = true;
                    chars.next();
                } else {
                    break;
                }
            }
            if !has_digits && !has_decimal_digits {
                return Err(ParseNumberError::InvalidFormat);
            }
        } else if !has_digits {
            return Err(ParseNumberError::InvalidFormat);
        }
    } else if !has_digits {
        return Err(ParseNumberError::InvalidFormat);
    }
    
    // Optional exponent
    if let Some(&c) = chars.peek() {
        if c == 'e' || c == 'E' {
            chars.next();
            // Optional sign in exponent
            if let Some(&c) = chars.peek() {
                if c == '+' || c == '-' {
                    chars.next();
                }
            }
            // Must have digits in exponent
            let mut has_exponent_digits = false;
            while let Some(&c) = chars.peek() {
                if c.is_ascii_digit() {
                    has_exponent_digits = true;
                    chars.next();
                } else {
                    break;
                }
            }
            if !has_exponent_digits {
                return Err(ParseNumberError::InvalidFormat);
            }
        }
    }
    
    // Should be at end of string
    if chars.peek().is_some() {
        return Err(ParseNumberError::InvalidFormat);
    }
    
    // Parse the number
    let val = f64::from_str(s).map_err(|_| ParseNumberError::InvalidFormat)?;
    
    // Check range
    if !val.is_finite() {
        return Err(ParseNumberError::OutOfRange);
    }
    
    // Handle denormals by rounding to zero
    let val = if (-2.2250738585072014e-308f64..2.2250738585072014e-308f64).contains(&val) && val != 0.0 {
        0.0
    } else {
        val
    };
    
    Ok(val)
}