/*!
Iterating through multibyte strings in Rust.

This module provides safe iteration over multibyte strings without requiring
prior knowledge of the string's length.
*/

use std::mem;
use std::str;
use std::char;
use std::convert::TryInto;

/// Represents a multibyte character with its metadata.
#[derive(Debug, Clone)]
pub struct MbChar {
    pub ptr: *const u8,
    pub bytes: usize,
    pub wc_valid: bool,
    pub wc: char,
}

impl MbChar {
    /// Creates a new MbChar with default values.
    pub fn new() -> Self {
        MbChar {
            ptr: std::ptr::null(),
            bytes: 0,
            wc_valid: false,
            wc: '\0',
        }
    }

    /// Checks if the character is null.
    pub fn is_null(&self) -> bool {
        unsafe { *self.ptr == 0 }
    }
}

/// Iterator for multibyte strings.
#[derive(Debug, Clone)]
pub struct MbuIter {
    in_shift: bool,
    state: std::str::Chars,
    next_done: bool,
    cur: MbChar,
}

impl MbuIter {
    /// Creates a new iterator starting at the given pointer.
    pub fn new(start: *const u8) -> Self {
        unsafe {
            let s = std::ffi::CStr::from_ptr(start as *const i8).to_str().unwrap();
            MbuIter {
                in_shift: false,
                state: s.chars(),
                next_done: false,
                cur: MbChar {
                    ptr: start,
                    bytes: 0,
                    wc_valid: false,
                    wc: '\0',
                },
            }
        }
    }

    /// Advances the iterator to the next multibyte character.
    pub fn next(&mut self) -> Option<MbChar> {
        if self.next_done {
            self.next_done = false;
            return Some(self.cur.clone());
        }

        if self.in_shift {
            return self.next_with_shift();
        }

        // Handle ASCII quickly
        unsafe {
            if (*self.cur.ptr).is_ascii() {
                self.cur.bytes = 1;
                self.cur.wc = *self.cur.ptr as char;
                self.cur.wc_valid = true;
                self.next_done = true;
                return Some(self.cur.clone());
            }
        }

        self.in_shift = true;
        self.next_with_shift()
    }

    fn next_with_shift(&mut self) -> Option<MbChar> {
        match self.state.next() {
            Some(c) => {
                let bytes = c.len_utf8();
                self.cur.bytes = bytes;
                self.cur.wc = c;
                self.cur.wc_valid = true;
                self.next_done = true;
                
                // Update pointer (simplified - actual implementation would need more work)
                unsafe {
                    self.cur.ptr = self.cur.ptr.add(bytes);
                }
                
                Some(self.cur.clone())
            }
            None => None,
        }
    }

    /// Checks if there are more characters available.
    pub fn avail(&mut self) -> bool {
        self.next().is_some()
    }

    /// Advances the iterator by one character.
    pub fn advance(&mut self) {
        self.next();
    }

    /// Gets the current character.
    pub fn current(&self) -> MbChar {
        self.cur.clone()
    }

    /// Gets a pointer to the current character.
    pub fn current_ptr(&self) -> *const u8 {
        self.cur.ptr
    }

    /// Relocates the iterator by the given offset.
    pub fn relocate(&mut self, offset: isize) {
        unsafe {
            self.cur.ptr = self.cur.ptr.offset(offset);
        }
    }
}

/// Initializes a new iterator.
pub fn mbui_init(iter: &mut MbuIter, start: *const u8) {
    *iter = MbuIter::new(start);
}

/// Checks if there are more characters available.
pub fn mbui_avail(iter: &mut MbuIter) -> bool {
    iter.avail()
}

/// Advances the iterator by one character.
pub fn mbui_advance(iter: &mut MbuIter) {
    iter.advance();
}

/// Gets the current character.
pub fn mbui_cur(iter: &MbuIter) -> MbChar {
    iter.current()
}

/// Gets a pointer to the current character.
pub fn mbui_cur_ptr(iter: &MbuIter) -> *const u8 {
    iter.current_ptr()
}

/// Relocates the iterator.
pub fn mbui_reloc(iter: &mut MbuIter, offset: isize) {
    iter.relocate(offset);
}

/// Copies the iterator.
pub fn mbui_copy(dest: &mut MbuIter, src: &MbuIter) {
    *dest = src.clone();
}