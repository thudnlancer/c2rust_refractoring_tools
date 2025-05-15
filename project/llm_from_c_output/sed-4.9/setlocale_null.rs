//! Query the name of the current global locale.
//! 
//! This is a safe Rust translation of the original C code, maintaining the same functionality
//! while adhering to Rust's safety principles.

use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::ptr;
use std::sync::Mutex;
use lazy_static::lazy_static;

// Constants from the original C code
pub const SETLOCALE_NULL_MAX: usize = 256 + 1;
pub const SETLOCALE_NULL_ALL_MAX: usize = 148 + 12 * 256 + 1;

// Locale categories (from locale.h)
pub const LC_ALL: c_int = 0;
pub const LC_COLLATE: c_int = 1;
pub const LC_CTYPE: c_int = 2;
pub const LC_MONETARY: c_int = 3;
pub const LC_NUMERIC: c_int = 4;
pub const LC_TIME: c_int = 5;
pub const LC_MESSAGES: c_int = 6;

// Additional locale categories that may be available
#[cfg(target_os = "linux")]
pub const LC_PAPER: c_int = 7;
#[cfg(target_os = "linux")]
pub const LC_NAME: c_int = 8;
#[cfg(target_os = "linux")]
pub const LC_ADDRESS: c_int = 9;
#[cfg(target_os = "linux")]
pub const LC_TELEPHONE: c_int = 10;
#[cfg(target_os = "linux")]
pub const LC_MEASUREMENT: c_int = 11;
#[cfg(target_os = "linux")]
pub const LC_IDENTIFICATION: c_int = 12;

lazy_static! {
    static ref LOCALE_LOCK: Mutex<()> = Mutex::new(());
}

extern "C" {
    fn setlocale(category: c_int, locale: *const i8) -> *mut i8;
}

/// Android-specific fix for setlocale returning NULL
fn setlocale_null_androidfix(category: c_int) -> Option<String> {
    let result = unsafe {
        let ptr = setlocale(category, ptr::null());
        if ptr.is_null() {
            None
        } else {
            Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
        }
    };

    #[cfg(target_os = "android")]
    {
        result.or_else(|| {
            match category {
                LC_CTYPE | LC_NUMERIC | LC_TIME | LC_COLLATE | LC_MONETARY | 
                LC_MESSAGES | LC_ALL | LC_PAPER | LC_NAME | LC_ADDRESS | 
                LC_TELEPHONE | LC_MEASUREMENT => Some("C".to_string()),
                _ => None,
            }
        })
    }

    #[cfg(not(target_os = "android"))]
    {
        result
    }
}

/// Thread-safe version of setlocale(NULL)
pub fn setlocale_null(category: c_int) -> Option<String> {
    if category == LC_ALL {
        setlocale_null_r_all()
    } else {
        setlocale_null_r_one(category)
    }
}

/// Thread-safe version of setlocale_r for LC_ALL
fn setlocale_null_r_all() -> Option<String> {
    let buf = vec![0u8; SETLOCALE_NULL_ALL_MAX];
    match setlocale_null_r_impl(LC_ALL, buf) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

/// Thread-safe version of setlocale_r for single categories
fn setlocale_null_r_one(category: c_int) -> Option<String> {
    let buf = vec![0u8; SETLOCALE_NULL_MAX];
    match setlocale_null_r_impl(category, buf) {
        Ok(s) => Some(s),
        Err(_) => None,
    }
}

/// Implementation of setlocale_null_r
fn setlocale_null_r_impl(category: c_int, mut buf: Vec<u8>) -> Result<String, ()> {
    let _lock = LOCALE_LOCK.lock().unwrap();

    if let Some(result) = setlocale_null_androidfix(category) {
        if result.len() < buf.len() {
            buf[..result.len()].copy_from_slice(result.as_bytes());
            buf[result.len()] = 0;
            Ok(result)
        } else {
            let truncated_len = buf.len() - 1;
            buf[..truncated_len].copy_from_slice(&result.as_bytes()[..truncated_len]);
            buf[truncated_len] = 0;
            Err(())
        }
    } else {
        if !buf.is_empty() {
            buf[0] = 0;
        }
        Err(())
    }
}

/// Rust version of setlocale_null_r
pub fn setlocale_null_r(category: c_int, buf: &mut [u8]) -> Result<(), ()> {
    let result = setlocale_null_androidfix(category).ok_or(())?;
    
    if result.len() < buf.len() {
        buf[..result.len()].copy_from_slice(result.as_bytes());
        buf[result.len()] = 0;
        Ok(())
    } else {
        let truncated_len = buf.len().saturating_sub(1);
        buf[..truncated_len].copy_from_slice(&result.as_bytes()[..truncated_len]);
        if !buf.is_empty() {
            buf[truncated_len] = 0;
        }
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setlocale_null() {
        // Just verify it doesn't panic and returns something
        let result = setlocale_null(LC_CTYPE);
        assert!(result.is_some());
        
        let result = setlocale_null(LC_ALL);
        assert!(result.is_some());
    }

    #[test]
    fn test_setlocale_null_r() {
        let mut buf = [0u8; SETLOCALE_NULL_MAX];
        let result = setlocale_null_r(LC_CTYPE, &mut buf);
        assert!(result.is_ok());
        
        let mut small_buf = [0u8; 1];
        let result = setlocale_null_r(LC_CTYPE, &mut small_buf);
        assert!(result.is_err());
    }
}