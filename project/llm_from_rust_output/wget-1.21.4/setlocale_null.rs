use std::ffi::{CStr, CString};
use std::ptr;

#[repr(i32)]
pub enum LocaleCategory {
    All = 0,
    Collate = 1,
    CType = 2,
    Monetary = 3,
    Numeric = 4,
    Time = 5,
}

pub fn setlocale_null(category: LocaleCategory) -> Option<&'static CStr> {
    unsafe {
        let result = libc::setlocale(category as i32, ptr::null());
        if result.is_null() {
            None
        } else {
            Some(CStr::from_ptr(result))
        }
    }
}

pub fn setlocale_null_r(
    category: LocaleCategory,
    buf: &mut [u8],
) -> Result<(), (i32, Option<CString>)> {
    let locale = match setlocale_null(category) {
        Some(l) => l,
        None => {
            if !buf.is_empty() {
                buf[0] = 0;
            }
            return Err((libc::EINVAL, None));
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
        Err((libc::ERANGE, Some(locale.to_owned())))
    }
}