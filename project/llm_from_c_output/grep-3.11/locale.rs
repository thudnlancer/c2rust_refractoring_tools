/*
 * Program that prints the names of the categories of the current locale.
 * Translated from C to Rust while maintaining original functionality.
 */

use std::env;
use std::ffi::CString;
use std::os::raw::c_int;

extern "C" {
    fn setlocale(category: c_int, locale: *const i8) -> *const i8;
}

const LC_CTYPE: c_int = 0;
const LC_NUMERIC: c_int = 1;
const LC_TIME: c_int = 2;
const LC_COLLATE: c_int = 3;
const LC_MONETARY: c_int = 4;
const LC_MESSAGES: c_int = 5;
const LC_ALL: c_int = 6;
#[cfg(feature = "extended")]
const LC_PAPER: c_int = 7;
#[cfg(feature = "extended")]
const LC_NAME: c_int = 8;
#[cfg(feature = "extended")]
const LC_ADDRESS: c_int = 9;
#[cfg(feature = "extended")]
const LC_TELEPHONE: c_int = 10;
#[cfg(feature = "extended")]
const LC_MEASUREMENT: c_int = 11;
#[cfg(feature = "extended")]
const LC_IDENTIFICATION: c_int = 12;

fn defaulted_getenv(variable: &str) -> String {
    env::var(variable).unwrap_or_default()
}

fn print_category(category: c_int, variable: &str) {
    let value = defaulted_getenv(variable);
    let lc_all = defaulted_getenv("LC_ALL");

    if !value.is_empty() && lc_all.is_empty() {
        println!("{}={}", variable, value);
    } else {
        unsafe {
            let current = setlocale(category, std::ptr::null());
            if !current.is_null() {
                let s = CString::from_raw(current as *mut i8);
                println!("{}=\"{}\"", variable, s.to_string_lossy());
                std::mem::forget(s); // Prevent double-free
            }
        }
    }
}

fn main() {
    unsafe {
        setlocale(LC_ALL, CString::new("").unwrap().as_ptr());
    }

    println!("LANG={}", defaulted_getenv("LANG"));
    print_category(LC_CTYPE, "LC_CTYPE");
    print_category(LC_NUMERIC, "LC_NUMERIC");
    print_category(LC_TIME, "LC_TIME");
    print_category(LC_COLLATE, "LC_COLLATE");
    print_category(LC_MONETARY, "LC_MONETARY");
    print_category(LC_MESSAGES, "LC_MESSAGES");
    
    #[cfg(feature = "extended")]
    print_category(LC_PAPER, "LC_PAPER");
    #[cfg(feature = "extended")]
    print_category(LC_NAME, "LC_NAME");
    #[cfg(feature = "extended")]
    print_category(LC_ADDRESS, "LC_ADDRESS");
    #[cfg(feature = "extended")]
    print_category(LC_TELEPHONE, "LC_TELEPHONE");
    #[cfg(feature = "extended")]
    print_category(LC_MEASUREMENT, "LC_MEASUREMENT");
    #[cfg(feature = "extended")]
    print_category(LC_IDENTIFICATION, "LC_IDENTIFICATION");

    println!("LC_ALL={}", defaulted_getenv("LC_ALL"));
}