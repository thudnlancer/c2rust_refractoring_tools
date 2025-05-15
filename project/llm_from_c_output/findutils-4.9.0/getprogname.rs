// Program name management.
// Copyright (C) 2016-2022 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation; either version 2.1 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::env;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use std::sync::Once;

#[cfg(target_os = "linux")]
extern "C" {
    #[link_name = "program_invocation_short_name"]
    static PROGRAM_INVOCATION_SHORT_NAME: *const c_char;
    #[link_name = "program_invocation_name"]
    static PROGRAM_INVOCATION_NAME: *const c_char;
}

#[cfg(target_os = "solaris")]
extern "C" {
    fn getexecname() -> *const c_char;
}

#[cfg(target_os = "windows")]
extern "C" {
    static __argv: *const *const c_char;
}

#[cfg(any(target_os = "openbsd", target_os = "android", target_os = "qnx"))]
extern "C" {
    static mut __progname: *const c_char;
}

fn last_component(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("?")
}

pub fn getprogname() -> &'static str {
    #[cfg(target_os = "linux")]
    unsafe {
        if !PROGRAM_INVOCATION_SHORT_NAME.is_null() {
            return CStr::from_ptr(PROGRAM_INVOCATION_SHORT_NAME)
                .to_str()
                .unwrap_or("?");
        } else if !PROGRAM_INVOCATION_NAME.is_null() {
            return last_component(
                CStr::from_ptr(PROGRAM_INVOCATION_NAME)
                    .to_str()
                    .unwrap_or("?"),
            );
        }
    }

    #[cfg(target_os = "solaris")]
    unsafe {
        let p = getexecname();
        if !p.is_null() {
            return last_component(CStr::from_ptr(p).to_str().unwrap_or("?"));
        }
    }

    #[cfg(target_os = "windows")]
    unsafe {
        if !__argv.is_null() && !(*__argv).is_null() {
            return last_component(CStr::from_ptr(*__argv).to_str().unwrap_or("?"));
        }
    }

    #[cfg(any(target_os = "openbsd", target_os = "android", target_os = "qnx"))]
    unsafe {
        if !__progname.is_null() {
            #[cfg(target_os = "android")]
            {
                return last_component(CStr::from_ptr(__progname).to_str().unwrap_or("?"));
            }
            #[cfg(not(target_os = "android"))]
            {
                return CStr::from_ptr(__progname).to_str().unwrap_or("?");
            }
        }
    }

    // Fallback for other platforms
    env::args()
        .next()
        .as_ref()
        .map(|s| last_component(s))
        .unwrap_or("?")
}

/*
 * Hey Emacs!
 * Local Variables:
 * coding: utf-8
 * End:
 */