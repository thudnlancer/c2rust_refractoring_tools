use std::process;
use gettextrs::{dcgettext, TextDomain};

const GETTEXT_CATEGORY_LC_MESSAGES: i32 = 5;

fn openat_save_fail(errnum: i32) -> ! {
    let msg = unsafe {
        dcgettext(
            "",
            "unable to record current working directory",
            GETTEXT_CATEGORY_LC_MESSAGES,
        )
    };
    let msg_str = unsafe { std::ffi::CStr::from_ptr(msg) }.to_string_lossy();
    eprintln!("error: {} (errno: {})", msg_str, errnum);
    process::abort();
}

fn openat_restore_fail(errnum: i32) -> ! {
    let msg = unsafe {
        dcgettext(
            "",
            "failed to return to initial working directory",
            GETTEXT_CATEGORY_LC_MESSAGES,
        )
    };
    let msg_str = unsafe { std::ffi::CStr::from_ptr(msg) }.to_string_lossy();
    eprintln!("error: {} (errno: {})", msg_str, errnum);
    process::abort();
}