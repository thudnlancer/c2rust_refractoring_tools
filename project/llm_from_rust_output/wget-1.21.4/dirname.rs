use std::ffi::CStr;
use std::ffi::CString;
use std::ptr;

extern "C" {
    fn mdir_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn xalloc_die();
}

pub fn dir_name(file: &CStr) -> CString {
    let result = unsafe { mdir_name(file.as_ptr()) };
    if result.is_null() {
        unsafe { xalloc_die() };
        // xalloc_die typically doesn't return, but if it does we'll panic
        panic!("xalloc_die returned unexpectedly");
    }
    unsafe { CString::from_raw(result) }
}