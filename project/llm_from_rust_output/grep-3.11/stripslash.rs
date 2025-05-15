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
        &mut file[..]
    } else {
        let base_start = unsafe { base_ptr.offset_from(file.as_ptr()) as usize };
        &mut file[base_start..]
    };

    let base_lim = base.len().min(base_len);
    let had_slash = base_lim < base.len() && unsafe { *base.as_ptr().add(base_lim) } != 0;

    if base_lim < base.len() {
        unsafe { *base.as_mut_ptr().add(base_lim) = 0 };
    }

    had_slash
}