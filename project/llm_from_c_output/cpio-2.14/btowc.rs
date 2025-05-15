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

const WEOF: Option<char> = None;

fn btowc(c: i32) -> Option<char> {
    if c != -1 {  // -1 represents EOF in C
        let buf = [c as u8];
        
        #[cfg(feature = "mbrtowc")]
        {
            use std::str;
            let s = match str::from_utf8(&buf) {
                Ok(s) => s,
                Err(_) => return WEOF,
            };
            let mut chars = s.chars();
            if let Some(wc) = chars.next() {
                if chars.next().is_none() {
                    return Some(wc);
                }
            }
        }
        
        #[cfg(not(feature = "mbrtowc"))]
        {
            if let Ok(s) = String::from_utf8(vec![buf[0]]) {
                let mut chars = s.chars();
                if let Some(wc) = chars.next() {
                    if chars.next().is_none() {
                        return Some(wc);
                    }
                }
            }
        }
    }
    WEOF
}