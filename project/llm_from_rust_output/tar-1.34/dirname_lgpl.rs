use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

pub type size_t = usize;

fn last_component(filename: &[u8]) -> &[u8] {
    let mut last_slash = None;
    for (i, &c) in filename.iter().enumerate() {
        if c == b'/' {
            last_slash = Some(i);
        }
    }
    match last_slash {
        Some(pos) => &filename[pos + 1..],
        None => filename,
    }
}

pub fn dir_len(file: &[u8]) -> size_t {
    let mut prefix_length = 0;
    let mut length;

    // Handle prefix cases
    if !file.is_empty() {
        if file[0] == b'/' {
            prefix_length = 1;
            if file.len() > 1 && file[1] == b'/' && (file.len() <= 2 || file[2] != b'/') {
                prefix_length = 2;
            }
        }
    }

    let last_comp = last_component(file);
    length = file.len() - last_comp.len();

    // Remove trailing slashes
    while prefix_length < length {
        if file[length - 1] != b'/' {
            break;
        }
        length -= 1;
    }

    length
}

pub fn mdir_name(file: &[u8]) -> Option<Vec<u8>> {
    let mut length = dir_len(file);
    let append_dot = length == 0 || (length == 0 && file.len() > 2 && file[2] != b'\0' && file[2] != b'/');

    let mut dir = Vec::with_capacity(length + append_dot as usize + 1);
    dir.extend_from_slice(&file[..length]);

    if append_dot {
        dir.push(b'.');
    }
    dir.push(b'\0');

    Some(dir)
}

#[no_mangle]
pub extern "C" fn dir_len_c(file: *const c_char) -> size_t {
    if file.is_null() {
        return 0;
    }
    unsafe {
        let c_str = CStr::from_ptr(file);
        dir_len(c_str.to_bytes())
    }
}

#[no_mangle]
pub extern "C" fn mdir_name_c(file: *const c_char) -> *mut c_char {
    if file.is_null() {
        return ptr::null_mut();
    }

    let bytes = unsafe { CStr::from_ptr(file).to_bytes() };
    match mdir_name(bytes) {
        Some(mut vec) => {
            let ptr = vec.as_mut_ptr() as *mut c_char;
            std::mem::forget(vec);
            ptr
        }
        None => ptr::null_mut(),
    }
}