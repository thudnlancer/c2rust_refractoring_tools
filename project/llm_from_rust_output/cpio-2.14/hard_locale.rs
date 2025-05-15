use std::ffi::CStr;
use libc::{c_int, c_char, c_ulong};

extern "C" {
    fn setlocale_null_r(category: c_int, buf: *mut c_char, bufsize: c_ulong) -> c_int;
}

pub type size_t = c_ulong;

#[no_mangle]
pub fn hard_locale(category: c_int) -> bool {
    let mut locale: [c_char; 257] = [0; 257];
    
    let result = unsafe {
        setlocale_null_r(
            category,
            locale.as_mut_ptr(),
            std::mem::size_of::<[c_char; 257]>() as c_ulong,
        )
    };
    
    if result != 0 {
        return false;
    }
    
    let locale_str = unsafe { CStr::from_ptr(locale.as_ptr()) };
    
    locale_str != CStr::from_bytes_with_nul(b"C\0").unwrap() &&
    locale_str != CStr::from_bytes_with_nul(b"POSIX\0").unwrap()
}