use std::ffi::{CStr, CString};
use std::ptr;

pub fn hard_locale(category: i32) -> bool {
    let mut locale = [0i8; 257];
    
    // Safe wrapper around setlocale_null_r
    let result = unsafe {
        setlocale_null_r(
            category,
            locale.as_mut_ptr(),
            std::mem::size_of::<[i8; 257]>() as u64,
        )
    };

    if result != 0 {
        return false;
    }

    // Convert to CStr for safe comparison
    let locale_cstr = unsafe { CStr::from_ptr(locale.as_ptr()) };
    
    locale_cstr != CStr::from_bytes_with_nul(b"C\0").unwrap() &&
    locale_cstr != CStr::from_bytes_with_nul(b"POSIX\0").unwrap()
}

extern "C" {
    fn setlocale_null_r(
        category: libc::c_int,
        buf: *mut libc::c_char,
        bufsize: libc::size_t,
    ) -> libc::c_int;
}