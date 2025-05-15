use std::ffi::CString;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

#[no_mangle]
pub extern "C" fn concatenated_filename(
    directory: *const c_char,
    filename: *const c_char,
    suffix: *const c_char,
) -> *mut c_char {
    let dir = unsafe { CString::from_raw(directory as *mut c_char) };
    let dir_str = dir.to_string_lossy();
    let file = unsafe { CString::from_raw(filename as *mut c_char) };
    let file_str = file.to_string_lossy();
    
    let suffix_str = if !suffix.is_null() {
        unsafe { CString::from_raw(suffix as *mut c_char) }.to_string_lossy()
    } else {
        "".into()
    };

    let mut path = if dir_str == "." {
        PathBuf::new()
    } else {
        let mut p = PathBuf::from(&*dir_str);
        if !p.to_string_lossy().ends_with('/') {
            p.push("/");
        }
        p
    };

    path.push(&*file_str);
    if !suffix_str.is_empty() {
        path.set_extension(&*suffix_str);
    }

    match CString::new(path.to_string_lossy().into_owned()) {
        Ok(c_str) => c_str.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}