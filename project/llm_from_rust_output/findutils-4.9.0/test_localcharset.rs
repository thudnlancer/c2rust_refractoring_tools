use std::ffi::{CStr, CString};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Set locale to empty string to use environment's default locale
    let empty = CString::new("").unwrap();
    unsafe { libc::setlocale(libc::LC_ALL, empty.as_ptr()); }

    // Get and print the current locale's charset
    let charset = unsafe { CStr::from_ptr(libc::locale_charset()) };
    println!("{}", charset.to_str().unwrap());

    Ok(())
}