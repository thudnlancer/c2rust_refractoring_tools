//! Take file names apart into directory and base names.
//!
//! This module provides functionality similar to the C dirname.h header,
//! implemented in safe Rust.

use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};
use std::ptr;

const DIRECTORY_SEPARATOR: char = '/';

/// Returns the base name of the given file path.
/// Similar to base_name() in C.
/// 
/// # Panics
/// Panics if the input is not valid UTF-8.
pub fn base_name(file: &Path) -> PathBuf {
    PathBuf::from(file.file_name().unwrap_or_else(|| file.as_os_str()))
}

/// Returns the directory name of the given file path.
/// Similar to dir_name() in C, but handles memory allocation safely.
/// 
/// # Panics
/// Panics if memory allocation fails or if the input is not valid UTF-8.
pub fn dir_name(file: &Path) -> PathBuf {
    file.parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Returns the directory name of the given file path.
/// Similar to mdir_name() in C, but returns Result for error handling.
/// 
/// # Errors
/// Returns an error if memory allocation fails.
pub fn mdir_name(file: &Path) -> Result<PathBuf, std::io::Error> {
    Ok(file.parent()
        .map(|p| p.to_path_buf())
        .unwrap_or_else(|| PathBuf::from(".")))
}

/// Returns the length of the directory portion of the path.
/// Similar to dir_len() in C.
pub fn dir_len(file: &Path) -> usize {
    file.parent()
        .map(|p| p.as_os_str().len())
        .unwrap_or(0)
}

/// Strips trailing slashes from the given path buffer.
/// Similar to strip_trailing_slashes() in C.
/// 
/// Returns true if any slashes were stripped.
pub fn strip_trailing_slashes(file: &mut PathBuf) -> bool {
    let mut changed = false;
    while file.as_os_str().to_string_lossy().ends_with(DIRECTORY_SEPARATOR) {
        file.pop();
        changed = true;
    }
    changed
}

// FFI-compatible versions for C interop (marked as unsafe since they deal with raw pointers)

#[no_mangle]
pub unsafe extern "C" fn base_name_c(file: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(file) };
    let path = Path::new(c_str.to_str().expect("Invalid UTF-8"));
    let result = base_name(path);
    let c_string = std::ffi::CString::new(result.to_string_lossy().into_owned())
        .expect("Failed to create CString");
    c_string.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn dir_name_c(file: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(file) };
    let path = Path::new(c_str.to_str().expect("Invalid UTF-8"));
    let result = dir_name(path);
    let c_string = std::ffi::CString::new(result.to_string_lossy().into_owned())
        .expect("Failed to create CString");
    c_string.into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn mdir_name_c(file: *const c_char) -> *mut c_char {
    let c_str = unsafe { CStr::from_ptr(file) };
    let path = Path::new(c_str.to_str().expect("Invalid UTF-8"));
    match mdir_name(path) {
        Ok(result) => {
            let c_string = std::ffi::CString::new(result.to_string_lossy().into_owned())
                .expect("Failed to create CString");
            c_string.into_raw()
        }
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn dir_len_c(file: *const c_char) -> usize {
    let c_str = unsafe { CStr::from_ptr(file) };
    let path = Path::new(c_str.to_str().expect("Invalid UTF-8"));
    dir_len(path)
}

#[no_mangle]
pub unsafe extern "C" fn strip_trailing_slashes_c(file: *mut c_char) -> bool {
    let c_str = unsafe { CStr::from_ptr(file) };
    let mut path = PathBuf::from(c_str.to_str().expect("Invalid UTF-8"));
    let result = strip_trailing_slashes(&mut path);
    let c_string = std::ffi::CString::new(path.to_string_lossy().into_owned())
        .expect("Failed to create CString");
    unsafe {
        ptr::copy_nonoverlapping(c_string.as_ptr(), file, c_string.as_bytes().len() + 1);
    }
    result
}