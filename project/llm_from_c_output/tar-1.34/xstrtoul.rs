use std::num::ParseIntError;
use std::str::FromStr;

pub fn xstrtoul(s: &str) -> Result<u64, ParseIntError> {
    let min = 0u64;
    let max = u64::MAX;
    
    match u64::from_str(s) {
        Ok(value) if value >= min && value <= max => Ok(value),
        Ok(_) => Err(u64::from_str("").unwrap_err()), // Simulate overflow error
        Err(e) => Err(e),
    }
}