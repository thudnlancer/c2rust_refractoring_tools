use std::ffi::CStr;
use libc::{c_char, nl_item};

pub const CODESET: nl_item = 14;

pub fn locale_charset() -> &'static str {
    let codeset = unsafe {
        let ptr = nl_langinfo(CODESET);
        if ptr.is_null() {
            b"\0".as_ptr() as *const c_char
        } else {
            ptr
        }
    };

    let c_str = unsafe { CStr::from_ptr(codeset) };
    let bytes = c_str.to_bytes();

    if bytes.is_empty() || bytes[0] == 0 {
        "ASCII"
    } else {
        c_str.to_str().unwrap_or("ASCII")
    }
}

extern "C" {
    fn nl_langinfo(__item: nl_item) -> *const c_char;
}