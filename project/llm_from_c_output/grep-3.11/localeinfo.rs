// locale information
//
// Copyright 2016-2023 Free Software Foundation, Inc.
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

const UCHAR_MAX: usize = u8::MAX as usize;
const CASE_FOLDED_BUFSIZE: usize = 32;

#[derive(Debug)]
pub struct LocaleInfo {
    /// MB_CUR_MAX > 1
    pub multibyte: bool,
    /// The locale is simple, like the C locale
    pub simple: bool,
    /// The locale uses UTF-8
    pub using_utf8: bool,
    /// Single-byte character lengths
    pub sbclen: [i8; UCHAR_MAX + 1],
    /// Wide character equivalents
    pub sbctowc: [char; UCHAR_MAX + 1],
}

impl Default for LocaleInfo {
    fn default() -> Self {
        Self {
            multibyte: false,
            simple: false,
            using_utf8: false,
            sbclen: [0; UCHAR_MAX + 1],
            sbctowc: [char::REPLACEMENT_CHARACTER; UCHAR_MAX + 1],
        }
    }
}

/// Return true if the locale uses UTF-8
fn is_using_utf8() -> bool {
    let s = b"\xc4\x80";
    let mut chars = s.iter().copied().collect::<Vec<_>>();
    chars.push(0);
    
    let c_str = CString::new(chars).unwrap();
    let mbs = unsafe { libc::mbrtowc(
        std::ptr::null_mut(),
        c_str.as_ptr(),
        s.len(),
        std::ptr::null_mut()
    ) };
    
    mbs == 2 && unsafe { *libc::__errno_location() } == 0
}

/// Return true if the locale is compatible with the C locale
fn using_simple_locale(multibyte: bool) -> bool {
    const NATIVE_C_CHARSET: bool = {
        let mut result = true;
        result &= '\x08' as u8 == 8;
        result &= '\x09' as u8 == 9;
        result &= '\x0A' as u8 == 10;
        result &= '\x0B' as u8 == 11;
        result &= '\x0C' as u8 == 12;
        result &= '\x0D' as u8 == 13;
        result &= ' ' as u8 == 32;
        result &= '!' as u8 == 33;
        result &= '"' as u8 == 34;
        result &= '#' as u8 == 35;
        result &= '%' as u8 == 37;
        result &= '&' as u8 == 38;
        result &= '\'' as u8 == 39;
        result &= '(' as u8 == 40;
        result &= ')' as u8 == 41;
        result &= '*' as u8 == 42;
        result &= '+' as u8 == 43;
        result &= ',' as u8 == 44;
        result &= '-' as u8 == 45;
        result &= '.' as u8 == 46;
        result &= '/' as u8 == 47;
        result &= '0' as u8 == 48;
        result &= '9' as u8 == 57;
        result &= ':' as u8 == 58;
        result &= ';' as u8 == 59;
        result &= '<' as u8 == 60;
        result &= '=' as u8 == 61;
        result &= '>' as u8 == 62;
        result &= '?' as u8 == 63;
        result &= 'A' as u8 == 65;
        result &= 'Z' as u8 == 90;
        result &= '[' as u8 == 91;
        result &= '\\' as u8 == 92;
        result &= ']' as u8 == 93;
        result &= '^' as u8 == 94;
        result &= '_' as u8 == 95;
        result &= 'a' as u8 == 97;
        result &= 'z' as u8 == 122;
        result &= '{' as u8 == 123;
        result &= '|' as u8 == 124;
        result &= '}' as u8 == 125;
        result &= '~' as u8 == 126;
        result
    };

    if !NATIVE_C_CHARSET || multibyte {
        return false;
    }

    for i in 0..UCHAR_MAX {
        let s1 = [i as u8, 0];
        let s2 = [(i + 1) as u8, 0];
        
        let c_s1 = CString::new(s1).unwrap();
        let c_s2 = CString::new(s2).unwrap();
        
        let cmp = unsafe { libc::strcoll(c_s1.as_ptr(), c_s2.as_ptr()) };
        if cmp >= 0 {
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

    for i in 0..=UCHAR_MAX {
        let c = i as u8;
        let mut s = unsafe { mem::zeroed() };
        let mut wc = 0;
        let len = unsafe { libc::mbrtowc(&mut wc, &c as *const u8 as *const c_char, 1, &mut s) };
        
        localeinfo.sbclen[i] = if len <= 1 { 1 } else { -(len as i8) };
        localeinfo.sbctowc[i] = if len <= 1 {
            char::from_u32(wc as u32).unwrap_or(char::REPLACEMENT_CHARACTER)
        } else {
            char::REPLACEMENT_CHARACTER
        };
    }
}

/// Lonesome lowercase characters
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