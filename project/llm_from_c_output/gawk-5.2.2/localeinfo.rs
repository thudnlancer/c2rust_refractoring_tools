// locale information

// Copyright 2016-2023 Free Software Foundation, Inc.

// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3, or (at your option)
// any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street - Fifth Floor, Boston, MA
// 02110-1301, USA.

// Written by Paul Eggert.

use std::char;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ffi::CString;
use std::mem;
use std::os::raw::c_char;
use std::ptr;

#[derive(Debug)]
pub struct LocaleInfo {
    /// MB_CUR_MAX > 1
    pub multibyte: bool,
    /// The locale is simple, like the C locale
    pub simple: bool,
    /// The locale uses UTF-8
    pub using_utf8: bool,
    /// Byte classification array
    pub sbclen: [i8; 256],
    /// Wide character mapping array
    pub sbctowc: [char; 256],
}

impl Default for LocaleInfo {
    fn default() -> Self {
        LocaleInfo {
            multibyte: false,
            simple: false,
            using_utf8: false,
            sbclen: [0; 256],
            sbctowc: ['\0'; 256],
        }
    }
}

/// Maximum number of characters that can be the case-folded counterparts
pub const CASE_FOLDED_BUFSIZE: usize = 32;

/// Return true if the locale uses UTF-8
fn is_using_utf8() -> bool {
    let s = "\u{0100}";
    let mut chars = s.chars();
    chars.next().is_some() && chars.next().is_none()
}

/// Return true if the locale is compatible enough with the C locale
fn using_simple_locale(multibyte: bool) -> bool {
    const NATIVE_C_CHARSET: bool = [
        8, 9, 10, 11, 12, 13, 32, 33, 34, 35, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 57,
        58, 59, 60, 61, 62, 63, 65, 90, 91, 92, 93, 94, 95, 97, 122, 123, 124, 125, 126,
    ]
    .iter()
    .zip([
        '\x08', '\t', '\n', '\x0B', '\x0C', '\r', ' ', '!', '"', '#', '%', '&', '\'', '(', ')',
        '*', '+', ',', '-', '.', '/', '0', '9', ':', ';', '<', '=', '>', '?', 'A', 'Z', '[', '\\',
        ']', '^', '_', 'a', 'z', '{', '|', '}', '~',
    ])
    .all(|(&expected, actual)| actual as u32 == expected);

    if !NATIVE_C_CHARSET || multibyte {
        return false;
    }

    // Compare native character order using strcoll
    for i in 0..=u8::MAX {
        let s1 = [i, 0];
        let s2 = [i.wrapping_add(1), 0];
        
        let c_s1 = CString::new(&s1[..]).unwrap();
        let c_s2 = CString::new(&s2[..]).unwrap();
        
        unsafe {
            if libc::strcoll(c_s1.as_ptr(), c_s2.as_ptr()) >= 0 {
                return false;
            }
        }
    }

    true
}

/// Initialize LocaleInfo from the current locale
pub fn init_localeinfo(localeinfo: &mut LocaleInfo) {
    localeinfo.multibyte = unsafe { libc::MB_CUR_MAX } > 1;
    localeinfo.simple = using_simple_locale(localeinfo.multibyte);
    localeinfo.using_utf8 = is_using_utf8();

    for i in 0..=255 {
        let c = i as c_char;
        let mut s = unsafe { mem::zeroed() };
        let mut wc = 0;
        let len = unsafe { libc::mbrtowc(&mut wc, &c, 1, &mut s) };

        localeinfo.sbclen[i] = if len <= 1 { 1 } else { -(len as i8) };
        localeinfo.sbctowc[i] = if len <= 1 {
            char::from_u32(wc as u32).unwrap_or('\0')
        } else {
            '\0'
        };
    }
}

/// Lonesome lower case characters
const LONESOME_LOWER: [char; 15] = [
    '\u{00B5}', '\u{0131}', '\u{017F}', '\u{01C5}', '\u{01C8}', '\u{01CB}', '\u{01F2}', '\u{0345}',
    '\u{03C2}', '\u{03D0}', '\u{03D1}', '\u{03D5}', '\u{03D6}', '\u{03F0}', '\u{03F1}',
];

/// Find case-folded counterparts
pub fn case_folded_counterparts(c: char, folded: &mut [char; CASE_FOLDED_BUFSIZE]) -> usize {
    let mut n = 0;
    let uc = c.to_uppercase().next().unwrap_or(c);
    let lc = uc.to_lowercase().next().unwrap_or(uc);

    if uc != c {
        folded[n] = uc;
        n += 1;
    }

    if lc != uc && lc != c && lc.to_uppercase().next().unwrap_or(lc) == uc {
        folded[n] = lc;
        n += 1;
    }

    for &li in &LONESOME_LOWER {
        if li != lc && li != uc && li != c && li.to_uppercase().next().unwrap_or(li) == uc {
            folded[n] = li;
            n += 1;
        }
    }

    n
}