use std::ffi::CString;
use std::os::raw::c_char;

pub fn glp_strtrim(str: &mut CString) -> &mut CString {
    let bytes = str.as_bytes_mut();
    let mut i = bytes.len();
    
    while i > 0 {
        i -= 1;
        if bytes[i] != b' ' {
            break;
        }
        bytes[i] = b'\0';
    }
    
    // Truncate the string at the first null byte
    if let Some(pos) = bytes.iter().position(|&b| b == b'\0') {
        str.as_mut_vec().truncate(pos);
    }
    
    str
}