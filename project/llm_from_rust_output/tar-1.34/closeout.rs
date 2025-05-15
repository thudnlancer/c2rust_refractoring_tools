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
        let ignore_epipe = unsafe { IGNORE_EPIPE };
        if !(ignore_epipe && e.kind() == io::ErrorKind::BrokenPipe) {
            let write_error = "write error";
            let file_name = unsafe { FILE_NAME.as_ref() };
            
            if let Some(name) = file_name {
                let _ = writeln!(
                    io::stderr(),
                    "{}: {}: {}",
                    name.to_string_lossy(),
                    write_error,
                    e
                );
            } else {
                let _ = writeln!(io::stderr(), "{}: {}", write_error, e);
            }
            process::exit(1);
        }
    }

    if let Err(e) = stderr_result {
        let _ = writeln!(io::stderr(), "write error: {}", e);
        process::exit(1);
    }
}