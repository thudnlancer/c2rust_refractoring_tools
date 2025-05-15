// Convert unibyte character to wide character.
// Copyright (C) 2008, 2010-2023 Free Software Foundation, Inc.
// Written by Bruno Haible <bruno@clisp.org>, 2008.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::char;
use std::io;

/// Converts a byte to a wide character (wchar_t equivalent).
/// Returns WEOF (None) on failure or if c is EOF.
pub fn btowc(c: i32) -> Option<char> {
    if c != -1 {  // -1 represents EOF in C
        let byte = c as u8;
        let mut buf = [0u8; 1];
        buf[0] = byte;
        
        // Try to convert using Rust's built-in char conversion
        if let Ok(s) = std::str::from_utf8(&buf) {
            if let Some(c) = s.chars().next() {
                return Some(c);
            }
        }
        
        // Fallback for non-UTF-8 encodings (simplified version)
        // Note: This doesn't fully replicate C's locale-dependent behavior
        if byte.is_ascii() {
            return Some(byte as char);
        }
    }
    None  // WEOF equivalent
}