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
    let bytes = name.to_bytes();
    let mut len = bytes.len();

    // Remove trailing slashes
    while len > 1 && bytes[len - 1] == b'/' {
        len -= 1;
    }

    // Special case for root directory
    if len == 1 && bytes[0] == b'/' {
        return 1;
    }

    len
}