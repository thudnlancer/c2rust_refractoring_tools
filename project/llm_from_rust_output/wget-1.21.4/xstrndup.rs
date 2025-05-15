use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

pub type size_t = usize;

pub fn xstrndup(string: *const c_char, n: size_t) -> *mut c_char {
    if string.is_null() {
        return ptr::null_mut();
    }

    let c_str = unsafe {
        std::ffi::CStr::from_ptr(string)
    };

    let bytes = c_str.to_bytes();
    let len = std::cmp::min(n, bytes.len());
    let truncated = &bytes[..len];

    match CString::new(truncated) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => {
            eprintln!("Failed to allocate memory");
            ptr::null_mut()
        }
    }
}