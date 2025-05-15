/*!
Iterating through multibyte strings in Rust.

This module provides safe iteration over multibyte strings without requiring
prior knowledge of the string's length.

The implementation follows similar functionality to the original C code but
uses Rust's standard library and safety features.
*/

use std::str;
use std::mem;
use std::fmt;
use std::char;
use std::convert::TryFrom;

/// Represents a multibyte character with its properties
#[derive(Debug, Clone)]
pub struct MbChar {
    pub ptr: *const u8,
    pub bytes: usize,
    pub wc_valid: bool,
    pub wc: char,
}

impl MbChar {
    /// Creates a new MbChar
    pub fn new() -> Self {
        MbChar {
            ptr: std::ptr::null(),
            bytes: 0,
            wc_valid: false,
            wc: '\0',
        }
    }

    /// Checks if this is a null character
    pub fn is_null(&self) -> bool {
        unsafe { *self.ptr == 0 }
    }
}

/// Iterator for multibyte strings
pub struct MbuIter {
    in_shift: bool,
    state: std::str::Chars,
    next_done: bool,
    current: MbChar,
    source: String,
}

impl MbuIter {
    /// Creates a new iterator starting at the given string
    pub fn new(start: &str) -> Self {
        MbuIter {
            in_shift: false,
            state: start.chars(),
            next_done: false,
            current: MbChar::new(),
            source: start.to_string(),
        }
    }

    /// Advances the iterator to the next character
    pub fn next(&mut self) -> Option<MbChar> {
        if self.next_done {
            self.next_done = false;
            return Some(self.current.clone());
        }

        if self.in_shift {
            return self.next_with_shift();
        }

        // Handle ASCII quickly
        if let Some(c) = self.state.next() {
            if c.is_ascii() {
                self.current.bytes = 1;
                self.current.wc = c;
                self.current.wc_valid = true;
                self.next_done = true;
                return Some(self.current.clone());
            } else {
                self.in_shift = true;
                return self.next_with_shift();
            }
        }

        None
    }

    fn next_with_shift(&mut self) -> Option<MbChar> {
        if let Some(c) = self.state.next() {
            let bytes = c.len_utf8();
            self.current.bytes = bytes;
            self.current.wc = c;
            self.current.wc_valid = true;

            // If we're back to initial state, we can optimize ASCII again
            if c.is_ascii() {
                self.in_shift = false;
            }

            self.next_done = true;
            Some(self.current.clone())
        } else {
            None
        }
    }

    /// Returns true if there are more characters available
    pub fn avail(&mut self) -> bool {
        self.next().is_some()
    }

    /// Advances the iterator by one character
    pub fn advance(&mut self) {
        self.next();
    }

    /// Returns the current character
    pub fn current(&self) -> &MbChar {
        &self.current
    }

    /// Returns a pointer to the current character
    pub fn current_ptr(&self) -> *const u8 {
        self.current.ptr
    }

    /// Relocates the iterator when the string is moved
    pub fn relocate(&mut self, diff: isize) {
        unsafe {
            self.current.ptr = self.current.ptr.offset(diff);
        }
    }
}

impl Iterator for MbuIter {
    type Item = MbChar;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_iteration() {
        let s = "hello";
        let mut iter = MbuIter::new(s);
        assert!(iter.avail());
        assert_eq!(iter.current().wc, 'h');
        iter.advance();
        assert_eq!(iter.current().wc, 'e');
    }

    #[test]
    fn test_unicode_iteration() {
        let s = "こんにちは";
        let mut iter = MbuIter::new(s);
        assert!(iter.avail());
        assert_eq!(iter.current().wc, 'こ');
        iter.advance();
        assert_eq!(iter.current().wc, 'ん');
    }
}