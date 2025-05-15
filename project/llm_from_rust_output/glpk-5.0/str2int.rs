use std::char;

#[derive(Debug, PartialEq)]
pub enum CharClass {
    Digit,
    Alpha,
    Alnum,
    Punct,
    Cntrl,
    Blank,
    Graph,
    Print,
    Space,
    Xdigit,
    Lower,
    Upper,
}

#[derive(Debug, PartialEq)]
pub enum Str2IntError {
    Overflow,
    InvalidChar,
}

pub fn str2int(s: &str) -> Result<i32, Str2IntError> {
    let mut chars = s.chars().peekable();
    let mut sign = 1;
    let mut val = 0i32;

    // Handle optional sign
    match chars.peek() {
        Some('+') => {
            chars.next();
        }
        Some('-') => {
            sign = -1;
            chars.next();
        }
        _ => {}
    }

    // Check first character is digit
    match chars.peek() {
        Some(c) if c.is_ascii_digit() => {}
        _ => return Err(Str2IntError::InvalidChar),
    }

    // Process digits
    for c in chars {
        if !c.is_ascii_digit() {
            return Err(Str2IntError::InvalidChar);
        }

        let d = c.to_digit(10).unwrap() as i32;

        if sign > 0 {
            val = val.checked_mul(10).ok_or(Str2IntError::Overflow)?;
            val = val.checked_add(d).ok_or(Str2IntError::Overflow)?;
        } else {
            val = val.checked_mul(10).ok_or(Str2IntError::Overflow)?;
            val = val.checked_sub(d).ok_or(Str2IntError::Overflow)?;
        }
    }

    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_str2int() {
        assert_eq!(str2int("123"), Ok(123));
        assert_eq!(str2int("+456"), Ok(456));
        assert_eq!(str2int("-789"), Ok(-789));
        assert_eq!(str2int("2147483647"), Ok(2147483647));
        assert_eq!(str2int("-2147483648"), Ok(-2147483648));
        assert_eq!(str2int("2147483648"), Err(Str2IntError::Overflow));
        assert_eq!(str2int("-2147483649"), Err(Str2IntError::Overflow));
        assert_eq!(str2int("12a34"), Err(Str2IntError::InvalidChar));
        assert_eq!(str2int("abc"), Err(Str2IntError::InvalidChar));
    }
}