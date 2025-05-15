/* Program name management.
   Copyright (C) 2016-2023 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as published by
   the Free Software Foundation; either version 2.1 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::env;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;

#[cfg(target_os = "linux")]
extern "C" {
    static program_invocation_short_name: *const c_char;
    static program_invocation_name: *const c_char;
}

#[cfg(target_os = "solaris")]
extern "C" {
    fn getexecname() -> *const c_char;
}

#[cfg(any(target_os = "windows", target_os = "cygwin"))]
extern "C" {
    static __argv: *const *const c_char;
}

#[cfg(any(target_os = "openbsd", target_os = "android", target_os = "qnx"))]
extern "C" {
    static __progname: *const c_char;
}

pub fn getprogname() -> &'static str {
    #[cfg(target_os = "linux")]
    unsafe {
        if !program_invocation_short_name.is_null() {
            return CStr::from_ptr(program_invocation_short_name)
                .to_str()
                .unwrap_or("?");
        } else if !program_invocation_name.is_null() {
            let path = CStr::from_ptr(program_invocation_name).to_str().unwrap_or("?");
            return Path::new(path).file_name().unwrap_or(Path::new("?")).to_str().unwrap_or("?");
        }
    }

    #[cfg(target_os = "solaris")]
    unsafe {
        let p = getexecname();
        if !p.is_null() {
            let path = CStr::from_ptr(p).to_str().unwrap_or("?");
            return Path::new(path).file_name().unwrap_or(Path::new("?")).to_str().unwrap_or("?");
        }
    }

    #[cfg(any(target_os = "windows", target_os = "cygwin"))]
    unsafe {
        if !__argv.is_null() && !(*__argv).is_null() {
            let p = *__argv;
            let path = CStr::from_ptr(p).to_str().unwrap_or("?");
            return Path::new(path).file_name().unwrap_or(Path::new("?")).to_str().unwrap_or("?");
        }
    }

    #[cfg(any(target_os = "openbsd", target_os = "android", target_os = "qnx"))]
    unsafe {
        if !__progname.is_null() {
            let p = CStr::from_ptr(__progname).to_str().unwrap_or("?");
            #[cfg(target_os = "android")]
            {
                return Path::new(p).file_name().unwrap_or(Path::new("?")).to_str().unwrap_or("?");
            }
            #[cfg(not(target_os = "android"))]
            {
                return if !p.is_empty() { p } else { "?" };
            }
        }
    }

    // Fallback for other platforms
    if let Ok(arg0) = env::args().next() {
        Path::new(&arg0).file_name().unwrap_or(Path::new("?")).to_str().unwrap_or("?")
    } else {
        "?"
    }
}

/*
 * Hey Emacs!
 * Local Variables:
 * coding: utf-8
 * End:
 */