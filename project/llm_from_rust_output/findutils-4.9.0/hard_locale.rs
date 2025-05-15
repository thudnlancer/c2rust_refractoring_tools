use std::ffi::{CStr, CString};
use std::ptr;

pub fn hard_locale(category: libc::c_int) -> bool {
    let mut locale = [0i8; 257];
    
    let result = unsafe {
        libc::setlocale(
            category,
            ptr::null()
        )
    };
    
    if result.is_null() {
        return false;
    }
    
    let current_locale = unsafe { CStr::from_ptr(result) };
    
    current_locale != CStr::from_bytes_with_nul(b"C\0").unwrap() &&
    current_locale != CStr::from_bytes_with_nul(b"POSIX\0").unwrap()
}