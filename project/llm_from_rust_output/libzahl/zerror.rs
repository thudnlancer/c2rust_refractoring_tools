use std::ffi::CStr;
use std::ptr;
use std::os::raw::{c_char, c_int};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZError {
    ErrnoSet,
    ZeroPowZero,
    ZeroDivZero,
    DivZero,
    Negative,
    InvalidRadix,
}

static mut LIBZAHL_ERROR: c_int = 0;

fn get_libzahl_error() -> c_int {
    unsafe { LIBZAHL_ERROR }
}

fn set_libzahl_error(err: c_int) {
    unsafe { LIBZAHL_ERROR = err };
}

pub fn zerror(desc: Option<&mut &'static CStr>) -> ZError {
    let libzahl_error = get_libzahl_error();
    
    if libzahl_error >= 0 {
        if let Some(desc_ptr) = desc {
            let err_str = unsafe { CStr::from_ptr(libc::strerror(libzahl_error)) };
            *desc_ptr = err_str;
        }
        unsafe { *libc::__errno_location() = libzahl_error };
        return ZError::ErrnoSet;
    }

    let error_code = -libzahl_error;
    let error = match error_code {
        1 => ZError::ZeroPowZero,
        2 => ZError::ZeroDivZero,
        3 => ZError::DivZero,
        4 => ZError::Negative,
        5 => ZError::InvalidRadix,
        _ => panic!("Invalid libzahl error code"),
    };

    if let Some(desc_ptr) = desc {
        let msg = match error {
            ZError::ZeroPowZero => CStr::from_bytes_with_nul(b"indeterminate form: 0:th power of 0\0").unwrap(),
            ZError::ZeroDivZero => CStr::from_bytes_with_nul(b"indeterminate form: 0 divided by 0\0").unwrap(),
            ZError::DivZero => CStr::from_bytes_with_nul(b"undefined result: division by 0\0").unwrap(),
            ZError::Negative => CStr::from_bytes_with_nul(b"argument must be non-negative\0").unwrap(),
            _ => panic!("Unexpected error code with description"),
        };
        *desc_ptr = msg;
    }

    error
}