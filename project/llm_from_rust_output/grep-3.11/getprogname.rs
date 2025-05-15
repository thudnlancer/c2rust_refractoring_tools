use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn getprogname() -> *const libc::c_char {
    unsafe {
        program_invocation_short_name
    }
}

extern "C" {
    static program_invocation_short_name: *const libc::c_char;
}