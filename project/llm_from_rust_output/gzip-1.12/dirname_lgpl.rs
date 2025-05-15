use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

fn last_component(filename: &CStr) -> usize {
    let bytes = filename.to_bytes();
    let mut pos = bytes.len();
    
    while pos > 0 && bytes[pos - 1] == b'/' {
        pos -= 1;
    }
    
    while pos > 0 && bytes[pos - 1] != b'/' {
        pos -= 1;
    }
    
    pos
}

pub fn dir_len(file: &CStr) -> usize {
    let bytes = file.to_bytes();
    let mut prefix_length = 0;
    
    if !bytes.is_empty() {
        if bytes[0] == b'/' {
            prefix_length = 1;
            if bytes.len() > 2 && bytes[1] == b'/' && bytes[2] != b'/' {
                prefix_length = 2;
            }
        }
    }
    
    let mut length = last_component(file);
    while prefix_length < length {
        if bytes[length - 1] != b'/' {
            break;
        }
        length -= 1;
    }
    
    length
}

pub fn mdir_name(file: &CStr) -> Option<Vec<u8>> {
    let mut length = dir_len(file);
    let append_dot = length == 0 || (length == 0 && file.to_bytes().len() > 2 && 
        file.to_bytes()[2] != 0 && file.to_bytes()[2] != b'/');
    
    let mut result = Vec::with_capacity(length + append_dot as usize + 1);
    result.extend_from_slice(&file.to_bytes()[..length]);
    
    if append_dot {
        result.push(b'.');
    }
    result.push(0);
    
    Some(result)
}

#[no_mangle]
pub extern "C" fn dir_len_c(file: *const c_char) -> usize {
    unsafe { dir_len(CStr::from_ptr(file)) }
}

#[no_mangle]
pub extern "C" fn mdir_name_c(file: *const c_char) -> *mut c_char {
    let cstr = unsafe { CStr::from_ptr(file) };
    match mdir_name(cstr) {
        Some(mut vec) => {
            let ptr = vec.as_mut_ptr() as *mut c_char;
            std::mem::forget(vec);
            ptr
        }
        None => ptr::null_mut(),
    }
}