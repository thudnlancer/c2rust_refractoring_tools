/* API for GNU Make dynamic objects.
Copyright (C) 2013-2023 Free Software Foundation, Inc.
This file is part of GNU Make.

GNU Make is free software; you can redistribute it and/or modify it under the
terms of the GNU General Public License as published by the Free Software
Foundation; either version 3 of the License, or (at your option) any later
version.

GNU Make is distributed in the hope that it will be useful, but WITHOUT ANY
WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
A PARTICULAR PURPOSE.  See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with
this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::ffi::{CString, CStr};
use std::os::raw::c_char;
use std::ptr;

#[repr(C)]
pub struct GmkFloc {
    filenm: *const c_char,
    lineno: u32,
}

#[repr(C)]
struct Floc {
    filenm: *const c_char,
    lineno: u32,
    offset: u32,
}

type GmkFuncPtr = extern "C" fn();

/// Allocate a buffer in our context, so we can free it.
pub fn gmk_alloc(len: usize) -> *mut c_char {
    let buf = vec![0u8; len];
    let boxed_slice = buf.into_boxed_slice();
    Box::into_raw(boxed_slice) as *mut c_char
}

/// Free a buffer returned by gmk_expand().
pub fn gmk_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            let _ = Box::from_raw(s);
        }
    }
}

/// Evaluate a buffer as make syntax.
pub fn gmk_eval(buffer: *const c_char, gfloc: *const GmkFloc) {
    // Preserve existing variable buffer context.
    let mut pbuf: *mut c_char = ptr::null_mut();
    let mut plen: usize = 0;
    
    let flp = if !gfloc.is_null() {
        let gfloc_ref = unsafe { &*gfloc };
        let fl = Floc {
            filenm: gfloc_ref.filenm,
            lineno: gfloc_ref.lineno,
            offset: 0,
        };
        &fl as *const Floc
    } else {
        ptr::null()
    };

    unsafe {
        install_variable_buffer(&mut pbuf, &mut plen);
        
        let c_str = CStr::from_ptr(buffer);
        let s = CString::new(c_str.to_bytes()).unwrap();
        eval_buffer(s.as_ptr(), flp);
        
        restore_variable_buffer(pbuf, plen);
    }
}

/// Expand a string and return an allocated buffer.
/// Caller must call gmk_free() with this buffer.
pub fn gmk_expand(ref_: *const c_char) -> *mut c_char {
    unsafe { allocated_variable_expand(ref_) }
}

/// Register a function to be called from makefiles.
pub fn gmk_add_function(
    name: *const c_char,
    func: GmkFuncPtr,
    min: u32,
    max: u32,
    flags: u32,
) {
    unsafe {
        define_new_function(reading_file, name, min, max, flags, func);
    }
}

// Placeholder for external C functions that need to be linked
extern "C" {
    fn xmalloc(len: usize) -> *mut c_char;
    fn install_variable_buffer(pbuf: *mut *mut c_char, plen: *mut usize);
    fn eval_buffer(buffer: *const c_char, floc: *const Floc);
    fn restore_variable_buffer(pbuf: *mut c_char, plen: usize);
    fn allocated_variable_expand(ref_: *const c_char) -> *mut c_char;
    fn define_new_function(
        reading_file: *const (),
        name: *const c_char,
        min: u32,
        max: u32,
        flags: u32,
        func: GmkFuncPtr,
    );
    static reading_file: *const ();
}