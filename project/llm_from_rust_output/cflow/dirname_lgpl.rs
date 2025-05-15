use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

pub fn dir_len(file: &CStr) -> usize {
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
    length = bytes.iter().rposition(|&c| c != b'/').map_or(0, |pos| pos + 1);

    // Adjust length to not include trailing slashes
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
    
    // Copy directory part
    result.extend_from_slice(&bytes[..length]);
    
    // Append dot if needed
    if append_dot {
        result.push(b'.');
    }
    
    // Null terminate
    result.push(0);
    
    Some(result.into_boxed_slice())
}