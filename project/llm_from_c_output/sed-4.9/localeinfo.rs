// locale information
//
// Copyright 2016-2022 Free Software Foundation, Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street - Fifth Floor, Boston, MA
// 02110-1301, USA.

use std::char;
use std::cmp::Ordering;
use std::collections::HashSet;
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
    /// Array indicating single-byte character status
    pub sbclen: [i8; u8::MAX as usize + 1],
    /// Array mapping single-byte characters to wide characters
    pub sbctowc: [u32; u8::MAX as usize + 1],
}

impl Default for LocaleInfo {
    fn default() -> Self {
        Self {
            multibyte: false,
            simple: false,
            using_utf8: false,
            sbclen: [0; u8::MAX as usize + 1],
            sbctowc: [0; u8::MAX as usize + 1],
        }
    }
}

/// Maximum number of characters that can be the case-folded
/// counterparts of a single character, not counting the character
/// itself.
pub const CASE_FOLDED_BUFSIZE: usize = 32;

/// Check if the locale uses UTF-8
fn is_using_utf8() -> bool {
    let bytes = b"\xc4\x80";
    let s = std::str::from_utf8(bytes).unwrap();
    s.chars().next() == Some('\u{0100}')
}

/// Check if the locale is compatible with the C locale
fn using_simple_locale(multibyte: bool) -> bool {
    // Check native character set compatibility
    let native_c_charset = [
        ('\x08', 8), ('\t', 9), ('\n', 10), ('\x0B', 11), ('\x0C', 12),
        ('\r', 13), (' ', 32), ('!', 33), ('"', 34), ('#', 35),
        ('%', 37), ('&', 38), ('\'', 39), ('(', 40), (')', 41),
        ('*', 42), ('+', 43), (',', 44), ('-', 45), ('.', 46),
        ('/', 47), ('0', 48), ('9', 57), (':', 58), (';', 59),
        ('<', 60), ('=', 61), ('>', 62), ('?', 63), ('A', 65),
        ('Z', 90), ('[', 91), ('\\', 92), (']', 93), ('^', 94),
        ('_', 95), ('a', 97), ('z', 122), ('{', 123), ('|', 124),
        ('}', 125), ('~', 126),
    ];

    if !native_c_charset.iter().all(|&(c, v)| c as u32 == v) || multibyte {
        return false;
    }

    // Compare native character order using strcoll
    for i in 0..=u8::MAX - 1 {
        let s1 = [i as c_char, 0];
        let s2 = [i as c_char + 1, 0];
        
        let cs1 = unsafe { CString::from_vec_unchecked(vec![i, 0]) };
        let cs2 = unsafe { CString::from_vec_unchecked(vec![i + 1, 0]) };
        
        let res = unsafe { libc::strcoll(cs1.as_ptr(), cs2.as_ptr()) };
        if res >= 0 {
            return false;
        }
    }

    true
}

/// Initialize LocaleInfo from the current locale
pub fn init_localeinfo(localeinfo: &mut LocaleInfo) {
    localeinfo.multibyte = unsafe { libc::MB_CUR_MAX } > 1;
    localeinfo.simple = using_simple_locale(localeinfo.multibyte);
    localeinfo.using_utf8 = is_using_utf8();

    for i in 0..=u8::MAX {
        let c = i as char;
        let mut mbstate = unsafe { mem::zeroed() };
        let mut wc = 0;
        
        let bytes = [i];
        let len = unsafe {
            libc::mbrtowc(
                &mut wc,
                bytes.as_ptr() as *const c_char,
                bytes.len(),
                &mut mbstate,
            )
        };

        localeinfo.sbclen[i as usize] = match len {
            libc::size_t::MAX => -1,
            0 | 1 => 1,
            n => -i8::try_from(n).unwrap_or(-1),
        };

        localeinfo.sbctowc[i as usize] = match len {
            libc::size_t::MAX => u32::MAX,
            0 | 1 => wc as u32,
            _ => u32::MAX,
        };
    }
}

/// Characters with special case folding behavior
const LONESOME_LOWER: [u32; 15] = [
    0x00B5, 0x0131, 0x017F, 0x01C5, 0x01C8, 0x01CB, 0x01F2, 0x0345,
    0x03C2, 0x03D0, 0x03D1, 0x03D5, 0x03D6, 0x03F0, 0x03F1,
];

/// Find case-folded counterparts of a character
pub fn case_folded_counterparts(c: Option<char>) -> Vec<char> {
    let mut folded = Vec::with_capacity(CASE_FOLDED_BUFSIZE);
    
    if let Some(c) = c {
        let uc = c.to_uppercase().next().unwrap_or(c);
        let lc = uc.to_lowercase().next().unwrap_or(uc);
        
        if uc != c {
            folded.push(uc);
        }
        
        if lc != uc && lc != c && lc.to_uppercase().next().unwrap_or(lc) == uc {
            folded.push(lc);
        }
        
        for &li in &LONESOME_LOWER {
            if let Some(li_char) = char::from_u32(li) {
                if li_char != lc && li_char != uc && li_char != c 
                    && li_char.to_uppercase().next().unwrap_or(li_char) == uc {
                    folded.push(li_char);
                }
            }
        }
    }
    
    folded
}