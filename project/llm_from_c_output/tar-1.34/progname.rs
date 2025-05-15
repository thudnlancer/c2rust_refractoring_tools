//! Program name management.
//! 
//! This module provides functionality for managing the program name,
//! similar to the original C implementation.

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;
use std::path::Path;

/// Global variable containing the name the program is called with.
pub static mut PROGRAM_NAME: *const c_char = ptr::null();

/// Sets the program name based on argv[0].
/// 
/// # Safety
/// 
/// The argv0 must be a valid null-terminated C string with indefinite extent,
/// and must not be modified after this call.
pub unsafe fn set_program_name(argv0: *const c_char) {
    // Sanity check. POSIX requires the invoking process to pass a non-NULL argv[0].
    if argv0.is_null() {
        eprintln!("A NULL argv[0] was passed through an exec system call.");
        std::process::abort();
    }

    let c_str = CStr::from_ptr(argv0);
    let path = Path::new(c_str.to_str().unwrap_or(""));

    // Handle libtool temporary executable names
    let base_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_else(|| c_str.to_str().unwrap_or(""));

    let mut name = base_name;
    
    // Check for ".libs/" prefix and "lt-" prefix
    if let Some(parent) = path.parent() {
        if let Some(parent_str) = parent.to_str() {
            if parent_str.ends_with("/.libs") {
                if name.starts_with("lt-") {
                    name = &name[3..];
                }
            }
        }
    }

    // Convert back to C string
    let new_name = std::ffi::CString::new(name).unwrap();
    PROGRAM_NAME = new_name.into_raw();
}

/// Gets the current program name.
/// 
/// Returns None if the program name hasn't been set yet.
pub fn get_program_name() -> Option<String> {
    unsafe {
        if PROGRAM_NAME.is_null() {
            None
        } else {
            Some(CStr::from_ptr(PROGRAM_NAME).to_string_lossy().into_owned())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_set_program_name() {
        let name = CString::new("test_program").unwrap();
        unsafe {
            set_program_name(name.as_ptr());
            assert_eq!(get_program_name(), Some("test_program".to_string()));
        }
    }

    #[test]
    fn test_libtool_names() {
        let name = CString::new("/path/to/.libs/lt-test_program").unwrap();
        unsafe {
            set_program_name(name.as_ptr());
            assert_eq!(get_program_name(), Some("test_program".to_string()));
        }
    }

    #[test]
    #[should_panic]
    fn test_null_name() {
        unsafe {
            set_program_name(ptr::null());
        }
    }
}