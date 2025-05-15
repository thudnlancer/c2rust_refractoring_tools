//! Query the name of the current global locale.
//! 
//! This is a Rust translation of the original C code from GNU libc.

use std::ffi::{CStr, CString};
use std::os::raw::c_int;
use std::ptr;
use std::sync::Mutex;
use std::str;
use libc::{self, c_char, EINVAL, ERANGE, LC_ALL, LC_COLLATE, LC_CTYPE, LC_MESSAGES, 
           LC_MONETARY, LC_NUMERIC, LC_TIME};

// Constants from the original C code
pub const SETLOCALE_NULL_MAX: usize = 256 + 1;
pub const SETLOCALE_NULL_ALL_MAX: usize = 148 + 12 * 256 + 1;

// Thread-safe wrapper for setlocale
lazy_static::lazy_static! {
    static ref SETLOCALE_LOCK: Mutex<()> = Mutex::new(());
}

/// Android-specific workaround for setlocale returning NULL
fn setlocale_null_androidfix(category: c_int) -> Option<String> {
    unsafe {
        let result = libc::setlocale(category, ptr::null());
        if result.is_null() {
            match category {
                LC_CTYPE | LC_NUMERIC | LC_TIME | LC_COLLATE | LC_MONETARY | 
                LC_MESSAGES | LC_ALL => Some("C".to_string()),
                _ => None,
            }
        } else {
            Some(CStr::from_ptr(result).to_string_lossy().into_owned())
        }
    }
}

/// Thread-safe version of setlocale(NULL)
pub fn setlocale_null(category: c_int) -> Option<String> {
    if category == LC_ALL {
        // For LC_ALL, we need to handle it specially
        let _guard = SETLOCALE_LOCK.lock().unwrap();
        setlocale_null_androidfix(LC_ALL)
    } else {
        // For other categories, we can use thread-local storage
        thread_local! {
            static BUFFER: [u8; SETLOCALE_NULL_MAX] = [0; SETLOCALE_NULL_MAX];
        }
        
        BUFFER.with(|buf| {
            let mut result = None;
            let _guard = SETLOCALE_LOCK.lock().unwrap();
            
            if let Ok(s) = setlocale_null_r(category, buf) {
                result = Some(s.to_string());
            }
            
            result
        })
    }
}

/// Thread-safe version of setlocale_r(NULL)
pub fn setlocale_null_r(category: c_int, buf: &mut [u8]) -> Result<&str, c_int> {
    let _guard = SETLOCALE_LOCK.lock().unwrap();
    
    if let Some(locale) = setlocale_null_androidfix(category) {
        if locale.len() + 1 > buf.len() {
            // Buffer too small - copy what we can and return ERANGE
            let copy_len = buf.len().saturating_sub(1);
            buf[..copy_len].copy_from_slice(&locale.as_bytes()[..copy_len]);
            if !buf.is_empty() {
                buf[copy_len] = 0;
            }
            Err(ERANGE)
        } else {
            // Buffer is large enough
            buf[..locale.len()].copy_from_slice(locale.as_bytes());
            buf[locale.len()] = 0;
            Ok(unsafe { CStr::from_ptr(buf.as_ptr() as *const c_char) }
                .to_str()
                .unwrap())
        }
    } else {
        // Invalid category
        if !buf.is_empty() {
            buf[0] = 0;
        }
        Err(EINVAL)
    }
}

// Additional locale categories (if available)
#[cfg(feature = "extended_locale")]
mod extended {
    use libc::{LC_ADDRESS, LC_IDENTIFICATION, LC_MEASUREMENT, LC_NAME, LC_PAPER, LC_TELEPHONE};
    
    impl LocaleCategory {
        pub const ADDRESS: Self = Self(LC_ADDRESS);
        pub const IDENTIFICATION: Self = Self(LC_IDENTIFICATION);
        pub const MEASUREMENT: Self = Self(LC_MEASUREMENT);
        pub const NAME: Self = Self(LC_NAME);
        pub const PAPER: Self = Self(LC_PAPER);
        pub const TELEPHONE: Self = Self(LC_TELEPHONE);
    }
}

/// Wrapper enum for locale categories
#[derive(Debug, Copy, Clone)]
pub enum LocaleCategory {
    All = LC_ALL as isize,
    Collate = LC_COLLATE as isize,
    Ctype = LC_CTYPE as isize,
    Messages = LC_MESSAGES as isize,
    Monetary = LC_MONETARY as isize,
    Numeric = LC_NUMERIC as isize,
    Time = LC_TIME as isize,
    #[cfg(feature = "extended_locale")]
    Address = extended::LC_ADDRESS as isize,
    #[cfg(feature = "extended_locale")]
    Identification = extended::LC_IDENTIFICATION as isize,
    #[cfg(feature = "extended_locale")]
    Measurement = extended::LC_MEASUREMENT as isize,
    #[cfg(feature = "extended_locale")]
    Name = extended::LC_NAME as isize,
    #[cfg(feature = "extended_locale")]
    Paper = extended::LC_PAPER as isize,
    #[cfg(feature = "extended_locale")]
    Telephone = extended::LC_TELEPHONE as isize,
}

impl LocaleCategory {
    /// Get the current locale setting for this category
    pub fn get(self) -> Option<String> {
        setlocale_null(self as c_int)
    }
    
    /// Get the current locale setting into a buffer
    pub fn get_r(self, buf: &mut [u8]) -> Result<&str, c_int> {
        setlocale_null_r(self as c_int, buf)
    }
}