//! Query the name of the current global locale.
//! 
//! This implementation provides thread-safe versions of setlocale functionality.

use std::ffi::CStr;
use std::os::raw::c_int;
use std::ptr;
use std::sync::Mutex;
use std::str;

#[cfg(windows)]
use winapi::um::winnls::GetThreadLocale;
#[cfg(windows)]
use widestring::WideCString;

/// Recommended size of a buffer for a locale name for a single category.
pub const SETLOCALE_NULL_MAX: usize = 256 + 1;

/// Recommended size of a buffer for a locale name with all categories.
pub const SETLOCALE_NULL_ALL_MAX: usize = 148 + 12 * 256 + 1;

/// Thread-safe version of setlocale(NULL)
pub fn setlocale_null(category: c_int) -> Option<String> {
    // Use Mutex to ensure thread safety
    lazy_static::lazy_static! {
        static ref LOCALE_MUTEX: Mutex<()> = Mutex::new(());
    }
    
    let _guard = LOCALE_MUTEX.lock().unwrap();
    
    unsafe {
        let result = libc::setlocale(category, ptr::null());
        if result.is_null() {
            None
        } else {
            Some(CStr::from_ptr(result).to_string_lossy().into_owned())
        }
    }
}

/// Thread-safe version of setlocale_r
pub fn setlocale_null_r(category: c_int, buf: &mut [u8]) -> Result<usize, ()> {
    if let Some(locale) = setlocale_null(category) {
        let bytes = locale.as_bytes();
        if bytes.len() + 1 > buf.len() {
            return Err(());
        }
        
        buf[..bytes.len()].copy_from_slice(bytes);
        buf[bytes.len()] = 0;
        Ok(bytes.len())
    } else {
        Err(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::raw::c_int;

    #[test]
    fn test_setlocale_null() {
        let locale = setlocale_null(libc::LC_ALL);
        assert!(locale.is_some());
        println!("Current locale: {:?}", locale);
    }

    #[test]
    fn test_setlocale_null_r() {
        let mut buf = [0u8; SETLOCALE_NULL_MAX];
        let result = setlocale_null_r(libc::LC_ALL, &mut buf);
        assert!(result.is_ok());
        let len = result.unwrap();
        let locale_str = str::from_utf8(&buf[..len]).unwrap();
        println!("Current locale: {}", locale_str);
    }
}