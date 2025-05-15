/* Formatted output to a stream.
   Copyright (C) 2004, 2006-2022 Free Software Foundation, Inc.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation, either version 3 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::fmt;
use std::io::{self, Write};
use std::convert::TryFrom;

/// Print formatted output to the given writer.
/// Returns the string length of formatted string. On error, returns a negative value.
pub fn vfprintf<W: Write>(fp: &mut W, format: &str, args: fmt::Arguments<'_>) -> i32 {
    // Buffer for small outputs to avoid allocation
    let mut buf = [0u8; 2000];
    let mut output = Vec::new();
    
    // Format the string
    let formatted = match fmt::write(&mut output, args) {
        Ok(_) => output,
        Err(_) => {
            // Set error on the stream
            let _ = fp.flush();
            return -1;
        }
    };

    // Write the formatted output
    if let Err(_) = fp.write_all(&formatted) {
        return -1;
    }

    // Check for length overflow
    match i32::try_from(formatted.len()) {
        Ok(len) => len,
        Err(_) => {
            // Set error on the stream
            let _ = fp.flush();
            -1
        }
    }
}