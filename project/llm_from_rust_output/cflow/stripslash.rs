use std::ffi::CStr;
use std::os::raw::c_char;

pub fn strip_trailing_slashes(file: &mut [c_char]) -> bool {
    let file_ptr = file.as_mut_ptr();
    let base_ptr = unsafe { last_component(file_ptr) };
    
    let base = if unsafe { *base_ptr == 0 } {
        file_ptr
    } else {
        base_ptr
    };

    let base_len = unsafe { base_len(base) };
    let base_slice = unsafe { std::slice::from_raw_parts_mut(base, base_len + 1) };
    let base_lim = base_len;

    let had_slash = unsafe { *base_slice.get_unchecked(base_lim) != 0 };
    unsafe { *base_slice.get_unchecked_mut(base_lim) = 0 };

    had_slash
}

extern "C" {
    fn base_len(file: *const c_char) -> usize;
    fn last_component(file: *const c_char) -> *mut c_char;
}