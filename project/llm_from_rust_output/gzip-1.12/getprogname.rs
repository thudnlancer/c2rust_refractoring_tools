use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn getprogname() -> *const libc::c_char {
    unsafe {
        CStr::from_ptr(program_invocation_short_name).as_ptr()
    }
}

extern "C" {
    static program_invocation_short_name: *const libc::c_char;
}