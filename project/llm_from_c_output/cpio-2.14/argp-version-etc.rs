// Version hook for Argp.
// Copyright (C) 2009-2023 Free Software Foundation, Inc.
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
use std::ptr;
use std::sync::Once;

static mut PROGRAM_CANONICAL_NAME: *const c_char = ptr::null();
static mut PROGRAM_AUTHORS: *const *const c_char = ptr::null();
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
        version_etc_ar(
            stream,
            PROGRAM_CANONICAL_NAME,
            concat!(env!("PACKAGE_NAME"), "\0").as_ptr() as *const c_char,
            concat!(env!("VERSION"), "\0").as_ptr() as *const c_char,
            PROGRAM_AUTHORS,
        );
    }
}

#[no_mangle]
pub extern "C" fn argp_version_setup(name: *const c_char, authors: *const *const c_char) {
    unsafe {
        INIT.call_once(|| {
            libc::argp_program_version_hook = Some(version_etc_hook);
        });
        
        PROGRAM_CANONICAL_NAME = if name.is_null() {
            concat!(env!("PACKAGE"), "\0").as_ptr() as *const c_char
        } else {
            name
        };
        
        PROGRAM_AUTHORS = authors;
    }
}