/* Program that prints the names of the categories of the current locale.
   Copyright (C) 2019-2022 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* Written by Bruno Haible <bruno@clisp.org>, 2019.  */

use std::env;
use std::ffi::CString;
use std::os::raw::c_int;
use libc::{setlocale, LC_ALL, LC_CTYPE, LC_NUMERIC, LC_TIME, LC_COLLATE, LC_MONETARY, LC_MESSAGES};

#[cfg(feature = "extended")]
use libc::{LC_PAPER, LC_NAME, LC_ADDRESS, LC_TELEPHONE, LC_MEASUREMENT, LC_IDENTIFICATION};

fn defaulted_getenv(variable: &str) -> String {
    env::var(variable).unwrap_or_default()
}

fn print_category(category: c_int, variable: &str) {
    let value = defaulted_getenv(variable);
    let lc_all = defaulted_getenv("LC_ALL");
    
    unsafe {
        if !value.is_empty() && lc_all.is_empty() {
            println!("{}={}", variable, value);
        } else {
            let locale = setlocale(category, std::ptr::null());
            let locale_str = std::ffi::CStr::from_ptr(locale).to_string_lossy();
            println!("{}=\"{}\"", variable, locale_str);
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
    {
        print_category(LC_PAPER, "LC_PAPER");
        print_category(LC_NAME, "LC_NAME");
        print_category(LC_ADDRESS, "LC_ADDRESS");
        print_category(LC_TELEPHONE, "LC_TELEPHONE");
        print_category(LC_MEASUREMENT, "LC_MEASUREMENT");
        print_category(LC_IDENTIFICATION, "LC_IDENTIFICATION");
    }

    println!("LC_ALL={}", defaulted_getenv("LC_ALL"));
}