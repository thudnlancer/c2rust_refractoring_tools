//! Thread-safe replacements for standard C functions

use std::ffi::CStr;
use std::os::raw::c_int;
use std::ptr;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(windows)]
use windows_sys::Win32::Foundation::GetLastError;

thread_local! {
    static STRTOK_PTR: OnceLock<*mut u8> = OnceLock::new();
}

/// Thread-safe replacement for gmtime
pub fn xgmtime(timer: &SystemTime) -> Option<time::Tm> {
    let duration = timer.duration_since(UNIX_EPOCH).ok()?;
    time::at(time::Timespec::new(duration.as_secs() as i64, duration.subsec_nanos() as i32))
}

/// Thread-safe replacement for strerror
pub fn xstrerr(errnum: c_int) -> String {
    std::error::Error::description(&std::io::Error::from_raw_os_error(errnum)).to_string()
}

/// Thread-safe replacement for strtok
pub fn xstrtok(s1: Option<&mut [u8]>, s2: &CStr) -> Option<&mut [u8]> {
    STRTOK_PTR.with(|ptr_cell| {
        let s1_ptr = match s1 {
            Some(s) => s.as_mut_ptr(),
            None => {
                let ptr = ptr_cell.get()?;
                if ptr.is_null() {
                    return None;
                }
                *ptr
            }
        };

        let result = unsafe {
            let mut saveptr = ptr::null_mut();
            let tok = libc::strtok_r(s1_ptr, s2.as_ptr(), &mut saveptr);
            if tok.is_null() {
                None
            } else {
                Some(std::slice::from_raw_parts_mut(tok, libc::strlen(tok)))
            }
        };

        if let Some(ptr) = ptr_cell.get() {
            unsafe {
                *ptr = libc::strtok_r(ptr::null_mut(), ptr::null(), &mut ptr::null_mut());
            }
        } else {
            ptr_cell.set(ptr::null_mut()).ok()?;
        }

        result
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_xgmtime() {
        let now = SystemTime::now();
        assert!(xgmtime(&now).is_some());
    }

    #[test]
    fn test_xstrerr() {
        let err = xstrerr(libc::EINVAL);
        assert!(!err.is_empty());
    }

    #[test]
    fn test_xstrtok() {
        let mut s = b"hello world\0".to_vec();
        let delim = CString::new(" ").unwrap();
        
        let first = xstrtok(Some(&mut s), &delim);
        assert_eq!(first, Some(b"hello\0".as_mut_slice()));
        
        let second = xstrtok(None, &delim);
        assert_eq!(second, Some(b"world\0".as_mut_slice()));
        
        let none = xstrtok(None, &delim);
        assert_eq!(none, None);
    }
}