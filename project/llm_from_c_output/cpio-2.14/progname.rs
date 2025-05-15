// Program name management.
// Copyright (C) 2001-2004, 2006, 2009-2023 Free Software Foundation, Inc.
// Written by Bruno Haible <bruno@clisp.org>, 2001.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::ffi::CStr;
use std::os::raw::c_char;
use std::panic;
use std::path::Path;

static mut PROGRAM_NAME: Option<&'static str> = None;

/// Set program_name, based on argv[0].
/// argv0 must be a string allocated with indefinite extent, and must not be
/// modified after this call.
pub fn set_program_name(argv0: &'static str) {
    // Sanity check. POSIX requires the invoking process to pass a non-NULL argv[0].
    if argv0.is_empty() {
        panic!("A NULL argv[0] was passed through an exec system call.");
    }

    let path = Path::new(argv0);
    let base_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or(argv0);

    let parent = path.parent();
    let parent_str = parent.and_then(|p| p.to_str()).unwrap_or("");

    // Check for "/.libs/" prefix
    let adjusted_name = if parent_str.ends_with("/.libs") {
        if base_name.starts_with("lt-") {
            &base_name[3..]
        } else {
            base_name
        }
    } else {
        argv0
    };

    unsafe {
        PROGRAM_NAME = Some(adjusted_name);
    }

    // Note: Rust doesn't have direct equivalents to program_invocation_name
    // and program_invocation_short_name, so those parts are omitted.
}

/// Get the current program name
pub fn get_program_name() -> Option<&'static str> {
    unsafe { PROGRAM_NAME }
}

#[cfg(feature = "relocatable")]
pub mod relocatable {
    use super::*;
    use std::path::PathBuf;

    static mut INSTALL_PREFIX: Option<&'static str> = None;
    static mut INSTALL_DIR: Option<&'static str> = None;

    /// Set program_name and installation directories for relocatability
    pub fn set_program_name_and_installdir(
        argv0: &'static str,
        orig_installprefix: &'static str,
        orig_installdir: &'static str,
    ) {
        set_program_name(argv0);
        unsafe {
            INSTALL_PREFIX = Some(orig_installprefix);
            INSTALL_DIR = Some(orig_installdir);
        }
    }

    /// Get the full pathname of the current executable
    pub fn get_full_program_name() -> Option<PathBuf> {
        unsafe {
            PROGRAM_NAME.map(|name| {
                let mut path = PathBuf::new();
                if let Some(prefix) = INSTALL_PREFIX {
                    path.push(prefix);
                }
                path.push(name);
                path
            })
        }
    }
}

// FFI-compatible interface
#[no_mangle]
pub extern "C" fn set_program_name_c(argv0: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(argv0) };
    let str_slice = c_str.to_str().expect("Invalid UTF-8 in program name");
    let boxed = Box::new(str_slice.to_string());
    let leaked = Box::leak(boxed);
    set_program_name(leaked);
}

#[no_mangle]
pub extern "C" fn get_program_name_c() -> *const c_char {
    match get_program_name() {
        Some(name) => name.as_ptr() as *const c_char,
        None => std::ptr::null(),
    }
}