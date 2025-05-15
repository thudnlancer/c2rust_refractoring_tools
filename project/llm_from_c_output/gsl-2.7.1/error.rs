// err/error.rs
//
// Copyright (C) 1996, 1997, 1998, 1999, 2000, 2007 Gerard Jungman, Brian Gough
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or (at
// your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301, USA.

use std::sync::atomic::{AtomicPtr, Ordering};
use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::io::{self, Write};

type GslErrorHandler = fn(reason: *const c_char, file: *const c_char, line: c_int, gsl_errno: c_int);

static GSL_ERROR_HANDLER: AtomicPtr<GslErrorHandler> = AtomicPtr::new(std::ptr::null_mut());

extern "C" {
    fn gsl_stream_printf(prefix: *const c_char, file: *const c_char, line: c_int, reason: *const c_char);
}

fn no_error_handler(_reason: *const c_char, _file: *const c_char, _line: c_int, _gsl_errno: c_int) {
    // do nothing
}

pub fn gsl_error(reason: *const c_char, file: *const c_char, line: c_int, gsl_errno: c_int) {
    let handler = GSL_ERROR_HANDLER.load(Ordering::SeqCst);
    
    if !handler.is_null() {
        unsafe {
            let handler_fn: GslErrorHandler = std::mem::transmute(handler);
            handler_fn(reason, file, line, gsl_errno);
        }
        return;
    }

    unsafe {
        gsl_stream_printf(b"ERROR\0".as_ptr() as *const c_char, file, line, reason);
    }

    let _ = io::stdout().flush();
    let _ = writeln!(io::stderr(), "Default GSL error handler invoked.");
    let _ = io::stderr().flush();

    std::process::abort();
}

pub fn gsl_set_error_handler(new_handler: Option<GslErrorHandler>) -> Option<GslErrorHandler> {
    let new_ptr = new_handler.map(|f| f as *mut GslErrorHandler).unwrap_or(std::ptr::null_mut());
    let prev_ptr = GSL_ERROR_HANDLER.swap(new_ptr, Ordering::SeqCst);
    
    if prev_ptr.is_null() {
        None
    } else {
        unsafe {
            Some(std::mem::transmute(prev_ptr))
        }
    }
}

pub fn gsl_set_error_handler_off() -> Option<GslErrorHandler> {
    let prev_ptr = GSL_ERROR_HANDLER.swap(no_error_handler as *mut GslErrorHandler, Ordering::SeqCst);
    
    if prev_ptr.is_null() {
        None
    } else {
        unsafe {
            Some(std::mem::transmute(prev_ptr))
        }
    }
}