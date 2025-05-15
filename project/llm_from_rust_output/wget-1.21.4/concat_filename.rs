use std::ffi::CString;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

pub fn concatenated_filename(
    directory: *const c_char,
    filename: *const c_char,
    suffix: *const c_char,
) -> *mut c_char {
    // Convert C strings to Rust strings safely
    let dir_str = unsafe {
        if directory.is_null() {
            ".".to_string()
        } else {
            std::ffi::CStr::from_ptr(directory)
                .to_string_lossy()
                .into_owned()
        }
    };

    let file_str = unsafe {
        if filename.is_null() {
            return std::ptr::null_mut();
        }
        std::ffi::CStr::from_ptr(filename)
            .to_string_lossy()
            .into_owned()
    };

    let suffix_str = unsafe {
        if suffix.is_null() {
            "".to_string()
        } else {
            std::ffi::CStr::from_ptr(suffix)
                .to_string_lossy()
                .into_owned()
        }
    };

    // Build the path using PathBuf
    let mut path = PathBuf::new();
    if dir_str == "." {
        path.push(&file_str);
    } else {
        path.push(&dir_str);
        if !dir_str.ends_with('/') {
            path.push("/");
        }
        path.push(&file_str);
    }
    path.set_extension(suffix_str);

    // Convert back to C string
    match CString::new(path.to_string_lossy().into_owned()) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}