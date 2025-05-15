/* Multibyte character data type.
   Copyright (C) 2001, 2005-2007, 2009-2022 Free Software Foundation, Inc.

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

/* Written by Bruno Haible <bruno@clisp.org>.  */

use std::mem;
use std::os::raw::c_char;
use std::slice;
use std::wchar::{wchar_t, wint_t};
use std::wctype::{iswalnum, iswalpha, iswblank, iswcntrl, iswdigit, iswgraph, 
                 iswlower, iswprint, iswpunct, iswspace, iswupper, iswxdigit,
                 towlower, wcwidth};

pub const MBCHAR_BUF_SIZE: usize = 24;
pub const MB_UNPRINTABLE_WIDTH: i32 = 1;

#[derive(Debug, Clone)]
pub struct MbChar {
    pub ptr: *const c_char,    // pointer to current character
    pub bytes: usize,          // number of bytes of current character, > 0
    pub wc_valid: bool,        // true if wc is a valid wide character
    pub wc: wchar_t,           // if wc_valid: the current character
    pub buf: [c_char; MBCHAR_BUF_SIZE], // room for the bytes, used for file input only
}

pub type MbCharT = MbChar;

impl MbChar {
    // Access the current character
    pub fn ptr(&self) -> *const c_char {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.bytes
    }

    // Comparison of characters
    pub fn iseq(&self, sc: char) -> bool {
        self.wc_valid && self.wc == sc as wchar_t
    }

    pub fn isnul(&self) -> bool {
        self.wc_valid && self.wc == 0
    }

    pub fn cmp(&self, other: &MbChar) -> i32 {
        if self.wc_valid {
            if other.wc_valid {
                (self.wc as i32) - (other.wc as i32)
            } else {
                -1
            }
        } else if other.wc_valid {
            1
        } else if self.bytes == other.bytes {
            unsafe {
                let s1 = slice::from_raw_parts(self.ptr as *const u8, self.bytes);
                let s2 = slice::from_raw_parts(other.ptr as *const u8, other.bytes);
                s1.cmp(&s2) as i32
            }
        } else if self.bytes < other.bytes {
            unsafe {
                let s1 = slice::from_raw_parts(self.ptr as *const u8, self.bytes);
                let s2 = slice::from_raw_parts(other.ptr as *const u8, self.bytes);
                if s1 > s2 { 1 } else { -1 }
            }
        } else {
            unsafe {
                let s1 = slice::from_raw_parts(self.ptr as *const u8, other.bytes);
                let s2 = slice::from_raw_parts(other.ptr as *const u8, other.bytes);
                if s1 >= s2 { 1 } else { -1 }
            }
        }
    }

    pub fn casecmp(&self, other: &MbChar) -> i32 {
        if self.wc_valid {
            if other.wc_valid {
                (towlower(self.wc) as i32 - (towlower(other.wc) as i32
            } else {
                -1
            }
        } else if other.wc_valid {
            1
        } else if self.bytes == other.bytes {
            unsafe {
                let s1 = slice::from_raw_parts(self.ptr as *const u8, self.bytes);
                let s2 = slice::from_raw_parts(other.ptr as *const u8, other.bytes);
                s1.cmp(&s2) as i32
            }
        } else if self.bytes < other.bytes {
            unsafe {
                let s1 = slice::from_raw_parts(self.ptr as *const u8, self.bytes);
                let s2 = slice::from_raw_parts(other.ptr as *const u8, self.bytes);
                if s1 > s2 { 1 } else { -1 }
            }
        } else {
            unsafe {
                let s1 = slice::from_raw_parts(self.ptr as *const u8, other.bytes);
                let s2 = slice::from_raw_parts(other.ptr as *const u8, other.bytes);
                if s1 >= s2 { 1 } else { -1 }
            }
        }
    }

    pub fn equal(&self, other: &MbChar) -> bool {
        if self.wc_valid && other.wc_valid {
            self.wc == other.wc
        } else {
            self.bytes == other.bytes && unsafe {
                let s1 = slice::from_raw_parts(self.ptr as *const u8, self.bytes);
                let s2 = slice::from_raw_parts(other.ptr as *const u8, other.bytes);
                s1 == s2
            }
        }
    }

    pub fn caseequal(&self, other: &MbChar) -> bool {
        if self.wc_valid && other.wc_valid {
            towlower(self.wc) == towlower(other.wc)
        } else {
            self.bytes == other.bytes && unsafe {
                let s1 = slice::from_raw_parts(self.ptr as *const u8, self.bytes);
                let s2 = slice::from_raw_parts(other.ptr as *const u8, other.bytes);
                s1 == s2
            }
        }
    }

    // Character classification
    pub fn isascii(&self) -> bool {
        self.wc_valid && self.wc >= 0 && self.wc <= 127
    }

    pub fn isalnum(&self) -> bool {
        self.wc_valid && iswalnum(self.wc) != 0
    }

    pub fn isalpha(&self) -> bool {
        self.wc_valid && iswalpha(self.wc) != 0
    }

    pub fn isblank(&self) -> bool {
        self.wc_valid && iswblank(self.wc) != 0
    }

    pub fn iscntrl(&self) -> bool {
        self.wc_valid && iswcntrl(self.wc) != 0
    }

    pub fn isdigit(&self) -> bool {
        self.wc_valid && iswdigit(self.wc) != 0
    }

    pub fn isgraph(&self) -> bool {
        self.wc_valid && iswgraph(self.wc) != 0
    }

    pub fn islower(&self) -> bool {
        self.wc_valid && iswlower(self.wc) != 0
    }

    pub fn isprint(&self) -> bool {
        self.wc_valid && iswprint(self.wc) != 0
    }

    pub fn ispunct(&self) -> bool {
        self.wc_valid && iswpunct(self.wc) != 0
    }

    pub fn isspace(&self) -> bool {
        self.wc_valid && iswspace(self.wc) != 0
    }

    pub fn isupper(&self) -> bool {
        self.wc_valid && iswupper(self.wc) != 0
    }

    pub fn isxdigit(&self) -> bool {
        self.wc_valid && iswxdigit(self.wc) != 0
    }

    // Character width
    pub fn width(&self) -> i32 {
        if self.wc_valid {
            let w = unsafe { wcwidth(self.wc as wint_t) };
            if w >= 0 {
                w
            } else if iswcntrl(self.wc) != 0 {
                0
            } else {
                MB_UNPRINTABLE_WIDTH
            }
        } else {
            MB_UNPRINTABLE_WIDTH
        }
    }

    // Assignment
    pub fn setascii(&mut self, sc: char) {
        self.ptr = self.buf.as_ptr();
        self.bytes = 1;
        self.wc_valid = true;
        self.wc = sc as wchar_t;
        self.buf[0] = sc as c_char;
    }

    // Copying
    pub fn copy(&mut self, other: &MbChar) {
        if other.ptr == other.buf.as_ptr() {
            self.buf[..other.bytes].copy_from_slice(&other.buf[..other.bytes]);
            self.ptr = self.buf.as_ptr();
        } else {
            self.ptr = other.ptr;
        }
        self.bytes = other.bytes;
        self.wc_valid = other.wc_valid;
        if self.wc_valid {
            self.wc = other.wc;
        }
    }
}

// is_basic tests whether the single-byte character c is in the ISO C "basic character set"
pub fn is_basic(c: char) -> bool {
    match c {
        '\t' | '\v' | '\f' | ' ' | '!' | '"' | '#' | '%' | '&' | '\'' | '(' | ')' | '*' |
        '+' | ',' | '-' | '.' | '/' | '0'..='9' | ':' | ';' | '<' | '=' | '>' | '?' |
        'A'..='Z' | '[' | '\\' | ']' | '^' | '_' | 'a'..='z' | '{' | '|' | '}' | '~' => true,
        _ => false,
    }
}