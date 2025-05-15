use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

fn dir_len(file: &CStr) -> usize {
    let bytes = file.to_bytes();
    let mut prefix_length = 0;
    let mut length = 0;

    // Handle prefix (like leading slashes)
    if !bytes.is_empty() {
        if bytes[0] == b'/' {
            prefix_length = 1;
            if bytes.len() > 2 && bytes[1] == b'/' && bytes[2] != b'/' {
                prefix_length = 2;
            }
        }
    }

    // Find last component
    let path = Path::new(file.to_str().unwrap());
    let last_component = path.components().last().map(|c| c.as_os_str().len()).unwrap_or(0);
    length = path.as_os_str().len() - last_component;

    // Trim trailing slashes
    while prefix_length < length {
        if bytes[length - 1] != b'/' {
            break;
        }
        length -= 1;
    }

    length
}

fn mdir_name(file: &CStr) -> Option<CString> {
    let mut length = dir_len(file);
    let bytes = file.to_bytes();
    let append_dot = length == 0 || (length == 0 && bytes.len() > 2 && bytes[2] != 0 && bytes[2] != b'/');

    let mut result = Vec::with_capacity(length + append_dot as usize + 1);
    result.extend_from_slice(&bytes[..length]);

    if append_dot {
        result.push(b'.');
    }
    result.push(0);

    CString::new(result).ok()
}