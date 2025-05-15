use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

static mut PROGRAM_NAME: *const c_char = ptr::null();
static mut PROGRAM_INVOCATION_NAME: *mut c_char = ptr::null_mut();
static mut PROGRAM_INVOCATION_SHORT_NAME: *mut c_char = ptr::null_mut();

pub fn set_program_name(argv0: &CStr) {
    let bytes = argv0.to_bytes();
    if bytes.is_empty() {
        eprintln!("A NULL argv[0] was passed through an exec system call.");
        std::process::abort();
    }

    let base = match bytes.iter().rposition(|&b| b == b'/') {
        Some(pos) => &bytes[pos + 1..],
        None => bytes,
    };

    let argv0 = if base.len() >= 7 && &bytes[bytes.len() - base.len() - 7..bytes.len() - base.len()] == b"/.libs/" {
        if base.starts_with(b"lt-") {
            unsafe {
                PROGRAM_INVOCATION_SHORT_NAME = CStr::from_bytes_with_nul_unchecked(&base[3..]).as_ptr() as *mut c_char;
            }
            &base[3..]
        } else {
            base
        }
    } else {
        bytes
    };

    unsafe {
        PROGRAM_NAME = argv0.as_ptr() as *const c_char;
        PROGRAM_INVOCATION_NAME = argv0.as_ptr() as *mut c_char;
        if PROGRAM_INVOCATION_SHORT_NAME.is_null() {
            PROGRAM_INVOCATION_SHORT_NAME = PROGRAM_INVOCATION_NAME;
        }
    }
}