//! Iterating through multibyte strings: iterator for multi-byte encodings.
//!
//! This module provides an iterator for safely traversing multibyte strings
//! with proper handling of invalid sequences while preserving them.

use std::mem;
use std::str;
use std::char;
use std::convert::TryFrom;

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
#[derive(Debug)]
pub struct MbIterMulti {
    limit: *const u8,
    in_shift: bool,
    state: std::str::Chars,
    next_done: bool,
    cur: MbChar,
}

impl MbIterMulti {
    /// Initializes a new iterator
    pub fn new(startptr: *const u8, length: usize) -> Self {
        let slice = unsafe { std::slice::from_raw_parts(startptr, length) };
        let s = unsafe { std::str::from_utf8_unchecked(slice) };
        
        MbIterMulti {
            limit: unsafe { startptr.add(length) },
            in_shift: false,
            state: s.chars(),
            next_done: false,
            cur: MbChar::new(),
        }
    }

    /// Checks if there are more characters available
    pub fn avail(&mut self) -> bool {
        if self.cur.ptr >= self.limit {
            return false;
        }
        if !self.next_done {
            self.next();
        }
        true
    }

    /// Advances the iterator by one character
    pub fn advance(&mut self) {
        if self.cur.ptr < self.limit {
            unsafe {
                self.cur.ptr = self.cur.ptr.add(self.cur.bytes);
            }
            self.next_done = false;
        }
    }

    /// Gets the current character
    pub fn current(&self) -> &MbChar {
        &self.cur
    }

    /// Gets a pointer to the current character
    pub fn current_ptr(&self) -> *const u8 {
        self.cur.ptr
    }

    /// Relocates the iterator
    pub fn reloc(&mut self, ptrdiff: isize) {
        unsafe {
            self.cur.ptr = self.cur.ptr.offset(ptrdiff);
            self.limit = self.limit.offset(ptrdiff);
        }
    }

    /// Processes the next character
    fn next(&mut self) {
        if self.next_done {
            return;
        }

        self.cur.ptr = unsafe { self.limit.sub(1) }; // Temporary, will be updated
        self.cur.wc_valid = false;

        if self.in_shift {
            self.process_shift();
        } else {
            // Handle ASCII quickly
            let c = unsafe { *self.cur.ptr };
            if c.is_ascii() {
                self.cur.bytes = 1;
                self.cur.wc = c as char;
                self.cur.wc_valid = true;
            } else {
                self.in_shift = true;
                self.process_shift();
            }
        }
        self.next_done = true;
    }

    /// Processes characters in shift state
    fn process_shift(&mut self) {
        let remaining = unsafe { self.limit.offset_from(self.cur.ptr) } as usize;
        let slice = unsafe { std::slice::from_raw_parts(self.cur.ptr, remaining) };

        match std::str::from_utf8(slice) {
            Ok(s) => {
                if let Some((c, len)) = s.chars().next().map(|c| (c, c.len_utf8())) {
                    self.cur.bytes = len;
                    self.cur.wc = c;
                    self.cur.wc_valid = true;
                    
                    if len == 1 && c.is_ascii() {
                        self.in_shift = false;
                    }
                } else {
                    self.cur.bytes = 1;
                }
            }
            Err(e) => {
                // Handle invalid UTF-8
                self.cur.bytes = 1;
                self.cur.wc_valid = false;
            }
        }
    }
}

/// Creates a new MbIterMulti iterator
pub fn mbi_init(iter: &mut MbIterMulti, startptr: *const u8, length: usize) {
    *iter = MbIterMulti::new(startptr, length);
}

/// Checks if iterator has more characters
pub fn mbi_avail(iter: &mut MbIterMulti) -> bool {
    iter.avail()
}

/// Advances the iterator
pub fn mbi_advance(iter: &mut MbIterMulti) {
    iter.advance();
}

/// Gets current character
pub fn mbi_cur(iter: &MbIterMulti) -> &MbChar {
    iter.current()
}

/// Gets pointer to current character
pub fn mbi_cur_ptr(iter: &MbIterMulti) -> *const u8 {
    iter.current_ptr()
}

/// Relocates the iterator
pub fn mbi_reloc(iter: &mut MbIterMulti, ptrdiff: isize) {
    iter.reloc(ptrdiff);
}

/// Copies the iterator
pub fn mbi_copy(dest: &mut MbIterMulti, src: &MbIterMulti) {
    *dest = src.clone();
}