use std::ffi::CString;
use std::io::{self, Seek, SeekFrom};
use std::process;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref FILE_NAME: Mutex<Option<String>> = Mutex::new(None);
}

/// Set the file name to be reported in the event an error is detected
/// on stdin by close_stdin.
pub fn close_stdin_set_file_name(file: &str) {
    let mut file_name = FILE_NAME.lock().unwrap();
    *file_name = Some(file.to_string());
}

/// Close standard input, rewinding any unused input if stdin is seekable.
/// On error, issue a diagnostic and exit with status code 1.
/// Then call close_stdout.
pub fn close_stdin() {
    let mut fail = false;
    let stdin = io::stdin();

    // Check if stdin has buffered data
    if let Ok(metadata) = stdin.lock().metadata() {
        if metadata.is_file() {
            // Try to flush stdin if it's seekable
            if let Err(_) = stdin.lock().seek(SeekFrom::Current(0)) {
                fail = true;
            } else if let Err(_) = stdin.lock().flush() {
                fail = true;
            }
        }
    }

    // Close stdin
    if let Err(_) = stdin.lock().read(&mut []) {
        fail = true;
    }

    if fail {
        let file_name = FILE_NAME.lock().unwrap().as_ref();
        let close_error = "error closing file";
        match file_name {
            Some(name) => eprintln!("{}: {}: {}", name, close_error, io::Error::last_os_error()),
            None => eprintln!("{}: {}", close_error, io::Error::last_os_error()),
        }
    }

    close_stdout();

    if fail {
        process::exit(1);
    }
}

/// Close stdout, flushing any buffered output.
/// On error, issue a diagnostic and exit with status code 1.
pub fn close_stdout() {
    if let Err(e) = io::stdout().lock().flush() {
        eprintln!("error flushing stdout: {}", e);
        process::exit(1);
    }
}