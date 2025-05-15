use std::ffi::CStr;
use std::os::raw::c_char;

pub type size_t = usize;

fn field_length(str: &[u8], separators: &[u8]) -> size_t {
    if !separators.is_empty() {
        if let Some(pos) = str.iter().position(|&c| separators.contains(&c)) {
            return pos;
        }
    }
    str.len()
}

#[no_mangle]
pub extern "C" fn splitstring(
    s: *const c_char,
    separators: *const c_char,
    first: bool,
    pos: *mut size_t,
    len: *mut size_t,
) -> bool {
    unsafe {
        let s_cstr = CStr::from_ptr(s);
        let separators_cstr = CStr::from_ptr(separators);
        
        let s_bytes = s_cstr.to_bytes();
        let separators_bytes = separators_cstr.to_bytes();
        
        let current_pos = if first {
            0
        } else {
            let old_pos = *pos;
            let old_len = *len;
            let new_pos = old_pos + old_len;
            
            if new_pos >= s_bytes.len() {
                return false;
            }
            
            new_pos + 1
        };
        
        if current_pos >= s_bytes.len() {
            return false;
        }
        
        let remaining = &s_bytes[current_pos..];
        let length = field_length(remaining, separators_bytes);
        
        *pos = current_pos;
        *len = length;
        
        true
    }
}