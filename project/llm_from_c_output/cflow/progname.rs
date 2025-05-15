//! Program name management.
//! 
//! This module provides functionality for managing the program name,
//! similar to the original C implementation.
//! 
//! Programs using this module should call `set_program_name()` early in main().

use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;

/// Global variable containing the name the program is called with.
pub static mut PROGRAM_NAME: Option<&'static str> = None;

/// Sets the program name based on the provided argument.
/// 
/// # Arguments
/// * `argv0` - The first argument from the command line (program name/path)
pub fn set_program_name(argv0: &str) {
    let path = Path::new(argv0);
    
    // Handle libtool temporary executable names
    let base_name = if let Some(file_name) = path.file_name() {
        file_name.to_str().unwrap_or(argv0)
    } else {
        argv0
    };

    // Check for "/.libs/" prefix and "lt-" prefix
    let processed_name = if let Some(parent) = path.parent() {
        if let Some(parent_str) = parent.to_str() {
            if parent_str.ends_with("/.libs") {
                if base_name.starts_with("lt-") {
                    &base_name[3..]
                } else {
                    base_name
                }
            } else {
                argv0
            }
        } else {
            argv0
        }
    } else {
        argv0
    };

    // Store the program name in the global variable
    unsafe {
        PROGRAM_NAME = Some(Box::leak(processed_name.to_string().into_boxed_str()));
    }

    // Note: Rust doesn't have direct equivalents to program_invocation_name
    // and program_invocation_short_name, so those parts are omitted
}

/// Gets the current program name.
/// 
/// # Returns
/// Option containing the program name if set, None otherwise
pub fn get_program_name() -> Option<&'static str> {
    unsafe { PROGRAM_NAME }
}

// Note: The relocatable functionality (ENABLE_RELOCATABLE) is not implemented
// in this translation as it would require additional context about the build
// system and installation paths that isn't available in this scope.