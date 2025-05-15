use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    fn last_component(filename: *const c_char) -> *mut c_char;
    fn base_len(filename: *const c_char) -> usize;
}

pub fn base_name(name: &CStr) -> CString {
    let base_ptr = unsafe { last_component(name.as_ptr()) };
    let base = unsafe { CStr::from_ptr(base_ptr) };
    
    let (length, dotslash_len) = if !base.to_bytes().is_empty() {
        let len = unsafe { base_len(base.as_ptr()) } as isize;
        let extra = if len > 0 && unsafe { *base.as_ptr().offset(len) } == b'/' as c_char {
            1
        } else {
            0
        };
        (len + extra, if cfg!(windows) { 2 } else { 0 })
    } else {
        let len = unsafe { base_len(name.as_ptr()) } as isize;
        (len, 0)
    };

    let mut result = if dotslash_len > 0 {
        let mut s = CString::new(".").unwrap();
        s.push("/");
        s
    } else {
        CString::new("").unwrap()
    };

    let base_str = if !base.to_bytes().is_empty() {
        base.to_bytes()
    } else {
        name.to_bytes()
    };

    let slice = if length > 0 {
        &base_str[..length as usize]
    } else {
        &[]
    };

    result.append(CString::new(slice).unwrap());
    result
}