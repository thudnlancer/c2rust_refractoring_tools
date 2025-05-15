//! Convert integers to printable strings
//!
//! This module provides functions to convert various integer types to their string representations.
//! The implementation is safe and follows Rust's best practices.

use std::fmt::Write;

/// Converts an `intmax_t` integer to a printable string.
/// Returns a `Result` containing the string or an error if writing fails.
#[must_use]
pub fn imaxtostr(value: i64, buffer: &mut String) -> Result<(), std::fmt::Error> {
    write!(buffer, "{}", value)
}

/// Converts an `int` integer to a printable string.
/// Returns a `Result` containing the string or an error if writing fails.
#[must_use]
pub fn inttostr(value: i32, buffer: &mut String) -> Result<(), std::fmt::Error> {
    write!(buffer, "{}", value)
}

/// Converts an `off_t` integer to a printable string.
/// Returns a `Result` containing the string or an error if writing fails.
#[must_use]
pub fn offtostr(value: i64, buffer: &mut String) -> Result<(), std::fmt::Error> {
    write!(buffer, "{}", value)
}

/// Converts an `unsigned int` integer to a printable string.
/// Returns a `Result` containing the string or an error if writing fails.
#[must_use]
pub fn uinttostr(value: u32, buffer: &mut String) -> Result<(), std::fmt::Error> {
    write!(buffer, "{}", value)
}

/// Converts a `uintmax_t` integer to a printable string.
/// Returns a `Result` containing the string or an error if writing fails.
#[must_use]
pub fn umaxtostr(value: u64, buffer: &mut String) -> Result<(), std::fmt::Error> {
    write!(buffer, "{}", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_int_conversions() {
        let mut buffer = String::new();
        
        assert!(inttostr(42, &mut buffer).is_ok());
        assert_eq!(buffer, "42");
        buffer.clear();
        
        assert!(inttostr(-1, &mut buffer).is_ok());
        assert_eq!(buffer, "-1");
        buffer.clear();
        
        assert!(uinttostr(42, &mut buffer).is_ok());
        assert_eq!(buffer, "42");
        buffer.clear();
        
        assert!(imaxtostr(i64::MAX, &mut buffer).is_ok());
        assert_eq!(buffer, i64::MAX.to_string());
        buffer.clear();
        
        assert!(umaxtostr(u64::MAX, &mut buffer).is_ok());
        assert_eq!(buffer, u64::MAX.to_string());
    }
}