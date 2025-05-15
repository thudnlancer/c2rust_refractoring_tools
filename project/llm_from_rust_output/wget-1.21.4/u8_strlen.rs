use std::ffi::CStr;

pub fn u8_strlen(s: &[u8]) -> usize {
    unsafe {
        CStr::from_bytes_with_nul_unchecked(s).to_bytes().len()
    }
}