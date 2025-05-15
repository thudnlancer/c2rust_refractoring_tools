use std::ffi::CStr;
use std::num::ParseIntError;
use std::str::FromStr;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuotingStyle {
    Literal,
    Shell,
    ShellAlways,
    ShellEscape,
    ShellEscapeAlways,
    C,
    CMaybe,
    Escape,
    Locale,
    Clocale,
    Custom,
}

#[derive(Debug, Error)]
pub enum SafeAtoiError {
    #[error("Integer overflow: {0}")]
    Overflow(String),
    #[error("Unexpected suffix {0} on {1}")]
    Suffix(String, String),
    #[error("Expected an integer: {0}")]
    NotAnInteger(String),
    #[error("Failed to parse integer: {0}")]
    ParseInt(#[from] ParseIntError),
}

pub fn safe_atoi(s: &str, style: QuotingStyle) -> Result<i32, SafeAtoiError> {
    let s = s.trim();
    if s.is_empty() {
        return Err(SafeAtoiError::NotAnInteger(s.to_string()));
    }

    let val = i64::from_str(s)?;

    if val > i32::MAX as i64 || val < i32::MIN as i64 {
        return Err(SafeAtoiError::Overflow(s.to_string()));
    }

    // Check for trailing non-whitespace characters
    if let Some(non_digit_idx) = s.find(|c: char| !c.is_ascii_digit() && c != '-') {
        let suffix = &s[non_digit_idx..];
        if !suffix.chars().all(|c| c.is_whitespace()) {
            return Err(SafeAtoiError::Suffix(suffix.trim().to_string(), s.to_string()));
        }
    }

    Ok(val as i32)
}

// Helper functions for FFI compatibility
pub mod ffi {
    use super::*;
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_int};

    #[no_mangle]
    pub extern "C" fn safe_atoi(
        s: *const c_char,
        style: QuotingStyle,
    ) -> c_int {
        unsafe {
            match CStr::from_ptr(s).to_str() {
                Ok(s_str) => match super::safe_atoi(s_str, style) {
                    Ok(val) => val,
                    Err(e) => {
                        // Handle error cases appropriately for C callers
                        -1 // Example error return value
                    }
                },
                Err(_) => -1, // Invalid UTF-8
            }
        }
    }
}