/* Convert unibyte character to wide character.
   Copyright (C) 2008, 2010-2023 Free Software Foundation, Inc.
   Written by Bruno Haible <bruno@clisp.org>, 2008.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation; either version 2.1 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::char;
use std::io::Error;

/// Converts a byte to a wide character (wchar_t equivalent in Rust is char).
/// Returns None (equivalent to WEOF) on failure.
pub fn btowc(c: i32) -> Option<char> {
    if c != -1 {  // -1 represents EOF in C
        let buf = [c as u8];
        
        // Try to convert the single byte to a char
        if let Ok(s) = std::str::from_utf8(&buf) {
            if let Some(wc) = s.chars().next() {
                return Some(wc);
            }
        }
    }
    None
}