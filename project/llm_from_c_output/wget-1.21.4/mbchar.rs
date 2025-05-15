//! Multibyte character data type.
//!
//! A multibyte character is a short subsequence of a char* string,
//! representing a single wide character.
//!
//! We use multibyte characters instead of wide characters because of
//! the following goals:
//! 1) correct multibyte handling, i.e. operate according to the LC_CTYPE
//!    locale,
//! 2) ease of maintenance, i.e. the maintainer needs not know all details
//!    of the ISO C 99 standard,
//! 3) don't fail grossly if the input is not in the encoding set by the
//!    locale, because often different encodings are in use in the same
//!    countries (ISO-8859-1/UTF-8, EUC-JP/Shift_JIS, ...),
//! 4) fast in the case of ASCII characters,
//! 5) portability, i.e. don't make unportable assumptions about wchar_t.

use std::mem;
use std::os::raw::c_char;
use std::slice;
use std::wchar::{wchar_t, wint_t};
use std::wctype::*;

pub const MBCHAR_BUF_SIZE: usize = 24;
pub const MB_UNPRINTABLE_WIDTH: i32 = 1;

#[derive(Debug, Clone)]
pub struct MbChar {
    pub ptr: *const c_char,      // pointer to current character
    pub bytes: usize,            // number of bytes of current character, > 0
    pub wc_valid: bool,          // true if wc is a valid wide character
    pub wc: wchar_t,             // if wc_valid: the current character
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
                if s1.cmp(&s2) == std::cmp::Ordering::Greater { 1 } else { -1 }
            }
        } else {
            unsafe {
                let s1 = slice::from_raw_parts(self.ptr as *const u8, other.bytes);
                let s2 = slice::from_raw_parts(other.ptr as *const u8, other.bytes);
                if s1.cmp(&s2) != std::cmp::Ordering::Less { 1 } else { -1 }
            }
        }
    }

    pub fn casecmp(&self, other: &MbChar) -> i32 {
        if self.wc_valid {
            if other.wc_valid {
                (towlower(self.wc as wint_t) as i32) - (towlower(other.wc as wint_t) as i32)
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
                if s1.cmp(&s2) == std::cmp::Ordering::Greater { 1 } else { -1 }
            }
        } else {
            unsafe {
                let s1 = slice::from_raw_parts(self.ptr as *const u8, other.bytes);
                let s2 = slice::from_raw_parts(other.ptr as *const u8, other.bytes);
                if s1.cmp(&s2) != std::cmp::Ordering::Less { 1 } else { -1 }
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
            towlower(self.wc as wint_t) == towlower(other.wc as wint_t)
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
        self.wc_valid && iswalnum(self.wc as wint_t) != 0
    }

    pub fn isalpha(&self) -> bool {
        self.wc_valid && iswalpha(self.wc as wint_t) != 0
    }

    pub fn isblank(&self) -> bool {
        self.wc_valid && iswblank(self.wc as wint_t) != 0
    }

    pub fn iscntrl(&self) -> bool {
        self.wc_valid && iswcntrl(self.wc as wint_t) != 0
    }

    pub fn isdigit(&self) -> bool {
        self.wc_valid && iswdigit(self.wc as wint_t) != 0
    }

    pub fn isgraph(&self) -> bool {
        self.wc_valid && iswgraph(self.wc as wint_t) != 0
    }

    pub fn islower(&self) -> bool {
        self.wc_valid && iswlower(self.wc as wint_t) != 0
    }

    pub fn isprint(&self) -> bool {
        self.wc_valid && iswprint(self.wc as wint_t) != 0
    }

    pub fn ispunct(&self) -> bool {
        self.wc_valid && iswpunct(self.wc as wint_t) != 0
    }

    pub fn isspace(&self) -> bool {
        self.wc_valid && iswspace(self.wc as wint_t) != 0
    }

    pub fn isupper(&self) -> bool {
        self.wc_valid && iswupper(self.wc as wint_t) != 0
    }

    pub fn isxdigit(&self) -> bool {
        self.wc_valid && iswxdigit(self.wc as wint_t) != 0
    }

    // Extra functions
    pub fn width(&self) -> i32 {
        if self.wc_valid {
            let w = wcwidth(self.wc as wint_t);
            if w >= 0 {
                w
            } else if iswcntrl(self.wc as wint_t) != 0 {
                0
            } else {
                MB_UNPRINTABLE_WIDTH
            }
        } else {
            MB_UNPRINTABLE_WIDTH
        }
    }

    // Output
    pub fn putc(&self, stream: &mut std::fs::File) -> std::io::Result<()> {
        unsafe {
            let bytes = slice::from_raw_parts(self.ptr as *const u8, self.bytes);
            stream.write_all(bytes)
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
            self.buf[..other.bytes].copy_from_slice(unsafe {
                slice::from_raw_parts(other.buf.as_ptr(), other.bytes)
            });
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

// Helper function for is_basic
pub fn is_basic(c: char) -> bool {
    match c {
        '\t' | '\v' | '\f' |
        ' ' | '!' | '"' | '#' | '%' |
        '&' | '\'' | '(' | ')' | '*' |
        '+' | ',' | '-' | '.' | '/' |
        '0' | '1' | '2' | '3' | '4' |
        '5' | '6' | '7' | '8' | '9' |
        ':' | ';' | '<' | '=' | '>' |
        '?' |
        'A' | 'B' | 'C' | 'D' | 'E' |
        'F' | 'G' | 'H' | 'I' | 'J' |
        'K' | 'L' | 'M' | 'N' | 'O' |
        'P' | 'Q' | 'R' | 'S' | 'T' |
        'U' | 'V' | 'W' | 'X' | 'Y' |
        'Z' |
        '[' | '\\' | ']' | '^' | '_' |
        'a' | 'b' | 'c' | 'd' | 'e' |
        'f' | 'g' | 'h' | 'i' | 'j' |
        'k' | 'l' | 'm' | 'n' | 'o' |
        'p' | 'q' | 'r' | 's' | 't' |
        'u' | 'v' | 'w' | 'x' | 'y' |
        'z' | '{' | '|' | '}' | '~' => true,
        _ => false,
    }
}