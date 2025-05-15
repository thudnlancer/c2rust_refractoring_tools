//! Query the name of the current global locale.
//! 
//! This is a safe Rust translation of the original C code from GNU libc.
//! It provides thread-safe functions for querying locale names.

use std::ffi::CStr;
use std::os::raw::c_int;
use std::ptr;
use std::sync::Mutex;
use std::str;
use libc::{self, c_char, size_t, EINVAL, ERANGE};

/// Recommended size of a buffer for a locale name for a single category.
pub const SETLOCALE_NULL_MAX: usize = 256 + 1;

/// Recommended size of a buffer for a locale name with all categories.
pub const SETLOCALE_NULL_ALL_MAX: usize = 148 + 12 * 256 + 1;

lazy_static::lazy_static! {
    static ref LOCALE_LOCK: Mutex<()> = Mutex::new(());
}

/// Android-specific workaround for locale queries
fn setlocale_null_androidfix(category: c_int) -> Option<&'static str> {
    unsafe {
        let result = libc::setlocale(category, ptr::null());
        if result.is_null() {
            match category {
                libc::LC_CTYPE
                | libc::LC_NUMERIC
                | libc::LC_TIME
                | libc::LC_COLLATE
                | libc::LC_MONETARY
                | libc::LC_MESSAGES
                | libc::LC_ALL
                | libc::LC_PAPER
                | libc::LC_NAME
                | libc::LC_ADDRESS
                | libc::LC_TELEPHONE
                | libc::LC_MEASUREMENT => Some("C"),
                _ => None,
            }
        } else {
            Some(str::from_utf8_unchecked(CStr::from_ptr(result).to_bytes()))
        }
    }
}

/// Thread-safe version of setlocale(NULL)
/// 
/// Returns None if the category is invalid
pub fn setlocale_null(category: c_int) -> Option<&'static str> {
    if category == libc::LC_ALL {
        let _guard = LOCALE_LOCK.lock().unwrap();
        setlocale_null_androidfix(libc::LC_ALL)
    } else {
        setlocale_null_androidfix(category)
    }
}

/// Thread-safe version of setlocale that writes to a buffer
/// 
/// Returns Ok(()) on success, Err(EINVAL) for invalid category,
/// or Err(ERANGE) if buffer is too small
pub fn setlocale_null_r(
    category: c_int,
    buf: &mut [u8],
) -> Result<(), c_int> {
    let locale_name = match setlocale_null(category) {
        Some(name) => name,
        None => {
            if !buf.is_empty() {
                buf[0] = 0;
            }
            return Err(EINVAL);
        }
    };

    let bytes = locale_name.as_bytes();
    if bytes.len() < buf.len() {
        buf[..bytes.len()].copy_from_slice(bytes);
        if !buf.is_empty() {
            buf[bytes.len()] = 0;
        }
        Ok(())
    } else {
        if !buf.is_empty() {
            let copy_len = buf.len() - 1;
            buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
            buf[copy_len] = 0;
        }
        Err(ERANGE)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libc;

    #[test]
    fn test_basic() {
        let mut buf = [0u8; SETLOCALE_NULL_MAX];
        assert!(setlocale_null_r(libc::LC_CTYPE, &mut buf).is_ok());
        assert!(buf[0] != 0);

        let mut small_buf = [0u8; 1];
        assert_eq!(setlocale_null_r(libc::LC_CTYPE, &mut small_buf), Err(ERANGE));
        assert_eq!(small_buf[0], 0);

        assert!(setlocale_null(libc::LC_CTYPE).is_some());
        assert!(setlocale_null(9999).is_none());
    }
}