use std::ffi::CString;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

#[no_mangle]
pub extern "C" fn mfile_name_concat(
    dir: *const c_char,
    base: *const c_char,
    base_in_result: *mut *mut c_char,
) -> *mut c_char {
    if dir.is_null() || base.is_null() {
        return std::ptr::null_mut();
    }

    let dir_cstr = unsafe { std::ffi::CStr::from_ptr(dir) };
    let base_cstr = unsafe { std::ffi::CStr::from_ptr(base) };

    let dir_str = match dir_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let base_str = match base_cstr.to_str() {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    let dir_path = Path::new(dir_str);
    let base_path = Path::new(base_str);

    let last_component = dir_path.file_name().unwrap_or_default();
    let dir_base_len = last_component.to_str().unwrap_or("").len();

    let dir_len = dir_str.len() - dir_base_len;
    let base_len = base_str.len();

    let needs_sep = if dir_base_len > 0 {
        !dir_str.ends_with('/') && !base_str.starts_with('/')
    } else {
        base_str.starts_with('/')
    };

    let sep = if needs_sep {
        if dir_base_len == 0 && base_str.starts_with('/') {
            "."
        } else {
            "/"
        }
    } else {
        ""
    };

    let total_len = dir_len + sep.len() + base_len + 1;
    let mut buf = Vec::with_capacity(total_len);

    buf.extend_from_slice(&dir_str[..dir_len].as_bytes());
    buf.extend_from_slice(sep.as_bytes());
    buf.extend_from_slice(base_str.as_bytes());
    buf.push(0);

    let result = match CString::from_vec_with_nul(buf) {
        Ok(cstr) => cstr.into_raw(),
        Err(_) => return std::ptr::null_mut(),
    };

    if !base_in_result.is_null() {
        unsafe {
            let base_ptr = result.add(dir_len + sep.len());
            *base_in_result = base_ptr;
        }
    }

    result
}