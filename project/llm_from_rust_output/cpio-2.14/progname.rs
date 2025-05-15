use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;

static mut PROGRAM_NAME: *const c_char = ptr::null();
static mut PROGRAM_INVOCATION_NAME: *mut c_char = ptr::null_mut();
static mut PROGRAM_INVOCATION_SHORT_NAME: *mut c_char = ptr::null_mut();

pub fn set_program_name(argv0: &CStr) {
    let path = Path::new(argv0.to_str().expect("Invalid UTF-8 in program name"));
    
    let base = path.file_name()
        .map(|n| n.to_str().expect("Invalid UTF-8 in program name"))
        .unwrap_or_else(|| argv0.to_str().expect("Invalid UTF-8 in program name"));

    let base_ptr = if base.len() >= 7 && path.to_str().unwrap().ends_with("/.libs/") {
        if base.starts_with("lt-") {
            &base[3..]
        } else {
            base
        }
    } else {
        base
    };

    unsafe {
        PROGRAM_NAME = argv0.as_ptr();
        PROGRAM_INVOCATION_NAME = argv0.as_ptr() as *mut c_char;
        PROGRAM_INVOCATION_SHORT_NAME = if base_ptr.len() != base.len() {
            base_ptr.as_ptr() as *mut c_char
        } else {
            argv0.as_ptr() as *mut c_char
        };
    }
}