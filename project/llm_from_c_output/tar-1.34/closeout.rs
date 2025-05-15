/*
   Close standard output and standard error, exiting with a diagnostic on error.

   Copyright (C) 1998-2002, 2004, 2006, 2008-2021 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::ffi::CString;
use std::io;
use std::process;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::Once;

static FILE_NAME: AtomicPtr<u8> = AtomicPtr::new(std::ptr::null_mut());
static IGNORE_EPIPE: AtomicBool = AtomicBool::new(false);
static INIT: Once = Once::new();

/// Set the file name to be reported in the event an error is detected by close_stdout.
pub fn close_stdout_set_file_name(file: Option<&str>) {
    INIT.call_once(|| {
        // Initialize if needed
    });
    
    if let Some(f) = file {
        let c_str = CString::new(f).expect("CString::new failed");
        let ptr = c_str.into_raw();
        let old_ptr = FILE_NAME.swap(ptr, Ordering::SeqCst);
        if !old_ptr.is_null() {
            unsafe { CString::from_raw(old_ptr) };
        }
    } else {
        let old_ptr = FILE_NAME.swap(std::ptr::null_mut(), Ordering::SeqCst);
        if !old_ptr.is_null() {
            unsafe { CString::from_raw(old_ptr) };
        }
    }
}

/// Specify the reaction to an EPIPE error during the closing of stdout.
pub fn close_stdout_set_ignore_epipe(ignore: bool) {
    IGNORE_EPIPE.store(ignore, Ordering::SeqCst);
}

/// Close standard output and standard error, exiting with a diagnostic on error.
pub fn close_stdout() {
    fn close_stream(stream: &mut dyn io::Write) -> io::Result<()> {
        stream.flush()?;
        Ok(())
    }

    let stdout_result = close_stream(&mut io::stdout());
    let stderr_result = if !cfg!(sanitize = "address") {
        Some(close_stream(&mut io::stderr()))
    } else {
        None
    };

    if let Err(e) = stdout_result {
        if !(IGNORE_EPIPE.load(Ordering::SeqCst) && e.kind() == io::ErrorKind::BrokenPipe) {
            let write_error = "write error";
            let file_name_ptr = FILE_NAME.load(Ordering::SeqCst);
            
            if !file_name_ptr.is_null() {
                let file_name = unsafe { CString::from_raw(file_name_ptr) };
                eprintln!("{}: {}: {}", 
                    file_name.to_string_lossy(), 
                    write_error, 
                    e);
                let _ = FILE_NAME.swap(std::ptr::null_mut(), Ordering::SeqCst);
            } else {
                eprintln!("{}: {}", write_error, e);
            }
            
            process::exit(1);
        }
    }

    if let Some(Err(_)) = stderr_result {
        process::exit(1);
    }
}