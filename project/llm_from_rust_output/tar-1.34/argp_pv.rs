use std::ffi::CString;

#[no_mangle]
pub static argp_program_version: Option<&'static std::ffi::CStr> = None;