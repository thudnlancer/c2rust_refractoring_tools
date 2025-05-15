use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

pub type size_t = usize;

#[no_mangle]
pub extern "C" fn xstrndup(string: *const c_char, n: size_t) -> *mut c_char {
    if string.is_null() {
        return ptr::null_mut();
    }

    let slice = unsafe { std::slice::from_raw_parts(string, n) };
    let c_str = unsafe { std::ffi::CStr::from_ptr(string) };
    
    match CString::new(unsafe { c_str.to_bytes() }) {
        Ok(s) => s.into_raw(),
        Err(_) => {
            eprintln!("Failed to duplicate string");
            ptr::null_mut()
        }
    }
}