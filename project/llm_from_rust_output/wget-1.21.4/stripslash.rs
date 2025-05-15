use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    fn last_component(filename: *const c_char) -> *mut c_char;
    fn base_len(filename: *const c_char) -> usize;
}

pub fn strip_trailing_slashes(file: &mut [c_char]) -> bool {
    let base_ptr = unsafe { last_component(file.as_ptr()) };
    let base_slice = unsafe { std::slice::from_raw_parts_mut(base_ptr, file.len()) };

    let base = if base_slice[0] == 0 {
        file
    } else {
        unsafe {
            let len = base_len(base_ptr);
            std::slice::from_raw_parts_mut(base_ptr, len)
        }
    };

    let had_slash = if !base.is_empty() {
        base[base.len() - 1] != 0
    } else {
        false
    };

    if !base.is_empty() {
        base[base.len() - 1] = 0;
    }

    had_slash
}