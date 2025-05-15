use std::ffi::CStr;
use std::os::raw::c_char;

pub type size_t = usize;

pub fn last_component(name: &CStr) -> &[u8] {
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

    &bytes[base..]
}

pub fn base_len(name: &CStr) -> size_t {
    let mut bytes = name.to_bytes();
    
    // Trim trailing slashes
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
    unsafe {
        if name.is_null() {
            return std::ptr::null();
        }
        let cstr = CStr::from_ptr(name);
        let slice = last_component(cstr);
        name.add(cstr.to_bytes().len() - slice.len())
    }
}

#[no_mangle]
pub extern "C" fn base_len_c(name: *const c_char) -> size_t {
    unsafe {
        if name.is_null() {
            return 0;
        }
        let cstr = CStr::from_ptr(name);
        base_len(cstr)
    }
}