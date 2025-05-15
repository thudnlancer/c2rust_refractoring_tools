use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

#[derive(Debug, PartialEq)]
pub enum SetLocaleError {
    InvalidCategory,
    BufferTooSmall,
}

pub fn setlocale_null(category: i32) -> Option<&'static CStr> {
    let result = unsafe { libc::setlocale(category, ptr::null()) };
    if result.is_null() {
        None
    } else {
        Some(unsafe { CStr::from_ptr(result) })
    }
}

pub fn setlocale_null_r(
    category: i32,
    buf: &mut [u8],
) -> Result<(), SetLocaleError> {
    let locale = match setlocale_null(category) {
        Some(l) => l,
        None => {
            if !buf.is_empty() {
                buf[0] = 0;
            }
            return Err(SetLocaleError::InvalidCategory);
        }
    };

    let bytes = locale.to_bytes_with_nul();
    if bytes.len() <= buf.len() {
        buf[..bytes.len()].copy_from_slice(bytes);
        Ok(())
    } else {
        if !buf.is_empty() {
            let copy_len = buf.len() - 1;
            buf[..copy_len].copy_from_slice(&bytes[..copy_len]);
            buf[copy_len] = 0;
        }
        Err(SetLocaleError::BufferTooSmall)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_setlocale_null() {
        let result = setlocale_null(libc::LC_ALL);
        assert!(result.is_some());
    }

    #[test]
    fn test_setlocale_null_r() {
        let mut buf = [0u8; 256];
        let result = setlocale_null_r(libc::LC_ALL, &mut buf);
        assert_eq!(result, Ok(()));
    }
}