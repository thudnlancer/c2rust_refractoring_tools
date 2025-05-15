use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn base_len(filename: *const c_char) -> usize;
    fn last_component(filename: *const c_char) -> *mut c_char;
}

pub fn strip_trailing_slashes(file: &mut CString) -> bool {
    let file_ptr = file.as_ptr();
    let mut base = unsafe { last_component(file_ptr) };
    let mut had_slash = false;

    let base_str = unsafe { CStr::from_ptr(base) };
    if base_str.to_bytes().is_empty() {
        base = file.as_ptr() as *mut c_char;
    }

    let base_len = unsafe { base_len(base) };
    let base_lim = unsafe { base.add(base_len) };
    had_slash = unsafe { *base_lim != 0 };

    unsafe {
        *base_lim = 0;
    }

    had_slash
}