// Version hook for Argp.
// Copyright (C) 2009-2021 Free Software Foundation, Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
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
use std::ptr;
use std::sync::Once;

static mut PROGRAM_CANONICAL_NAME: Option<&'static CStr> = None;
static mut PROGRAM_AUTHORS: Option<&'static [&'static CStr]> = None;
static INIT: Once = Once::new();

extern "C" {
    fn version_etc_ar(
        stream: *mut libc::FILE,
        program_name: *const c_char,
        package: *const c_char,
        version: *const c_char,
        authors: *const *const c_char,
    );
}

extern "C" fn version_etc_hook(stream: *mut libc::FILE, _state: *mut libc::c_void) {
    unsafe {
        let name = PROGRAM_CANONICAL_NAME
            .map(|n| n.as_ptr())
            .unwrap_or(ptr::null());
        let package = CStr::from_bytes_with_nul_unchecked(b"PACKAGE\0").as_ptr();
        let version = CStr::from_bytes_with_nul_unchecked(b"VERSION\0").as_ptr();
        let authors = PROGRAM_AUTHORS
            .map(|a| a.as_ptr() as *const *const c_char)
            .unwrap_or(ptr::null());
        
        version_etc_ar(stream, name, package, version, authors);
    }
}

#[no_mangle]
pub extern "C" fn argp_version_setup(name: *const c_char, authors: *const *const c_char) {
    unsafe {
        INIT.call_once(|| {
            let name = if name.is_null() {
                None
            } else {
                Some(CStr::from_ptr(name))
            };
            
            let mut authors_vec = Vec::new();
            if !authors.is_null() {
                let mut i = 0;
                while !(*authors.offset(i)).is_null() {
                    authors_vec.push(CStr::from_ptr(*authors.offset(i)));
                    i += 1;
                }
            }
            
            let authors_slice = if authors_vec.is_empty() {
                None
            } else {
                Some(authors_vec.leak().as_slice())
            };
            
            PROGRAM_CANONICAL_NAME = name;
            PROGRAM_AUTHORS = authors_slice;
            
            libc::argp_program_version_hook = Some(version_etc_hook);
        });
    }
}