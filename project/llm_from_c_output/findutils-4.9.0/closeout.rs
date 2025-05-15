// Close standard output and standard error, exiting with a diagnostic on error.

use std::ffi::CString;
use std::io::{self, Write};
use std::process;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::Once;

static FILE_NAME: AtomicPtr<CString> = AtomicPtr::new(std::ptr::null_mut());
static IGNORE_EPIPE: AtomicBool = AtomicBool::new(false);
static INIT: Once = Once::new();

/// Set the file name to be reported in the event an error is detected
/// by close_stdout.
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

/// Specify the reaction to an EPIPE error during the closing of stdout:
/// - If ignore = true, it shall be ignored.
/// - If ignore = false, it shall evoke a diagnostic, along with a nonzero
///   exit status.
/// The default is ignore = false.
pub fn close_stdout_set_ignore_EPIPE(ignore: bool) {
    IGNORE_EPIPE.store(ignore, Ordering::SeqCst);
}

/// Close standard output. On error, issue a diagnostic and exit
/// with status 'exit_failure'.
/// Also close standard error. On error, exit with status 'exit_failure'.
pub fn close_stdout() {
    let stdout = io::stdout();
    let mut stdout_lock = stdout.lock();
    let result = stdout_lock.flush();

    if let Err(e) = result {
        if !(IGNORE_EPIPE.load(Ordering::SeqCst) && e.kind() == io::ErrorKind::BrokenPipe) {
            let write_error = "write error";
            let file_name_ptr = FILE_NAME.load(Ordering::SeqCst);
            if !file_name_ptr.is_null() {
                let file_name = unsafe { (*file_name_ptr).to_str().unwrap() };
                eprintln!("{}: {}: {}", file_name, write_error, e);
            } else {
                eprintln!("{}: {}", write_error, e);
            }
            process::exit(1);
        }
    }

    // In Rust, standard streams are closed automatically when they go out of scope
    // so we don't need explicit close calls. The flush above ensures all data is written.

    // For stderr, we follow the same pattern
    let stderr = io::stderr();
    let mut stderr_lock = stderr.lock();
    if let Err(e) = stderr_lock.flush() {
        // Skip if sanitizing (not implemented here as Rust doesn't expose sanitizer status)
        eprintln!("stderr flush failed: {}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static TEST_MUTEX: Mutex<()> = Mutex::new(());

    #[test]
    fn test_close_stdout_set_file_name() {
        let _guard = TEST_MUTEX.lock().unwrap();
        close_stdout_set_file_name("test_file");
        // Can't easily test the atomic behavior, but at least verify it doesn't panic
    }

    #[test]
    fn test_close_stdout_set_ignore_epipe() {
        let _guard = TEST_MUTEX.lock().unwrap();
        close_stdout_set_ignore_EPIPE(true);
        assert!(IGNORE_EPIPE.load(Ordering::SeqCst));
        close_stdout_set_ignore_EPIPE(false);
        assert!(!IGNORE_EPIPE.load(Ordering::SeqCst));
    }

    // Note: Testing actual close_stdout behavior is tricky since it affects global state
    // and may interfere with test output. These tests are minimal to verify basic functionality.
}