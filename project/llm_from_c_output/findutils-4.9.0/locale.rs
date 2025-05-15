/* Program that prints the names of the categories of the current locale.
   Translated from C to Rust while maintaining functionality and safety.

   Original Copyright (C) 2019-2022 Free Software Foundation, Inc.
   This Rust version is also free software under the same license terms.
*/

use std::env;
use std::ffi::CString;
use std::os::raw::c_int;

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
            let current = libc::setlocale(category, std::ptr::null());
            if !current.is_null() {
                let current_str = std::ffi::CStr::from_ptr(current).to_string_lossy();
                println!("{}=\"{}\"", variable, current_str);
            }
        }
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
    
    #[cfg(feature = "lc_paper")]
    print_category(libc::LC_PAPER, "LC_PAPER");
    
    #[cfg(feature = "lc_name")]
    print_category(libc::LC_NAME, "LC_NAME");
    
    #[cfg(feature = "lc_address")]
    print_category(libc::LC_ADDRESS, "LC_ADDRESS");
    
    #[cfg(feature = "lc_telephone")]
    print_category(libc::LC_TELEPHONE, "LC_TELEPHONE");
    
    #[cfg(feature = "lc_measurement")]
    print_category(libc::LC_MEASUREMENT, "LC_MEASUREMENT");
    
    #[cfg(feature = "lc_identification")]
    print_category(libc::LC_IDENTIFICATION, "LC_IDENTIFICATION");

    println!("LC_ALL={}", defaulted_getenv("LC_ALL"));
}