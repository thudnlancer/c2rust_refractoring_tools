// Multibyte character data type implementation in Rust

use std::mem;
use std::wchar;
use std::io::{self, Write};
use std::cmp::Ordering;

const MBCHAR_BUF_SIZE: usize = 24;
const MB_UNPRINTABLE_WIDTH: i32 = 1;

#[derive(Debug, Clone)]
pub struct MbChar {
    ptr: *const u8,         // pointer to current character
    bytes: usize,           // number of bytes of current character, > 0
    wc_valid: bool,         // true if wc is a valid wide character
    wc: wchar::wchar_t,     // if wc_valid: the current character
    buf: [u8; MBCHAR_BUF_SIZE], // room for the bytes, used for file input only
}

impl MbChar {
    // Access the current character
    pub fn ptr(&self) -> *const u8 {
        self.ptr
    }

    pub fn len(&self) -> usize {
        self.bytes
    }

    // Comparison of characters
    pub fn iseq(&self, sc: char) -> bool {
        self.wc_valid && self.wc == sc as wchar::wchar_t
    }

    pub fn isnul(&self) -> bool {
        self.wc_valid && self.wc == 0
    }

    pub fn cmp(&self, other: &MbChar) -> Ordering {
        match (self.wc_valid, other.wc_valid) {
            (true, true) => (self.wc as i32).cmp(&(other.wc as i32)),
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => {
                if self.bytes == other.bytes {
                    unsafe {
                        let self_slice = std::slice::from_raw_parts(self.ptr, self.bytes);
                        let other_slice = std::slice::from_raw_parts(other.ptr, other.bytes);
                        self_slice.cmp(other_slice)
                    }
                } else {
                    self.bytes.cmp(&other.bytes)
                }
            }
        }
    }

    pub fn casecmp(&self, other: &MbChar) -> Ordering {
        match (self.wc_valid, other.wc_valid) {
            (true, true) => {
                let wc1 = unsafe { libc::towlower(self.wc as libc::wint_t) };
                let wc2 = unsafe { libc::towlower(other.wc as libc::wint_t) };
                (wc1 as i32).cmp(&(wc2 as i32))
            }
            (true, false) => Ordering::Greater,
            (false, true) => Ordering::Less,
            (false, false) => {
                if self.bytes == other.bytes {
                    unsafe {
                        let self_slice = std::slice::from_raw_parts(self.ptr, self.bytes);
                        let other_slice = std::slice::from_raw_parts(other.ptr, other.bytes);
                        self_slice.cmp(other_slice)
                    }
                } else {
                    self.bytes.cmp(&other.bytes)
                }
            }
        }
    }

    pub fn equal(&self, other: &MbChar) -> bool {
        match (self.wc_valid, other.wc_valid) {
            (true, true) => self.wc == other.wc,
            _ => {
                if self.bytes != other.bytes {
                    false
                } else {
                    unsafe {
                        let self_slice = std::slice::from_raw_parts(self.ptr, self.bytes);
                        let other_slice = std::slice::from_raw_parts(other.ptr, other.bytes);
                        self_slice == other_slice
                    }
                }
            }
        }
    }

    pub fn caseequal(&self, other: &MbChar) -> bool {
        match (self.wc_valid, other.wc_valid) {
            (true, true) => {
                let wc1 = unsafe { libc::towlower(self.wc as libc::wint_t) };
                let wc2 = unsafe { libc::towlower(other.wc as libc::wint_t) };
                wc1 == wc2
            }
            _ => {
                if self.bytes != other.bytes {
                    false
                } else {
                    unsafe {
                        let self_slice = std::slice::from_raw_parts(self.ptr, self.bytes);
                        let other_slice = std::slice::from_raw_parts(other.ptr, other.bytes);
                        self_slice == other_slice
                    }
                }
            }
        }
    }

    // Character classification
    pub fn isascii(&self) -> bool {
        self.wc_valid && self.wc >= 0 && self.wc <= 127
    }

    pub fn isalnum(&self) -> bool {
        self.wc_valid && unsafe { libc::iswalnum(self.wc as libc::wint_t) != 0 }
    }

    pub fn isalpha(&self) -> bool {
        self.wc_valid && unsafe { libc::iswalpha(self.wc as libc::wint_t) != 0 }
    }

    pub fn isblank(&self) -> bool {
        self.wc_valid && unsafe { libc::iswblank(self.wc as libc::wint_t) != 0 }
    }

    pub fn iscntrl(&self) -> bool {
        self.wc_valid && unsafe { libc::iswcntrl(self.wc as libc::wint_t) != 0 }
    }

    pub fn isdigit(&self) -> bool {
        self.wc_valid && unsafe { libc::iswdigit(self.wc as libc::wint_t) != 0 }
    }

    pub fn isgraph(&self) -> bool {
        self.wc_valid && unsafe { libc::iswgraph(self.wc as libc::wint_t) != 0 }
    }

    pub fn islower(&self) -> bool {
        self.wc_valid && unsafe { libc::iswlower(self.wc as libc::wint_t) != 0 }
    }

    pub fn isprint(&self) -> bool {
        self.wc_valid && unsafe { libc::iswprint(self.wc as libc::wint_t) != 0 }
    }

    pub fn ispunct(&self) -> bool {
        self.wc_valid && unsafe { libc::iswpunct(self.wc as libc::wint_t) != 0 }
    }

    pub fn isspace(&self) -> bool {
        self.wc_valid && unsafe { libc::iswspace(self.wc as libc::wint_t) != 0 }
    }

    pub fn isupper(&self) -> bool {
        self.wc_valid && unsafe { libc::iswupper(self.wc as libc::wint_t) != 0 }
    }

    pub fn isxdigit(&self) -> bool {
        self.wc_valid && unsafe { libc::iswxdigit(self.wc as libc::wint_t) != 0 }
    }

    // Character width
    pub fn width(&self) -> i32 {
        if self.wc_valid {
            let w = unsafe { libc::wcwidth(self.wc as libc::wint_t) };
            if w >= 0 {
                w
            } else if unsafe { libc::iswcntrl(self.wc as libc::wint_t) != 0 } {
                0
            } else {
                MB_UNPRINTABLE_WIDTH
            }
        } else {
            MB_UNPRINTABLE_WIDTH
        }
    }

    // Output
    pub fn putc(&self, stream: &mut dyn Write) -> io::Result<()> {
        unsafe {
            let slice = std::slice::from_raw_parts(self.ptr, self.bytes);
            stream.write_all(slice)
        }
    }

    // Assignment
    pub fn setascii(&mut self, sc: char) {
        self.ptr = self.buf.as_ptr();
        self.bytes = 1;
        self.wc_valid = true;
        self.wc = sc as wchar::wchar_t;
        self.buf[0] = sc as u8;
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

// Basic character set check
pub fn is_basic(c: char) -> bool {
    match c {
        '\t' | '\v' | '\f' | ' '..='#' | '%'..='?' | 'A'..='Z' | '[' | '\\' | ']' | '^' | '_' |
        'a'..='z' | '{' | '|' | '}' | '~' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_basic() {
        assert!(is_basic('A'));
        assert!(is_basic('z'));
        assert!(is_basic('0'));
        assert!(is_basic(' '));
        assert!(is_basic('\t'));
        assert!(!is_basic('é'));
        assert!(!is_basic('€'));
    }
}