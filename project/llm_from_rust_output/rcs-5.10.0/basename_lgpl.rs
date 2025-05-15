use std::ffi::CStr;
use std::os::raw::c_char;

pub fn last_component(name: &CStr) -> &CStr {
    let bytes = name.to_bytes();
    let mut base = 0;
    let mut last_was_slash = false;

    // Skip leading slashes
    while base < bytes.len() && bytes[base] == b'/' {
        base += 1;
    }

    let mut p = base;
    while p < bytes.len() {
        if bytes[p] == b'/' {
            last_was_slash = true;
        } else if last_was_slash {
            base = p;
            last_was_slash = false;
        }
        p += 1;
    }

    unsafe { CStr::from_bytes_with_nul_unchecked(&bytes[base..]) }
}

pub fn base_len(name: &CStr) -> usize {
    let mut bytes = name.to_bytes();
    
    // Remove trailing slashes
    while bytes.len() > 1 && bytes[bytes.len() - 1] == b'/' {
        bytes = &bytes[..bytes.len() - 1];
    }

    // Special case for root directory
    if bytes.len() == 1 && bytes[0] == b'/' {
        return 1;
    }

    bytes.len()
}

#[no_mangle]
pub extern "C" fn last_component_c(name: *const c_char) -> *const c_char {
    let cstr = unsafe { CStr::from_ptr(name) };
    last_component(cstr).as_ptr()
}

#[no_mangle]
pub extern "C" fn base_len_c(name: *const c_char) -> usize {
    let cstr = unsafe { CStr::from_ptr(name) };
    base_len(cstr)
}