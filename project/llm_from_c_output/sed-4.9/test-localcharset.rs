/* Manual test of localcharset() function.
   Copyright (C) 2018-2022 Free Software Foundation, Inc.

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

/* This program prints the result of locale_charset in the current locale.
   One way to use it is:
     $ for l in `locale -a`; do
         echo -n "$l               "; LANG=$l ./test-localcharset;
       done \
       | sort -k 2
 */

use std::env;
use std::io::{self, Write};
use std::ffi::CString;
use libc::{setlocale, LC_ALL};

extern "C" {
    fn locale_charset() -> *const libc::c_char;
}

fn main() -> io::Result<()> {
    unsafe {
        let _ = setlocale(LC_ALL, CString::new("").unwrap().as_ptr());
    }
    
    let charset = unsafe {
        let c_str = locale_charset();
        if c_str.is_null() {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to get locale charset"));
        }
        std::ffi::CStr::from_ptr(c_str).to_string_lossy().into_owned()
    };
    
    println!("{}", charset);
    Ok(())
}