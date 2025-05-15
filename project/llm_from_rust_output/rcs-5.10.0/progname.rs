use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::process;

static mut PROGRAM_INVOCATION_NAME: *mut c_char = ptr::null_mut();
static mut PROGRAM_INVOCATION_SHORT_NAME: *mut c_char = ptr::null_mut();

#[no_mangle]
pub static mut program_name: *const c_char = ptr::null();

#[no_mangle]
pub extern "C" fn set_program_name(argv0: *const c_char) {
    unsafe {
        if argv0.is_null() {
            eprintln!("A NULL argv[0] was passed through an exec system call.");
            process::abort();
        }

        let argv0_cstr = CStr::from_ptr(argv0);
        let argv0_bytes = argv0_cstr.to_bytes();
        let argv0_str = std::str::from_utf8_unchecked(argv0_bytes);

        let slash_pos = argv0_str.rfind('/');
        let base = match slash_pos {
            Some(pos) => &argv0_str[pos + 1..],
            None => argv0_str,
        };

        let new_argv0 = if base.len() >= 7 && argv0_str.ends_with("/.libs/") {
            if base.starts_with("lt-") {
                PROGRAM_INVOCATION_SHORT_NAME = argv0.offset(3) as *mut c_char;
                argv0.offset(3)
            } else {
                argv0.offset((argv0_str.len() - base.len()) as isize)
            }
        } else {
            argv0
        };

        program_name = new_argv0;
        PROGRAM_INVOCATION_NAME = new_argv0 as *mut c_char;
    }
}