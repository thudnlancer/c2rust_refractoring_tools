//! Program name management.
//!
//! This module provides functionality for managing the program name,
//! similar to the C version but implemented in safe Rust.

use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;

/// Global variable containing the name the program is called with.
pub static mut PROGRAM_NAME: Option<&'static str> = None;

/// Sets the program name based on argv[0].
///
/// # Arguments
/// * `argv0` - The program name string from argv[0].
///
/// # Safety
/// The input string must be valid UTF-8 and have static lifetime.
pub unsafe fn set_program_name(argv0: &'static str) {
    let path = Path::new(argv0);
    let base_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(argv0);

    // Handle libtool temporary executable names
    let mut name = base_name;
    if let Some(parent) = path.parent() {
        if parent.ends_with(".libs") {
            if name.starts_with("lt-") {
                name = &name[3..];
                // On glibc systems, we would update program_invocation_short_name here
            }
        }
    }

    PROGRAM_NAME = Some(name);
}

#[cfg(feature = "relocatable")]
pub mod relocatable {
    use super::*;
    use std::path::PathBuf;

    static mut INSTALL_PREFIX: Option<&'static str> = None;
    static mut INSTALL_DIR: Option<&'static str> = None;

    /// Sets the program name and installation directory for relocatable binaries.
    pub unsafe fn set_program_name_and_installdir(
        argv0: &'static str,
        orig_installprefix: &'static str,
        orig_installdir: &'static str,
    ) {
        INSTALL_PREFIX = Some(orig_installprefix);
        INSTALL_DIR = Some(orig_installdir);
        set_program_name(argv0);
    }

    /// Gets the full path of the current executable.
    pub fn get_full_program_name() -> Option<PathBuf> {
        std::env::current_exe().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_program_name() {
        unsafe {
            set_program_name("/usr/bin/myprogram");
            assert_eq!(PROGRAM_NAME, Some("myprogram"));

            set_program_name("/path/to/.libs/lt-myprogram");
            assert_eq!(PROGRAM_NAME, Some("myprogram"));

            set_program_name("simple");
            assert_eq!(PROGRAM_NAME, Some("simple"));
        }
    }

    #[cfg(feature = "relocatable")]
    #[test]
    fn test_relocatable() {
        unsafe {
            relocatable::set_program_name_and_installdir(
                "/usr/bin/myprogram",
                "/usr",
                "/usr/bin",
            );
            assert_eq!(PROGRAM_NAME, Some("myprogram"));
            assert!(relocatable::get_full_program_name().is_some());
        }
    }
}