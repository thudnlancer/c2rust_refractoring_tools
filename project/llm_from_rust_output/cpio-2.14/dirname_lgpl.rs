use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

pub type size_t = usize;

fn dir_len(file: &CStr) -> size_t {
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
    let last_comp = bytes.iter().rposition(|&b| b != b'/')
        .map(|pos| {
            bytes[..=pos].iter().rposition(|&b| b == b'/')
                .map(|slash_pos| slash_pos + 1)
                .unwrap_or(0)
        })
        .unwrap_or(0);

    length = last_comp;

    // Trim trailing slashes
    while prefix_length < length {
        if bytes[length - 1] != b'/' {
            break;
        }
        length -= 1;
    }

    length
}

fn mdir_name(file: &CStr) -> Option<Box<[c_char]>> {
    let length = dir_len(file);
    let bytes = file.to_bytes();
    
    let append_dot = length == 0 || 
        (length == 0 && bytes.len() > 2 && bytes[2] != 0 && bytes[2] != b'/');

    let mut result = Vec::with_capacity(length + append_dot as usize + 1);
    
    // Copy directory part
    result.extend_from_slice(&bytes[..length]);
    
    // Append dot if needed
    if append_dot {
        result.push(b'.' as c_char);
    }
    
    // Null terminate
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
            let len = boxed.len();
            let ptr = Box::into_raw(boxed) as *mut c_char;
            unsafe { ptr::copy(ptr, ptr, len) };
            ptr
        }
        None => ptr::null_mut(),
    }
}