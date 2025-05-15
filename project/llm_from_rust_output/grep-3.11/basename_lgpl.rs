use std::ffi::CStr;
use std::os::raw::c_char;

pub type size_t = usize;

pub fn last_component(name: &[u8]) -> &[u8] {
    let mut base = name;
    let mut last_was_slash = false;
    
    // Skip leading slashes
    while !base.is_empty() && base[0] == b'/' {
        base = &base[1..];
    }
    
    let mut p = base;
    let mut last_component_start = 0;
    
    for (i, &c) in p.iter().enumerate() {
        if c == b'/' {
            last_was_slash = true;
        } else if last_was_slash {
            last_component_start = i;
            last_was_slash = false;
        }
    }
    
    &base[last_component_start..]
}

pub fn base_len(name: &[u8]) -> size_t {
    let mut len = name.len();
    
    // Remove trailing slashes
    while len > 1 && name[len - 1] == b'/' {
        len -= 1;
    }
    
    // Handle special case of root directory
    if len == 1 && name[0] == b'/' {
        return 1;
    }
    
    len
}

#[no_mangle]
pub extern "C" fn last_component_c(name: *const c_char) -> *const c_char {
    let name = unsafe { CStr::from_ptr(name) };
    let bytes = name.to_bytes();
    let component = last_component(bytes);
    unsafe { component.as_ptr() as *const c_char }
}

#[no_mangle]
pub extern "C" fn base_len_c(name: *const c_char) -> size_t {
    let name = unsafe { CStr::from_ptr(name) };
    base_len(name.to_bytes())
}