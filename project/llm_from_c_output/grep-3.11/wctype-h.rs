//! Inline functions for character classification and conversion.
//!
//! This module provides functionality equivalent to the C <wctype.h> header,
//! implemented in safe Rust using standard library facilities.
//!
//! Copyright (C) 2012-2023 Free Software Foundation, Inc.
//!
//! This file is free software: you can redistribute it and/or modify
//! it under the terms of the GNU Lesser General Public License as
//! published by the Free Software Foundation; either version 2.1 of the
//! License, or (at your option) any later version.
//!
//! This file is distributed in the hope that it will be useful,
//! but WITHOUT ANY WARRANTY; without even the implied warranty of
//! MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//! GNU Lesser General Public License for more details.
//!
//! You should have received a copy of the GNU Lesser General Public License
//! along with this program.  If not, see <https://www.gnu.org/licenses/>.

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::char;

/// Character classification and conversion functions equivalent to <wctype.h>
pub mod wctype {
    /// Checks if a character is alphanumeric.
    pub fn isalnum(c: char) -> bool {
        c.is_alphanumeric()
    }

    /// Checks if a character is alphabetic.
    pub fn isalpha(c: char) -> bool {
        c.is_alphabetic()
    }

    /// Checks if a character is blank (space or horizontal tab).
    pub fn isblank(c: char) -> bool {
        c == ' ' || c == '\t'
    }

    /// Checks if a character is a control character.
    pub fn iscntrl(c: char) -> bool {
        c.is_control()
    }

    /// Checks if a character is a digit.
    pub fn isdigit(c: char) -> bool {
        c.is_digit(10)
    }

    /// Checks if a character is a graphic character (except space).
    pub fn isgraph(c: char) -> bool {
        c != ' ' && c.is_ascii_graphic()
    }

    /// Checks if a character is lowercase.
    pub fn islower(c: char) -> bool {
        c.is_lowercase()
    }

    /// Checks if a character is printable (including space).
    pub fn isprint(c: char) -> bool {
        c.is_ascii_graphic() || c == ' '
    }

    /// Checks if a character is punctuation.
    pub fn ispunct(c: char) -> bool {
        c.is_ascii_punctuation()
    }

    /// Checks if a character is whitespace.
    pub fn isspace(c: char) -> bool {
        c.is_whitespace()
    }

    /// Checks if a character is uppercase.
    pub fn isupper(c: char) -> bool {
        c.is_uppercase()
    }

    /// Checks if a character is a hexadecimal digit.
    pub fn isxdigit(c: char) -> bool {
        c.is_digit(16)
    }

    /// Converts a character to lowercase.
    pub fn tolower(c: char) -> char {
        c.to_lowercase().next().unwrap_or(c)
    }

    /// Converts a character to uppercase.
    pub fn toupper(c: char) -> char {
        c.to_uppercase().next().unwrap_or(c)
    }
}