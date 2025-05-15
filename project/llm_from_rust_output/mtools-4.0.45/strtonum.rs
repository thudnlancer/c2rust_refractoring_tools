use std::num::{ParseIntError, Wrapping};
use std::str::FromStr;
use std::fmt;

#[derive(Debug)]
enum SizeUnit {
    Bytes,
    Kilobytes,
    Megabytes,
    Gigabytes,
    Terabytes,
    Sectors,
}

impl SizeUnit {
    fn from_char(c: char) -> Option<Self> {
        match c.to_ascii_uppercase() {
            'B' => Some(SizeUnit::Bytes),
            'K' => Some(SizeUnit::Kilobytes),
            'M' => Some(SizeUnit::Megabytes),
            'G' => Some(SizeUnit::Gigabytes),
            'T' => Some(SizeUnit::Terabytes),
            'S' => Some(SizeUnit::Sectors),
            _ => None,
        }
    }

    fn multiplier(&self) -> u32 {
        match self {
            SizeUnit::Bytes => 1,
            SizeUnit::Kilobytes => 1024,
            SizeUnit::Megabytes => 1024 * 1024,
            SizeUnit::Gigabytes => 1024 * 1024 * 1024,
            SizeUnit::Terabytes => 1024 * 1024 * 1024 * 1024,
            SizeUnit::Sectors => 512,
        }
    }
}

#[derive(Debug)]
enum ParseSizeError {
    InvalidNumber(ParseIntError),
    InvalidUnit,
    Overflow,
    ExtraCharacters,
}

impl fmt::Display for ParseSizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseSizeError::InvalidNumber(e) => write!(f, "Invalid number: {}", e),
            ParseSizeError::InvalidUnit => write!(f, "Invalid unit suffix"),
            ParseSizeError::Overflow => write!(f, "Value too large"),
            ParseSizeError::ExtraCharacters => write!(f, "Extra characters after size"),
        }
    }
}

impl From<ParseIntError> for ParseSizeError {
    fn from(err: ParseIntError) -> Self {
        ParseSizeError::InvalidNumber(err)
    }
}

fn parse_size(size_str: &str) -> Result<u32, ParseSizeError> {
    let (num_str, unit_char) = match size_str.find(|c: char| c.is_ascii_alphabetic()) {
        Some(pos) => {
            let (num, rest) = size_str.split_at(pos);
            let unit_char = rest.chars().next().unwrap();
            (num, Some(unit_char))
        }
        None => (size_str, None),
    };

    let mut value = u32::from_str(num_str.trim())?;
    
    if let Some(c) = unit_char {
        let unit = SizeUnit::from_char(c).ok_or(ParseSizeError::InvalidUnit)?;
        let multiplier = unit.multiplier();
        
        // Check for overflow before multiplying
        if value > u32::MAX / multiplier {
            return Err(ParseSizeError::Overflow);
        }
        value *= multiplier;
        
        // Check for any remaining characters after the unit
        if size_str[num_str.len() + 1..].trim().len() > 0 {
            return Err(ParseSizeError::ExtraCharacters);
        }
    }

    Ok(value)
}

// Convenience functions for different numeric types
fn str_to_u8(s: &str) -> Result<u8, ParseSizeError> {
    let val = parse_size(s)?;
    if val > u8::MAX as u32 {
        Err(ParseSizeError::Overflow)
    } else {
        Ok(val as u8)
    }
}

fn str_to_u16(s: &str) -> Result<u16, ParseSizeError> {
    let val = parse_size(s)?;
    if val > u16::MAX as u32 {
        Err(ParseSizeError::Overflow)
    } else {
        Ok(val as u16)
    }
}

fn str_to_u32(s: &str) -> Result<u32, ParseSizeError> {
    parse_size(s)
}

fn str_to_i32(s: &str) -> Result<i32, ParseSizeError> {
    let val = i32::from_str(s)?;
    Ok(val)
}

// "atou" convenience functions
fn atou8(s: &str) -> u8 {
    str_to_u8(s).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    })
}

fn atou16(s: &str) -> u16 {
    str_to_u16(s).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    })
}

fn atou32(s: &str) -> u32 {
    str_to_u32(s).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    })
}

fn atoi32(s: &str) -> i32 {
    str_to_i32(s).unwrap_or_else(|e| {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    })
}