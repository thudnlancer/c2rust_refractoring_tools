use std::ffi::CString;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

#[no_mangle]
pub extern "C" fn mfile_name_concat(
    dir: *const c_char,
    base: *const c_char,
    base_in_result: *mut *mut c_char,
) -> *mut c_char {
    // Convert C strings to Rust strings safely
    let dir_str = unsafe {
        if dir.is_null() {
            return std::ptr::null_mut();
        }
        std::ffi::CStr::from_ptr(dir).to_string_lossy()
    };
    let base_str = unsafe {
        if base.is_null() {
            return std::ptr::null_mut();
        }
        std::ffi::CStr::from_ptr(base).to_string_lossy()
    };

    // Process paths
    let dir_path = Path::new(&*dir_str);
    let base_path = Path::new(&*base_str);

    // Handle path concatenation
    let mut result_path = PathBuf::from(dir_path);
    if !dir_str.ends_with('/') && !base_str.starts_with('/') {
        result_path.push("/");
    }
    result_path.push(base_path);

    // Convert back to C string
    let result = match CString::new(result_path.to_string_lossy().as_bytes()) {
        Ok(s) => s,
        Err(_) => return std::ptr::null_mut(),
    };

    // Handle base_in_result if needed
    if !base_in_result.is_null() {
        let base_ptr = result.as_ptr() as *mut c_char;
        unsafe {
            *base_in_result = base_ptr;
        }
    }

    result.into_raw()
}