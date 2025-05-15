//! Iterating through multibyte strings without knowing length a-priori.
//!
//! This module provides safe iteration over multibyte strings, handling
//! invalid sequences while preserving them, similar to the C mbuiter.h.

use std::mem;
use std::ptr;
use std::str;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_void;

/// Represents a multibyte character with its metadata
#[derive(Debug, Clone)]
pub struct MbChar {
    pub ptr: *const c_char,
    pub bytes: usize,
    pub wc_valid: bool,
    pub wc: wchar_t,
}

impl MbChar {
    pub fn new() -> Self {
        MbChar {
            ptr: ptr::null(),
            bytes: 0,
            wc_valid: false,
            wc: 0,
        }
    }

    pub fn is_null(&self) -> bool {
        unsafe { *self.ptr == 0 }
    }
}

/// Multibyte string iterator state
#[derive(Debug, Clone)]
pub struct MbuIterMulti {
    in_shift: bool,
    state: mbstate_t,
    next_done: bool,
    cur: MbChar,
}

impl MbuIterMulti {
    /// Creates a new iterator starting at the given pointer
    pub fn new(start_ptr: *const c_char) -> Self {
        MbuIterMulti {
            in_shift: false,
            state: unsafe { mem::zeroed() },
            next_done: false,
            cur: MbChar {
                ptr: start_ptr,
                bytes: 0,
                wc_valid: false,
                wc: 0,
            },
        }
    }

    /// Advances to the next multibyte character
    pub fn next(&mut self) {
        if self.next_done {
            return;
        }

        if self.in_shift {
            self.process_with_shift();
        } else {
            // Handle ASCII quickly without mbrtowc
            unsafe {
                if is_basic(*self.cur.ptr) {
                    self.cur.bytes = 1;
                    self.cur.wc = *self.cur.ptr as wchar_t;
                    self.cur.wc_valid = true;
                } else {
                    assert!(mbsinit(&self.state));
                    self.in_shift = true;
                    self.process_with_shift();
                }
            }
        }
        self.next_done = true;
    }

    fn process_with_shift(&mut self) {
        unsafe {
            let max_bytes = MB_CUR_MAX;
            let mut bytes_remaining = 0;
            let mut p = self.cur.ptr;
            while *p != 0 && bytes_remaining < max_bytes {
                bytes_remaining += 1;
                p = p.offset(1);
            }

            let bytes = mbrtowc(
                &mut self.cur.wc,
                self.cur.ptr,
                bytes_remaining,
                &mut self.state,
            );

            match bytes {
                size_t::MAX => {
                    // Invalid sequence
                    self.cur.bytes = 1;
                    self.cur.wc_valid = false;
                }
                size_t::MAX - 1 => {
                    // Incomplete character
                    self.cur.bytes = unsafe { CStr::from_ptr(self.cur.ptr).to_bytes().len() };
                    self.cur.wc_valid = false;
                }
                _ => {
                    if bytes == 0 {
                        // Null character
                        self.cur.bytes = 1;
                        assert_eq!(*self.cur.ptr, 0);
                        assert_eq!(self.cur.wc, 0);
                    }
                    self.cur.wc_valid = true;

                    // Return to fast path if back in initial state
                    if mbsinit(&self.state) {
                        self.in_shift = false;
                    }
                }
            }
        }
    }

    /// Relocates the iterator when the string moves
    pub fn reloc(&mut self, ptrdiff: isize) {
        unsafe {
            self.cur.ptr = self.cur.ptr.offset(ptrdiff);
        }
    }

    /// Checks if current character is available
    pub fn avail(&mut self) -> bool {
        self.next();
        !self.cur.is_null()
    }

    /// Advances to next character
    pub fn advance(&mut self) {
        unsafe {
            self.cur.ptr = self.cur.ptr.offset(self.cur.bytes as isize);
        }
        self.next_done = false;
    }

    /// Gets current character
    pub fn current(&self) -> &MbChar {
        &self.cur
    }

    /// Gets pointer to current character
    pub fn current_ptr(&self) -> *const c_char {
        self.cur.ptr
    }
}

// FFI types and functions
type wchar_t = u32; // Assuming 32-bit wide chars
type mbstate_t = [u8; 128]; // Size varies by platform
type size_t = usize;

extern "C" {
    fn mbrtowc(
        pwc: *mut wchar_t,
        s: *const c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn mbsinit(ps: *const mbstate_t) -> i32;
    fn MB_CUR_MAX() -> size_t;
}

#[inline]
unsafe fn is_basic(c: c_char) -> bool {
    (c as u8).is_ascii()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_iteration() {
        let s = "hello\0";
        let mut iter = MbuIterMulti::new(s.as_ptr() as *const c_char);
        
        let mut count = 0;
        while iter.avail() {
            let ch = iter.current();
            assert!(ch.wc_valid);
            assert_eq!(ch.bytes, 1);
            count += 1;
            iter.advance();
        }
        assert_eq!(count, 5);
    }
}