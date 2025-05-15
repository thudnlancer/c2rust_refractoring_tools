use std::process;
use gettextrs::{dcgettext, LocaleCategory};

pub fn xalloc_die() -> ! {
    let msg = unsafe {
        dcgettext(
            "wget-gnulib\0".as_ptr() as *const libc::c_char,
            "memory exhausted\0".as_ptr() as *const libc::c_char,
            LocaleCategory::LC_MESSAGES,
        )
    };
    let msg_str = unsafe { std::ffi::CStr::from_ptr(msg) }.to_string_lossy();
    eprintln!("{}", msg_str);
    process::exit(1);
}