use std::ffi::{CStr, CString};
use std::os::raw::c_char;

const PATH_SEP: char = '/';

pub fn mt_basename(filename: &CStr) -> &CStr {
    if let Some(bytes) = filename.to_bytes().iter().rposition(|&b| b == PATH_SEP as u8) {
        unsafe { CStr::from_bytes_with_nul_unchecked(&filename.to_bytes()[bytes + 1..]) }
    } else {
        filename
    }
}

pub fn mt_stripexe(filename: &mut CString) {
    let bytes = filename.as_bytes_with_nul();
    if let Some(pos) = bytes.iter().rposition(|&b| b == b'.') {
        if bytes[pos..].eq_ignore_ascii_case(b".exe\0") {
            unsafe {
                let slice = filename.as_mut_vec();
                slice.truncate(pos);
                slice.push(b'\0');
            }
        }
    }
}