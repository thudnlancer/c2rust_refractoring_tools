use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    fn last_component(filename: *const c_char) -> *mut c_char;
    fn base_len(filename: *const c_char) -> usize;
}

pub fn strip_trailing_slashes(file: &mut [c_char]) -> bool {
    let base_ptr = unsafe { last_component(file.as_ptr()) };
    let base = unsafe { CStr::from_ptr(base_ptr) }.to_bytes();
    
    let base_start = if base.is_empty() {
        0
    } else {
        unsafe { base_ptr.offset_from(file.as_ptr()) } as usize
    };

    let base_len = unsafe { base_len(file.as_ptr().add(base_start)) };
    let base_end = base_start + base_len;

    let had_slash = if base_end < file.len() {
        file[base_end] != 0
    } else {
        false
    };

    if base_end < file.len() {
        file[base_end] = 0;
    }

    had_slash
}