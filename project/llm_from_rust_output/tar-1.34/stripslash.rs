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
    let base_lim = unsafe { base.offset(base_len as isize) };
    
    let had_slash = unsafe { *base_lim != 0 };
    unsafe { *base_lim = 0 };
    
    had_slash
}

extern "C" {
    fn base_len(filename: *const c_char) -> usize;
    fn last_component(filename: *const c_char) -> *mut c_char;
}