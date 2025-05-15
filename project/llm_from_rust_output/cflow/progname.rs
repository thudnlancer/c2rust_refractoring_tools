use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

static mut PROGRAM_NAME: *const c_char = ptr::null();
static mut PROGRAM_INVOCATION_NAME: *mut c_char = ptr::null_mut();
static mut PROGRAM_INVOCATION_SHORT_NAME: *mut c_char = ptr::null_mut();

pub fn set_program_name(argv0: &CStr) {
    let base = argv0.to_bytes()
        .rsplit(|&b| b == b'/')
        .next()
        .unwrap_or(argv0.to_bytes());

    let base_ptr = unsafe { argv0.as_ptr().add(argv0.to_bytes().len() - base.len()) };

    let new_name = if base.len() >= 7 {
        let potential_prefix = unsafe { base_ptr.sub(7) };
        let prefix = unsafe { CStr::from_ptr(potential_prefix) };
        
        if prefix.to_bytes() == b"/.libs/" {
            let base_cstr = unsafe { CStr::from_ptr(base_ptr) };
            
            if base_cstr.to_bytes().starts_with(b"lt-") {
                unsafe { PROGRAM_INVOCATION_SHORT_NAME = base_ptr.add(3) as *mut c_char };
                unsafe { base_ptr.add(3) }
            } else {
                base_ptr
            }
        } else {
            argv0.as_ptr()
        }
    } else {
        argv0.as_ptr()
    };

    unsafe {
        PROGRAM_NAME = new_name;
        PROGRAM_INVOCATION_NAME = new_name as *mut c_char;
    }
}