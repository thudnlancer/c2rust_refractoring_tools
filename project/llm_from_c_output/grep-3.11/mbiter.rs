//! Iterating through multibyte strings: iterator for multi-byte encodings.
//!
//! This module provides an iterator for safely traversing multibyte strings
//! with proper handling of invalid sequences while preserving them.

use std::str;
use std::mem;
use std::fmt;
use std::char;

/// Represents a multibyte character with its metadata
#[derive(Debug, Clone)]
pub struct MbChar {
    pub ptr: *const u8,
    pub bytes: usize,
    pub wc_valid: bool,
    pub wc: char,
}

impl MbChar {
    /// Creates a new MbChar with default values
    pub fn new() -> Self {
        MbChar {
            ptr: std::ptr::null(),
            bytes: 0,
            wc_valid: false,
            wc: '\0',
        }
    }

    /// Copies data from another MbChar
    pub fn copy_from(&mut self, other: &MbChar) {
        self.ptr = other.ptr;
        self.bytes = other.bytes;
        self.wc_valid = other.wc_valid;
        self.wc = other.wc;
    }
}

/// Iterator for multibyte strings
pub struct MbIterMulti {
    limit: *const u8,
    in_shift: bool,
    state: std::str::Chars,
    next_done: bool,
    cur: MbChar,
    original_str: String,
}

impl MbIterMulti {
    /// Creates a new iterator for the given string
    pub fn new(s: &str) -> Self {
        let original_str = s.to_string();
        let chars_iter = original_str.chars();
        
        MbIterMulti {
            limit: unsafe { s.as_ptr().add(s.len()) },
            in_shift: false,
            state: chars_iter,
            next_done: false,
            cur: MbChar::new(),
            original_str,
        }
    }

    /// Advances the iterator to the next character
    pub fn next(&mut self) -> Option<&MbChar> {
        if self.next_done {
            return Some(&self.cur);
        }

        if self.in_shift {
            return self.next_with_shift();
        }

        // Handle ASCII quickly
        if let Some(c) = self.state.next() {
            if c.is_ascii() {
                self.cur.bytes = 1;
                self.cur.wc = c;
                self.cur.wc_valid = true;
                self.next_done = true;
                return Some(&self.cur);
            } else {
                self.in_shift = true;
                return self.next_with_shift();
            }
        }

        None
    }

    fn next_with_shift(&mut self) -> Option<&MbChar> {
        if let Some(c) = self.state.next() {
            let bytes = c.len_utf8();
            self.cur.bytes = bytes;
            self.cur.wc = c;
            self.cur.wc_valid = true;

            if c == '\0' {
                self.cur.bytes = 1;
            }

            // If we're back to ASCII, optimize
            if c.is_ascii() {
                self.in_shift = false;
            }

            self.next_done = true;
            Some(&self.cur)
        } else {
            None
        }
    }

    /// Checks if there are more characters available
    pub fn avail(&mut self) -> bool {
        self.next().is_some()
    }

    /// Advances the iterator by one character
    pub fn advance(&mut self) {
        if let Some(cur) = self.next() {
            unsafe {
                self.cur.ptr = self.cur.ptr.add(cur.bytes);
            }
            self.next_done = false;
        }
    }

    /// Returns the current character
    pub fn current(&self) -> &MbChar {
        &self.cur
    }

    /// Returns a pointer to the current character
    pub fn current_ptr(&self) -> *const u8 {
        self.cur.ptr
    }

    /// Relocates the iterator by the given offset
    pub fn relocate(&mut self, offset: isize) {
        unsafe {
            self.cur.ptr = self.cur.ptr.offset(offset);
            self.limit = self.limit.offset(offset);
        }
    }
}

impl Clone for MbIterMulti {
    fn clone(&self) -> Self {
        let mut new = MbIterMulti {
            limit: self.limit,
            in_shift: self.in_shift,
            state: self.original_str.chars(),
            next_done: self.next_done,
            cur: MbChar::new(),
            original_str: self.original_str.clone(),
        };
        new.cur.copy_from(&self.cur);
        new
    }
}

/// Convenience type alias for the iterator
pub type MbIterator = MbIterMulti;

/// Initializes a new iterator
pub fn mb_iter_init(iter: &mut MbIterator, startptr: &str, length: usize) {
    *iter = MbIterator::new(&startptr[..length]);
}

/// Checks if there are more characters available
pub fn mb_iter_avail(iter: &mut MbIterator) -> bool {
    iter.avail()
}

/// Advances the iterator by one character
pub fn mb_iter_advance(iter: &mut MbIterator) {
    iter.advance();
}

/// Returns the current character
pub fn mb_iter_cur(iter: &MbIterator) -> &MbChar {
    iter.current()
}

/// Returns a pointer to the current character
pub fn mb_iter_cur_ptr(iter: &MbIterator) -> *const u8 {
    iter.current_ptr()
}

/// Relocates the iterator by the given offset
pub fn mb_iter_reloc(iter: &mut MbIterator, offset: isize) {
    iter.relocate(offset);
}

/// Copies the iterator
pub fn mb_iter_copy(dest: &mut MbIterator, src: &MbIterator) {
    *dest = src.clone();
}