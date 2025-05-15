use std::ffi::CStr;
use std::ffi::CString;

fn main() {
    // Safe wrapper around setlocale
    let empty_locale = CString::new("").unwrap();
    unsafe {
        libc::setlocale(libc::LC_ALL, empty_locale.as_ptr());
    }

    // Safe wrapper around locale_charset
    let charset_ptr = unsafe { libc::locale_charset() };
    let charset = unsafe { CStr::from_ptr(charset_ptr) };
    println!("{}", charset.to_string_lossy());
}