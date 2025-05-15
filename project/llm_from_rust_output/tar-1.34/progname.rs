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

    let slash_pos = bytes.iter().rposition(|&b| b == b'/');
    let base = match slash_pos {
        Some(pos) => &bytes[pos + 1..],
        None => bytes,
    };

    let new_argv0 = if base.len() >= 7 && bytes.len() >= 7 {
        let libs_prefix = b"/.libs/";
        if bytes[bytes.len() - base.len() - 7..bytes.len() - base.len()] == *libs_prefix {
            if base.starts_with(b"lt-") {
                unsafe {
                    PROGRAM_INVOCATION_SHORT_NAME = argv0.as_ptr().add(bytes.len() - base.len() + 3) as *mut c_char;
                }
                &base[3..]
            } else {
                base
            }
        } else {
            bytes
        }
    } else {
        bytes
    };

    unsafe {
        PROGRAM_NAME = new_argv0.as_ptr() as *const c_char;
        PROGRAM_INVOCATION_NAME = new_argv0.as_ptr() as *mut c_char;
        if PROGRAM_INVOCATION_SHORT_NAME.is_null() {
            PROGRAM_INVOCATION_SHORT_NAME = PROGRAM_INVOCATION_NAME;
        }
    }
}