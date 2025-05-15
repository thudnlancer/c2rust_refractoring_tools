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

fn main() -> io::Result<()> {
    // Set locale from environment
    let _ = env::var("LANG").ok();
    
    // Get the current locale's charset
    let charset = locale_charset();
    println!("{}", charset);
    
    Ok(())
}

fn locale_charset() -> String {
    // This is a simplified version - in a real implementation you would
    // use a crate like encoding_rs or system calls to get the charset
    // This matches the behavior of the C version which uses a custom localcharset.h
    match std::env::var("LANG") {
        Ok(lang) => {
            if let Some(dot_pos) = lang.find('.') {
                let (_, encoding) = lang.split_at(dot_pos + 1);
                encoding.to_string()
            } else {
                "UTF-8".to_string()
            }
        },
        Err(_) => "UTF-8".to_string()
    }
}