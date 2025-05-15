use std::process;
use gettextrs::{dcgettext, TextDomain};

const DOMAIN: &str = "";
const CATEGORY: i32 = 5; // LC_MESSAGES

fn openat_save_fail(errnum: i32) -> ! {
    let message = unsafe {
        dcgettext(
            DOMAIN.as_ptr() as *const libc::c_char,
            "unable to record current working directory\0".as_ptr() as *const libc::c_char,
            CATEGORY,
        )
    };
    let message = unsafe { std::ffi::CStr::from_ptr(message) }.to_string_lossy();
    eprintln!("error: {} (errno: {})", message, errnum);
    process::abort();
}

fn openat_restore_fail(errnum: i32) -> ! {
    let message = unsafe {
        dcgettext(
            DOMAIN.as_ptr() as *const libc::c_char,
            "failed to return to initial working directory\0".as_ptr() as *const libc::c_char,
            CATEGORY,
        )
    };
    let message = unsafe { std::ffi::CStr::from_ptr(message) }.to_string_lossy();
    eprintln!("error: {} (errno: {})", message, errnum);
    process::abort();
}