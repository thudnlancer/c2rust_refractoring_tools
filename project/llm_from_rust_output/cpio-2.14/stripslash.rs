use std::ffi::CStr;
use std::os::raw::c_char;

extern "C" {
    fn last_component(filename: *const c_char) -> *mut c_char;
    fn base_len(filename: *const c_char) -> usize;
}

pub fn strip_trailing_slashes(file: &mut [c_char]) -> bool {
    unsafe {
        let base_ptr = last_component(file.as_ptr());
        let mut base = if *base_ptr == 0 {
            file.as_mut_ptr()
        } else {
            base_ptr
        };

        let base_len = base_len(base);
        let base_lim = base.add(base_len);
        let had_slash = *base_lim != 0;
        *base_lim = 0;
        had_slash
    }
}