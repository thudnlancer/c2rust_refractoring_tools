use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

fn last_component(filename: &[u8]) -> usize {
    let mut pos = filename.len();
    while pos > 0 && filename[pos - 1] == b'/' {
        pos -= 1;
    }
    
    let mut last_slash = pos;
    while last_slash > 0 && filename[last_slash - 1] != b'/' {
        last_slash -= 1;
    }
    
    last_slash
}

pub fn dir_len(file: &[u8]) -> usize {
    let mut prefix_length = 0;
    
    if !file.is_empty() {
        if file[0] == b'/' {
            prefix_length = 1;
            if file.len() > 1 && file[1] == b'/' && (file.len() <= 2 || file[2] != b'/') {
                prefix_length = 2;
            }
        }
    }
    
    let last_comp_pos = last_component(file);
    let mut length = last_comp_pos;
    
    while prefix_length < length && file[length - 1] == b'/' {
        length -= 1;
    }
    
    length
}

pub fn mdir_name(file: &[u8]) -> Vec<u8> {
    let mut length = dir_len(file);
    let append_dot = length == 0 || (length == 0 && file.len() > 2 && file[2] != b'/' && file[2] != 0);
    
    let mut dir = Vec::with_capacity(length + append_dot as usize + 1);
    dir.extend_from_slice(&file[..length]);
    
    if append_dot {
        dir.push(b'.');
    }
    dir.push(0);
    
    dir
}

#[no_mangle]
pub extern "C" fn dir_len_c(file: *const c_char) -> usize {
    unsafe {
        let c_str = CStr::from_ptr(file);
        dir_len(c_str.to_bytes())
    }
}

#[no_mangle]
pub extern "C" fn mdir_name_c(file: *const c_char) -> *mut c_char {
    unsafe {
        let c_str = CStr::from_ptr(file);
        let bytes = c_str.to_bytes();
        let mut result = mdir_name(bytes);
        let ptr = result.as_mut_ptr() as *mut c_char;
        ptr::forget(result);
        ptr
    }
}