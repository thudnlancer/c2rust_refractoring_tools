//! Query the name of the current global locale.
//!
//! This is a safe Rust translation of the original C code from GNU libc.

use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::ptr;
use std::sync::Mutex;

// Recommended buffer sizes
pub const SETLOCALE_NULL_MAX: usize = 256 + 1;
pub const SETLOCALE_NULL_ALL_MAX: usize = 148 + 12 * 256 + 1;

// Locale categories (mirroring C's LC_* constants)
#[repr(i32)]
pub enum LocaleCategory {
    All = 0,
    Collate = 1,
    Ctype = 2,
    Messages = 3,
    Monetary = 4,
    Numeric = 5,
    Time = 6,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    Paper = 7,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    Name = 8,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    Address = 9,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    Telephone = 10,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    Measurement = 11,
    #[cfg(any(target_os = "linux", target_os = "android"))]
    Identification = 12,
}

// Thread-safe global lock for setlocale operations
lazy_static::lazy_static! {
    static ref SETLOCALE_LOCK: Mutex<()> = Mutex::new(());
}

/// Safe wrapper around setlocale(null)
fn setlocale_null_androidfix(category: c_int) -> Option<String> {
    let result = unsafe {
        let ptr = libc::setlocale(category, ptr::null());
        if ptr.is_null() {
            None
        } else {
            Some(CStr::from_ptr(ptr).to_string_lossy().into_owned())
        }
    };

    #[cfg(target_os = "android")]
    let result = result.or_else(|| {
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
            | libc::LC_MEASUREMENT => Some("C".to_string()),
            _ => None,
        }
    });

    result
}

/// Thread-safe version of setlocale_null_unlocked
fn setlocale_null_unlocked(category: c_int, buf: &mut [u8]) -> Result<(), std::io::Error> {
    let result = setlocale_null_androidfix(category).ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid locale category")
    })?;

    let bytes = result.as_bytes();
    if bytes.len() + 1 > buf.len() {
        if !buf.is_empty() {
            let copy_len = std::cmp::min(buf.len() - 1, bytes.len());
            buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
            buf[copy_len] = 0;
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "buffer too small",
        ))
    } else {
        buf[..bytes.len()].copy_from_slice(bytes);
        buf[bytes.len()] = 0;
        Ok(())
    }
}

/// Thread-safe version of setlocale_null_r
pub fn setlocale_null_r(category: c_int, buf: &mut [u8]) -> Result<(), std::io::Error> {
    let _guard = SETLOCALE_LOCK.lock().unwrap();
    setlocale_null_unlocked(category, buf)
}

/// Thread-safe version of setlocale_null
pub fn setlocale_null(category: c_int) -> Option<String> {
    if category == libc::LC_ALL {
        let _guard = SETLOCALE_LOCK.lock().unwrap();
        setlocale_null_androidfix(libc::LC_ALL)
    } else {
        let mut buf = [0u8; SETLOCALE_NULL_MAX];
        match setlocale_null_r(category, &mut buf) {
            Ok(_) => Some(unsafe { CStr::from_ptr(buf.as_ptr() as *const _) }
                .to_string_lossy()
                .into_owned()),
            Err(e) if e.kind() == std::io::ErrorKind::InvalidInput => None,
            Err(_) => Some("C".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_setlocale_null() {
        let result = setlocale_null(libc::LC_ALL);
        assert!(result.is_some());

        let result = setlocale_null(libc::LC_CTYPE);
        assert!(result.is_some());

        let result = setlocale_null(-1); // invalid category
        assert!(result.is_none());
    }

    #[test]
    fn test_setlocale_null_r() {
        let mut buf = [0u8; SETLOCALE_NULL_MAX];
        let result = setlocale_null_r(libc::LC_CTYPE, &mut buf);
        assert!(result.is_ok());

        let mut small_buf = [0u8; 1];
        let result = setlocale_null_r(libc::LC_CTYPE, &mut small_buf);
        assert!(result.is_err());
    }
}