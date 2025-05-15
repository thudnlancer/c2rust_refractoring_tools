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

    let argv0 = if base.len() >= 7 && bytes.len() >= 7 && &bytes[bytes.len() - 7..] == b"/.libs/" {
        base
    } else {
        bytes
    };

    let argv0 = if argv0.len() >= 3 && &argv0[0..3] == b"lt-" {
        &argv0[3..]
    } else {
        argv0
    };

    unsafe {
        PROGRAM_NAME = argv0.as_ptr() as *const c_char;
        PROGRAM_INVOCATION_NAME = argv0.as_ptr() as *mut c_char;
        PROGRAM_INVOCATION_SHORT_NAME = PROGRAM_INVOCATION_NAME;
    }
}