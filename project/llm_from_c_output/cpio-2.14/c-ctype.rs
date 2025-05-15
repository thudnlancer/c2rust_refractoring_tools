//! Character handling in C locale.
//!
//! These functions work like the corresponding functions in <ctype.h>,
//! except that they have the C (POSIX) locale hardwired, whereas the
//! <ctype.h> functions' behaviour depends on the current locale set via
//! setlocale.
//!
//! Copyright (C) 2000-2003, 2006, 2008-2023 Free Software Foundation, Inc.
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

/// Check if a character is alphanumeric (A-Z, a-z, 0-9)
pub fn c_isalnum(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

/// Check if a character is alphabetic (A-Z, a-z)
pub fn c_isalpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

/// Check if a character is ASCII (0-127)
pub fn c_isascii(c: char) -> bool {
    c.is_ascii()
}

/// Check if a character is blank (space or tab)
pub fn c_isblank(c: char) -> bool {
    c == ' ' || c == '\t'
}

/// Check if a character is a control character
pub fn c_iscntrl(c: char) -> bool {
    c.is_ascii_control()
}

/// Check if a character is a digit (0-9)
pub fn c_isdigit(c: char) -> bool {
    c.is_ascii_digit()
}

/// Check if a character is graphical (has a visible representation)
pub fn c_isgraph(c: char) -> bool {
    c.is_ascii_graphic()
}

/// Check if a character is lowercase (a-z)
pub fn c_islower(c: char) -> bool {
    c.is_ascii_lowercase()
}

/// Check if a character is printable (includes space)
pub fn c_isprint(c: char) -> bool {
    c.is_ascii() && !c.is_ascii_control()
}

/// Check if a character is punctuation
pub fn c_ispunct(c: char) -> bool {
    c.is_ascii_punctuation()
}

/// Check if a character is whitespace
pub fn c_isspace(c: char) -> bool {
    c.is_ascii_whitespace()
}

/// Check if a character is uppercase (A-Z)
pub fn c_isupper(c: char) -> bool {
    c.is_ascii_uppercase()
}

/// Check if a character is a hexadecimal digit (0-9, A-F, a-f)
pub fn c_isxdigit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

/// Convert character to lowercase (A-Z → a-z)
pub fn c_tolower(c: char) -> char {
    c.to_ascii_lowercase()
}

/// Convert character to uppercase (a-z → A-Z)
pub fn c_toupper(c: char) -> char {
    c.to_ascii_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alnum() {
        assert!(c_isalnum('a'));
        assert!(c_isalnum('Z'));
        assert!(c_isalnum('0'));
        assert!(!c_isalnum('!'));
    }

    #[test]
    fn test_alpha() {
        assert!(c_isalpha('a'));
        assert!(c_isalpha('Z'));
        assert!(!c_isalpha('0'));
    }

    #[test]
    fn test_ascii() {
        assert!(c_isascii('a'));
        assert!(c_isascii('\x7f'));
        assert!(!c_isascii('ü'));
    }

    #[test]
    fn test_blank() {
        assert!(c_isblank(' '));
        assert!(c_isblank('\t'));
        assert!(!c_isblank('a'));
    }

    #[test]
    fn test_cntrl() {
        assert!(c_iscntrl('\n'));
        assert!(c_iscntrl('\x1f'));
        assert!(!c_iscntrl('a'));
    }

    #[test]
    fn test_digit() {
        assert!(c_isdigit('0'));
        assert!(c_isdigit('9'));
        assert!(!c_isdigit('a'));
    }

    #[test]
    fn test_graph() {
        assert!(c_isgraph('a'));
        assert!(c_isgraph('!'));
        assert!(!c_isgraph(' '));
    }

    #[test]
    fn test_lower() {
        assert!(c_islower('a'));
        assert!(!c_islower('A'));
    }

    #[test]
    fn test_print() {
        assert!(c_isprint('a'));
        assert!(c_isprint(' '));
        assert!(!c_isprint('\n'));
    }

    #[test]
    fn test_punct() {
        assert!(c_ispunct('!'));
        assert!(!c_ispunct('a'));
    }

    #[test]
    fn test_space() {
        assert!(c_isspace(' '));
        assert!(c_isspace('\n'));
        assert!(!c_isspace('a'));
    }

    #[test]
    fn test_upper() {
        assert!(c_isupper('A'));
        assert!(!c_isupper('a'));
    }

    #[test]
    fn test_xdigit() {
        assert!(c_isxdigit('a'));
        assert!(c_isxdigit('F'));
        assert!(c_isxdigit('0'));
        assert!(!c_isxdigit('g'));
    }

    #[test]
    fn test_case_conversion() {
        assert_eq!(c_tolower('A'), 'a');
        assert_eq!(c_toupper('a'), 'A');
        assert_eq!(c_tolower('1'), '1');
    }
}