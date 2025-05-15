/*
 * Version hook for Argp.
 * Copyright (C) 2009 Free Software Foundation, Inc.
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

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
            PACKAGE_NAME.as_ptr(),
            VERSION.as_ptr(),
            PROGRAM_AUTHORS,
        );
    }
}

#[no_mangle]
pub extern "C" fn argp_version_setup(name: *const c_char, authors: *const *const c_char) {
    INIT.call_once(|| {
        unsafe {
            argp_program_version_hook = Some(version_etc_hook);
            PROGRAM_CANONICAL_NAME = name;
            PROGRAM_AUTHORS = authors;
        }
    });
}

// These would typically come from build.rs or config.rs
const PACKAGE_NAME: &'static [u8] = b"PACKAGE\0";
const VERSION: &'static [u8] = b"VERSION\0";

// This would be provided by the argp-rs crate
static mut argp_program_version_hook: Option<extern "C" fn(*mut libc::FILE, *mut libc::c_void)> = None;