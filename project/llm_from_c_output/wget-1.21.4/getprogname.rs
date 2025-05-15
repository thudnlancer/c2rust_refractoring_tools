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
use std::ffi::CString;
use std::path::Path;
use std::process;
use std::ptr;

#[cfg(target_os = "linux")]
extern "C" {
    static program_invocation_short_name: *const libc::c_char;
    static program_invocation_name: *const libc::c_char;
}

#[cfg(target_os = "solaris")]
extern "C" {
    fn getexecname() -> *const libc::c_char;
}

#[cfg(any(target_os = "windows", target_os = "mingw"))]
extern "C" {
    static __argv: *const *const libc::c_char;
}

#[cfg(any(target_os = "openbsd", target_os = "android"))]
extern "C" {
    static mut __progname: *const libc::c_char;
}

fn last_component(path: &str) -> &str {
    Path::new(path)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("?")
}

pub fn getprogname() -> &'static str {
    #[cfg(target_os = "linux")]
    unsafe {
        if !program_invocation_short_name.is_null() {
            let name = std::ffi::CStr::from_ptr(program_invocation_short_name)
                .to_str()
                .unwrap_or("?");
            return name;
        } else if !program_invocation_name.is_null() {
            let name = std::ffi::CStr::from_ptr(program_invocation_name)
                .to_str()
                .unwrap_or("?");
            return last_component(name);
        }
    }

    #[cfg(target_os = "solaris")]
    unsafe {
        let p = getexecname();
        if !p.is_null() {
            let name = std::ffi::CStr::from_ptr(p).to_str().unwrap_or("?");
            return last_component(name);
        }
        return "?";
    }

    #[cfg(any(target_os = "windows", target_os = "mingw"))]
    unsafe {
        if !__argv.is_null() && !(*__argv).is_null() {
            let p = *__argv;
            let name = std::ffi::CStr::from_ptr(p).to_str().unwrap_or("?");
            return last_component(name);
        }
        return "?";
    }

    #[cfg(any(target_os = "openbsd", target_os = "android"))]
    unsafe {
        let p = __progname;
        if !p.is_null() {
            let name = std::ffi::CStr::from_ptr(p).to_str().unwrap_or("?");
            #[cfg(target_os = "android")]
            return last_component(name);
            #[cfg(not(target_os = "android"))]
            return if !name.is_empty() { name } else { "?" };
        }
        return "?";
    }

    // Fallback for other platforms
    if let Some(name) = env::args().next() {
        last_component(&name)
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