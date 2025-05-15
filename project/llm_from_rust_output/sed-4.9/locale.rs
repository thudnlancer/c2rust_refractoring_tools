use std::ffi::{CStr, CString};
use std::env;

fn defaulted_getenv(variable: &str) -> String {
    env::var(variable).unwrap_or_default()
}

fn print_category(category: libc::c_int, variable: &str) {
    let value = defaulted_getenv(variable);
    let lc_all = defaulted_getenv("LC_ALL");

    if !value.is_empty() && lc_all.is_empty() {
        println!("{}={}", variable, value);
    } else {
        let locale = unsafe {
            CStr::from_ptr(libc::setlocale(category, std::ptr::null()))
                .to_string_lossy()
                .into_owned()
        };
        println!("{}=\"{}\"", variable, locale);
    }
}

fn main() {
    unsafe {
        libc::setlocale(libc::LC_ALL, CString::new("").unwrap().as_ptr());
    }

    println!("LANG={}", defaulted_getenv("LANG"));
    print_category(libc::LC_CTYPE, "LC_CTYPE");
    print_category(libc::LC_NUMERIC, "LC_NUMERIC");
    print_category(libc::LC_TIME, "LC_TIME");
    print_category(libc::LC_COLLATE, "LC_COLLATE");
    print_category(libc::LC_MONETARY, "LC_MONETARY");
    print_category(libc::LC_MESSAGES, "LC_MESSAGES");
    print_category(libc::LC_PAPER, "LC_PAPER");
    print_category(libc::LC_NAME, "LC_NAME");
    print_category(libc::LC_ADDRESS, "LC_ADDRESS");
    print_category(libc::LC_TELEPHONE, "LC_TELEPHONE");
    print_category(libc::LC_MEASUREMENT, "LC_MEASUREMENT");
    print_category(libc::LC_IDENTIFICATION, "LC_IDENTIFICATION");
    println!("LC_ALL={}", defaulted_getenv("LC_ALL"));
}