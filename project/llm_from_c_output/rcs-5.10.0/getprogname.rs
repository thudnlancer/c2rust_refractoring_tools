//! Program name management.
//! 
//! This module provides functionality to get the name of the currently running program.
//! 
//! The code is based on the GNU C implementation but rewritten in safe Rust.

use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use std::ffi::OsStr;
use std::env;

#[cfg(target_os = "windows")]
use std::os::windows::ffi::OsStrExt;

#[cfg(any(target_os = "linux", target_os = "android"))]
extern {
    #[link_name = "program_invocation_short_name"]
    static PROGRAM_INVOCATION_SHORT_NAME: *const c_char;
}

#[cfg(any(target_os = "linux", target_os = "android"))]
extern {
    #[link_name = "program_invocation_name"]
    static PROGRAM_INVOCATION_NAME: *const c_char;
}

#[cfg(target_os = "solaris")]
extern {
    fn getexecname() -> *const c_char;
}

#[cfg(target_os = "windows")]
extern {
    static __argv: *const *const c_char;
}

#[cfg(any(target_os = "openbsd", target_os = "android", target_os = "qnx"))]
extern {
    static __progname: *const c_char;
}

/// Returns the base name of the currently running program.
/// 
/// On Windows, this will usually end in ".exe" or ".EXE".
pub fn getprogname() -> &'static str {
    #[cfg(any(target_os = "linux", target_os = "android"))] {
        if !PROGRAM_INVOCATION_SHORT_NAME.is_null() {
            unsafe {
                return CStr::from_ptr(PROGRAM_INVOCATION_SHORT_NAME)
                    .to_str()
                    .unwrap_or("?");
            }
        }
    }

    #[cfg(any(target_os = "linux", target_os = "android"))] {
        if !PROGRAM_INVOCATION_NAME.is_null() {
            unsafe {
                let path = CStr::from_ptr(PROGRAM_INVOCATION_NAME).to_str().unwrap_or("?");
                return Path::new(path)
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap_or("?");
            }
        }
    }

    #[cfg(target_os = "solaris")] {
        unsafe {
            let p = getexecname();
            if !p.is_null() {
                let path = CStr::from_ptr(p).to_str().unwrap_or("?");
                return Path::new(path)
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap_or("?");
            }
        }
    }

    #[cfg(target_os = "windows")] {
        unsafe {
            if !__argv.is_null() && !(*__argv).is_null() {
                let arg0 = CStr::from_ptr(*__argv).to_str().unwrap_or("?");
                return Path::new(arg0)
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap_or("?");
            }
        }
    }

    #[cfg(any(target_os = "openbsd", target_os = "android", target_os = "qnx"))] {
        unsafe {
            if !__progname.is_null() {
                let progname = CStr::from_ptr(__progname).to_str().unwrap_or("?");
                #[cfg(target_os = "android")] {
                    return Path::new(progname)
                        .file_name()
                        .and_then(OsStr::to_str)
                        .unwrap_or("?");
                }
                #[cfg(not(target_os = "android"))] {
                    return if !progname.is_empty() { progname } else { "?" };
                }
            }
        }
    }

    // Fallback for other platforms
    if let Some(arg0) = env::args().next() {
        Path::new(&arg0)
            .file_name()
            .and_then(OsStr::to_str)
            .unwrap_or("?")
    } else {
        "?"
    }
}