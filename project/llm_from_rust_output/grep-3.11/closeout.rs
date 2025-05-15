use std::ffi::CStr;
use std::io::{self, Write};
use std::process;

static mut FILE_NAME: Option<&'static CStr> = None;
static mut IGNORE_EPIPE: bool = false;

pub fn close_stdout_set_file_name(file: &'static CStr) {
    unsafe {
        FILE_NAME = Some(file);
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
            let msg = "write error";
            let file_name = unsafe { FILE_NAME };
            
            if let Some(name) = file_name {
                let name_str = name.to_string_lossy();
                eprintln!("{}: {}: {}", name_str, msg, e);
            } else {
                eprintln!("{}: {}", msg, e);
            }
            process::exit(1);
        }
    }

    if let Err(_) = stderr_result {
        process::exit(1);
    }
}