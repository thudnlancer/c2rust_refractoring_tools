use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

pub type size_t = usize;

pub fn dir_len(file: &CStr) -> size_t {
    let bytes = file.to_bytes();
    let mut prefix_length = 0;
    let mut length = 0;

    // Calculate prefix length
    if !bytes.is_empty() {
        if bytes[0] == b'/' {
            prefix_length = 1;
            if bytes.len() > 2 && bytes[1] == b'/' && bytes[2] != b'/' {
                prefix_length = 2;
            }
        }
    }

    // Find last component
    let last_slash = bytes[prefix_length..]
        .iter()
        .rposition(|&b| b == b'/')
        .map(|pos| pos + prefix_length)
        .unwrap_or(prefix_length);

    length = last_slash + 1;

    // Trim trailing slashes
    while length > prefix_length && bytes[length - 1] == b'/' {
        length -= 1;
    }

    length
}

pub fn mdir_name(file: &CStr) -> Option<Box<[u8]>> {
    let mut length = dir_len(file);
    let bytes = file.to_bytes();
    let append_dot = length == 0 || (length == 0 && bytes.len() > 2 && bytes[2] != 0 && bytes[2] != b'/');

    let mut result = Vec::with_capacity(length + append_dot as usize + 1);
    result.extend_from_slice(&bytes[..length]);

    if append_dot {
        result.push(b'.');
    }
    result.push(0);

    Some(result.into_boxed_slice())
}

#[no_mangle]
pub extern "C" fn dir_len_c(file: *const c_char) -> size_t {
    if file.is_null() {
        return 0;
    }
    unsafe { dir_len(CStr::from_ptr(file)) }
}

#[no_mangle]
pub extern "C" fn mdir_name_c(file: *const c_char) -> *mut c_char {
    if file.is_null() {
        return ptr::null_mut();
    }

    let cstr = unsafe { CStr::from_ptr(file) };
    match mdir_name(cstr) {
        Some(boxed) => {
            let ptr = Box::into_raw(boxed) as *mut u8 as *mut c_char;
            ptr
        }
        None => ptr::null_mut(),
    }
}