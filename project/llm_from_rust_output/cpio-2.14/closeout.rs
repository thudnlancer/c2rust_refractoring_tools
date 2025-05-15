use std::ffi::{CStr, CString};
use std::io::{self, Write};
use std::process;

static mut FILE_NAME: Option<CString> = None;
static mut IGNORE_EPIPE: bool = false;

pub fn close_stdout_set_file_name(file: &CStr) {
    unsafe {
        FILE_NAME = Some(file.to_owned());
    }
}

pub fn close_stdout_set_ignore_epipe(ignore: bool) {
    unsafe {
        IGNORE_EPIPE = ignore;
    }
}

pub fn close_stdout() {
    let stdout_result = io::stdout().flush();
    let stderr_result = io::stderr().flush();

    if let Err(e) = stdout_result {
        let ignore = unsafe { IGNORE_EPIPE };
        if !ignore || e.kind() != io::ErrorKind::BrokenPipe {
            let write_error = "write error";
            
            unsafe {
                if let Some(file_name) = &FILE_NAME {
                    eprintln!("{}: {}", file_name.to_string_lossy(), write_error);
                } else {
                    eprintln!("{}", write_error);
                }
            }
            
            process::exit(1);
        }
    }

    if let Err(_) = stderr_result {
        process::exit(1);
    }
}