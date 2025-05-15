/* Close standard output and standard error, exiting with a diagnostic on error.

   Copyright (C) 1998-2002, 2004, 2006, 2008-2023 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::ffi::CString;
use std::io;
use std::process;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::Once;

static FILE_NAME: AtomicPtr<CString> = AtomicPtr::new(std::ptr::null_mut());
static IGNORE_EPIPE: AtomicBool = AtomicBool::new(false);
static INIT: Once = Once::new();

/// Set the file name to be reported in the event an error is detected by close_stdout.
pub fn close_stdout_set_file_name(file: &str) {
    INIT.call_once(|| {
        // Initialize the atomic pointer
    });
    let cstring = CString::new(file).unwrap();
    let ptr = Box::into_raw(Box::new(cstring));
    let old_ptr = FILE_NAME.swap(ptr, Ordering::SeqCst);
    if !old_ptr.is_null() {
        unsafe { Box::from_raw(old_ptr) };
    }
}

/// Specify the reaction to an EPIPE error during the closing of stdout.
pub fn close_stdout_set_ignore_epipe(ignore: bool) {
    IGNORE_EPIPE.store(ignore, Ordering::SeqCst);
}

/// Close standard output and standard error, exiting with a diagnostic on error.
pub fn close_stdout() {
    let stdout = std::io::stdout();
    let result = stdout.lock().flush();

    if let Err(e) = result {
        if !(IGNORE_EPIPE.load(Ordering::SeqCst) && e.kind() == io::ErrorKind::BrokenPipe) {
            let write_error = "write error";
            let file_name_ptr = FILE_NAME.load(Ordering::SeqCst);
            if !file_name_ptr.is_null() {
                let file_name = unsafe { &*file_name_ptr }.to_string_lossy();
                eprintln!("{}: {}: {}", file_name, write_error, e);
            } else {
                eprintln!("{}: {}", write_error, e);
            }
            process::exit(1);
        }
    }

    // Close stderr only if not sanitizing (Rust doesn't have a direct equivalent
    // to the C sanitizer check, so we always close stderr)
    let stderr = std::io::stderr();
    if let Err(_) = stderr.lock().flush() {
        process::exit(1);
    }
}