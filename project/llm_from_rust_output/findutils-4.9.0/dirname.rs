use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern "C" {
    fn mdir_name(file: *const c_char) -> *mut c_char;
    fn xalloc_die();
}

pub fn dir_name(file: &CStr) -> CString {
    let result_ptr = unsafe { mdir_name(file.as_ptr()) };
    if result_ptr.is_null() {
        unsafe { xalloc_die() };
        // xalloc_die typically doesn't return, but if it does we'll panic
        panic!("Memory allocation failed in dir_name");
    }
    unsafe { CString::from_raw(result_ptr) }
}