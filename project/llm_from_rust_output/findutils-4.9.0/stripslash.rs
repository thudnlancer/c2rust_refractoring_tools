use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    fn last_component(filename: *const c_char) -> *mut c_char;
    fn base_len(filename: *const c_char) -> usize;
}

pub fn strip_trailing_slashes(file: &mut [c_char]) -> bool {
    let base_ptr = unsafe { last_component(file.as_ptr()) };
    let base_len = unsafe { base_len(file.as_ptr()) };

    let base = if unsafe { *base_ptr == 0 } {
        file.as_mut_ptr()
    } else {
        base_ptr
    };

    let base_slice = unsafe { std::slice::from_raw_parts_mut(base, file.len()) };
    let base_str = unsafe { CStr::from_ptr(base) };
    let base_len = base_str.to_bytes().len();

    let had_slash = if base_len < file.len() {
        unsafe { *base_slice.get_unchecked(base_len) != 0 }
    } else {
        false
    };

    if base_len < file.len() {
        unsafe { *base_slice.get_unchecked_mut(base_len) = 0 };
    }

    had_slash
}