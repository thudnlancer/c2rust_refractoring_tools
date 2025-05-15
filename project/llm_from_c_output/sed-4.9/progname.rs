//! Program name management.
//! 
//! This module provides functionality for managing the program name,
//! similar to the C version but implemented in safe Rust.

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
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    
    let mut final_name = base_name;
    
    // Check for "/.libs/" prefix and "lt-" prefix
    if let Some(parent) = path.parent() {
        if let Some(parent_str) = parent.to_str() {
            if parent_str.ends_with("/.libs") {
                if final_name.starts_with("lt-") {
                    final_name = &final_name[3..];
                }
            }
        }
    }

    // Convert back to C string
    let new_name = std::ffi::CString::new(final_name).unwrap();
    PROGRAM_NAME = new_name.into_raw();
}

/// Gets the current program name.
/// 
/// Returns None if the program name hasn't been set yet.
pub fn get_program_name() -> Option<&'static str> {
    unsafe {
        if PROGRAM_NAME.is_null() {
            None
        } else {
            Some(CStr::from_ptr(PROGRAM_NAME).to_str().ok())
        }
    }
}

#[cfg(feature = "relocatable")]
mod relocatable {
    use super::*;
    use std::ffi::CString;

    static mut INSTALL_PREFIX: *const c_char = ptr::null();
    static mut INSTALL_DIR: *const c_char = ptr::null();

    /// Sets the program name and installation directories for relocatability.
    pub unsafe fn set_program_name_and_installdir(
        argv0: *const c_char,
        orig_installprefix: *const c_char,
        orig_installdir: *const c_char,
    ) {
        set_program_name(argv0);
        
        if !orig_installprefix.is_null() {
            INSTALL_PREFIX = CStr::from_ptr(orig_installprefix).to_owned().into_raw();
        }
        
        if !orig_installdir.is_null() {
            INSTALL_DIR = CStr::from_ptr(orig_installdir).to_owned().into_raw();
        }
    }

    /// Gets the full pathname of the current executable.
    pub fn get_full_program_name() -> Option<CString> {
        unsafe {
            if PROGRAM_NAME.is_null() {
                None
            } else {
                Some(CStr::from_ptr(PROGRAM_NAME).to_owned())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_set_and_get_program_name() {
        let name = CString::new("test_program").unwrap();
        unsafe {
            set_program_name(name.as_ptr());
        }
        assert_eq!(get_program_name(), Some("test_program"));
    }

    #[test]
    fn test_null_program_name() {
        unsafe {
            set_program_name(ptr::null());
        }
        // Should abort, so this test is expected to panic
    }
}