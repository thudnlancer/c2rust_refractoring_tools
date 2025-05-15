// Close standard output and standard error, exiting with a diagnostic on error.

use std::ffi::CString;
use std::io;
use std::process;
use std::sync::atomic::{AtomicBool, AtomicPtr, Ordering};
use std::sync::Once;

static FILE_NAME: AtomicPtr<u8> = AtomicPtr::new(std::ptr::null_mut());
static IGNORE_EPIPE: AtomicBool = AtomicBool::new(false);
static INIT: Once = Once::new();

#[cfg(any(
    feature = "sanitize_address",
    all(
        not(feature = "no_sanitize_address"),
        any(
            target_feature = "address_sanitizer",
            all(
                not(target_arch = "wasm32"),
                not(target_arch = "wasm64"),
                sanitize = "address"
            )
        )
    )
))]
const SANITIZE_ADDRESS: bool = true;
#[cfg(not(any(
    feature = "sanitize_address",
    all(
        not(feature = "no_sanitize_address"),
        any(
            target_feature = "address_sanitizer",
            all(
                not(target_arch = "wasm32"),
                not(target_arch = "wasm64"),
                sanitize = "address"
            )
        )
    )
)))]
const SANITIZE_ADDRESS: bool = false;

/// Set the file name to be reported in the event an error is detected by close_stdout.
pub fn close_stdout_set_file_name(file: Option<&str>) {
    INIT.call_once(|| {
        // Initialize the atomic pointer
    });

    let ptr = match file {
        Some(f) => {
            let cstr = CString::new(f).expect("CString::new failed");
            let ptr = cstr.into_raw();
            ptr
        }
        None => std::ptr::null_mut(),
    };

    let old_ptr = FILE_NAME.swap(ptr, Ordering::SeqCst);
    if !old_ptr.is_null() {
        unsafe { CString::from_raw(old_ptr) };
    }
}

/// Specify the reaction to an EPIPE error during the closing of stdout:
/// - If ignore = true, it shall be ignored.
/// - If ignore = false, it shall evoke a diagnostic, along with a nonzero exit status.
/// The default is ignore = false.
pub fn close_stdout_set_ignore_epipe(ignore: bool) {
    IGNORE_EPIPE.store(ignore, Ordering::SeqCst);
}

/// Close standard output. On error, issue a diagnostic and exit with status 'exit_failure'.
/// Also close standard error. On error, exit with status 'exit_failure'.
pub fn close_stdout() {
    fn handle_error(file_name: Option<&str>, err: io::Error) -> ! {
        let write_error = "write error";
        if let Some(file) = file_name {
            eprintln!("{}: {}: {}", file, err, write_error);
        } else {
            eprintln!("{}: {}", err, write_error);
        }
        process::exit(1);
    }

    // Close stdout
    if let Err(err) = std::io::stdout().into_raw().and_then(|handle| {
        let mut file = unsafe { std::fs::File::from_raw_fd(handle) };
        file.flush()?;
        file.sync_all()?;
        drop(file);
        Ok(())
    }) {
        if !(IGNORE_EPIPE.load(Ordering::SeqCst) && err.kind() == io::ErrorKind::BrokenPipe) {
            let file_name_ptr = FILE_NAME.load(Ordering::SeqCst);
            let file_name = if file_name_ptr.is_null() {
                None
            } else {
                unsafe { Some(std::ffi::CStr::from_ptr(file_name_ptr).to_str().unwrap_or("")) }
            };
            handle_error(file_name, err);
        }
    }

    // Close stderr only if not sanitizing
    if !SANITIZE_ADDRESS {
        if let Err(err) = std::io::stderr().into_raw().and_then(|handle| {
            let mut file = unsafe { std::fs::File::from_raw_fd(handle) };
            file.flush()?;
            file.sync_all()?;
            drop(file);
            Ok(())
        }) {
            process::exit(1);
        }
    }
}