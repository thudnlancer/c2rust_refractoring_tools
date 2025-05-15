use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;

extern "C" {
    fn mdir_name(file: *const c_char) -> *mut c_char;
    fn xalloc_die();
}

pub fn dir_name(file: &CStr) -> CString {
    let result = unsafe { mdir_name(file.as_ptr()) };
    if result.is_null() {
        unsafe { xalloc_die() };
        panic!("Memory allocation failed in dir_name");
    }
    unsafe { CString::from_raw(result) }
}