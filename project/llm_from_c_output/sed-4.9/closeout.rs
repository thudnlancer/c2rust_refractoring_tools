/* Close standard output and standard error.

   Copyright (C) 1998, 2000, 2003-2004, 2006, 2008-2022 Free Software
   Foundation, Inc.

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
use std::io::{self, Write};
use std::process;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::Once;

static FILE_NAME: AtomicPtr<CString> = AtomicPtr::new(std::ptr::null_mut());
static IGNORE_EPIPE: AtomicBool = AtomicBool::new(false);
static INIT: Once = Once::new();

/// Set the file name to be reported in the event an error is detected by close_stdout.
pub fn close_stdout_set_file_name(file: &str) {
    INIT.call_once(|| {
        let cstring = CString::new(file).unwrap();
        let ptr = Box::into_raw(Box::new(cstring));
        FILE_NAME.store(ptr, Ordering::Release);
    });
}

/// Specify the reaction to an EPIPE error during the closing of stdout.
pub fn close_stdout_set_ignore_epipe(ignore: bool) {
    IGNORE_EPIPE.store(ignore, Ordering::Release);
}

/// Close standard output and standard error, exiting with a diagnostic on error.
pub fn close_stdout() {
    let stdout_result = std::io::stdout().flush();
    let stderr_result = std::io::stderr().flush();

    if let Err(e) = stdout_result {
        if !(IGNORE_EPIPE.load(Ordering::Acquire) && e.kind() == io::ErrorKind::BrokenPipe) {
            let write_error = "write error";
            let file_name_ptr = FILE_NAME.load(Ordering::Acquire);
            
            if !file_name_ptr.is_null() {
                let file_name = unsafe { &*file_name_ptr };
                eprintln!("{}: {}: {}", file_name.to_str().unwrap(), write_error, e);
            } else {
                eprintln!("{}: {}", write_error, e);
            }

            process::exit(1);
        }
    }

    // Close stderr only if not sanitizing, as sanitizers may report to stderr after this function returns.
    if !cfg!(sanitize = "address") {
        if let Err(e) = stderr_result {
            eprintln!("Failed to flush stderr: {}", e);
            process::exit(1);
        }
    }
}