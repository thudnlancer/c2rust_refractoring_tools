//! Program name management.
//! 
//! This module provides functionality for managing the program name,
//! similar to the C version but implemented safely in Rust.

use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;

/// Global variable containing the name the program is called with.
static mut PROGRAM_NAME: *const c_char = ptr::null();

/// Sets the program name based on argv[0].
/// 
/// # Safety
/// 
/// The input string must be a valid null-terminated C string with indefinite extent.
/// The caller must ensure the string remains valid for the lifetime of the program.
pub unsafe fn set_program_name(argv0: *const c_char) {
    // Sanity check. POSIX requires the invoking process to pass a non-NULL argv[0].
    if argv0.is_null() {
        eprintln!("A NULL argv[0] was passed through an exec system call.");
        std::process::abort();
    }

    let c_str = CStr::from_ptr(argv0);
    let path = Path::new(c_str.to_str().unwrap_or_default());
    
    // Handle libtool temporary executable names
    let base_name = path.file_name()
        .and_then(|s| s.to_str())
        .unwrap_or_default();

    let mut final_name = c_str;
    
    if let Some(parent) = path.parent() {
        if parent.ends_with(".libs") {
            if base_name.starts_with("lt-") {
                final_name = CStr::from_bytes_with_nul(b"lt-\0").unwrap();
                // Skip the "lt-" prefix
                final_name = CStr::from_ptr(argv0.add(3));
            } else {
                final_name = CStr::from_ptr(path.file_name().unwrap().as_ptr());
            }
        }
    }

    PROGRAM_NAME = final_name.as_ptr();
}

/// Gets the current program name.
/// 
/// Returns None if the program name hasn't been set yet.
pub fn get_program_name() -> Option<&'static str> {
    unsafe {
        if PROGRAM_NAME.is_null() {
            None
        } else {
            CStr::from_ptr(PROGRAM_NAME).to_str().ok()
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
        }
        assert_eq!(get_program_name(), Some("test_program"));
    }

    #[test]
    fn test_set_program_name_null() {
        unsafe {
            set_program_name(ptr::null());
        }
        // Should abort before we get here
    }
}