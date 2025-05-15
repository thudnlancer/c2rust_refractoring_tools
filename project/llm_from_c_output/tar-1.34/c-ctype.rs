//! Character handling in C locale (Rust implementation).
//!
//! These functions work like the corresponding functions in Rust's `char` methods,
//! except that they have the C (POSIX) locale hardwired, whereas the Rust `char`
//! methods' behavior depends on the Unicode properties.

#![allow(non_snake_case)]

/// Checks if a character is alphanumeric (ASCII only).
pub fn c_isalnum(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

/// Checks if a character is alphabetic (ASCII only).
pub fn c_isalpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

/// Checks if a character is ASCII (0-127).
pub fn c_isascii(c: char) -> bool {
    c.is_ascii()
}

/// Checks if a character is blank (space or tab).
pub fn c_isblank(c: char) -> bool {
    c == ' ' || c == '\t'
}

/// Checks if a character is a control character (ASCII only).
pub fn c_iscntrl(c: char) -> bool {
    c.is_ascii_control()
}

/// Checks if a character is a digit (ASCII only).
pub fn c_isdigit(c: char) -> bool {
    c.is_ascii_digit()
}

/// Checks if a character is graphical (ASCII only).
pub fn c_isgraph(c: char) -> bool {
    c.is_ascii_graphic()
}

/// Checks if a character is lowercase (ASCII only).
pub fn c_islower(c: char) -> bool {
    c.is_ascii_lowercase()
}

/// Checks if a character is printable (ASCII only).
pub fn c_isprint(c: char) -> bool {
    c.is_ascii() && !c.is_ascii_control()
}

/// Checks if a character is punctuation (ASCII only).
pub fn c_ispunct(c: char) -> bool {
    match c {
        '!'..='/' | ':'..='@' | '['..='`' | '{'..='~' => true,
        _ => false,
    }
}

/// Checks if a character is whitespace (ASCII only).
pub fn c_isspace(c: char) -> bool {
    c.is_ascii_whitespace()
}

/// Checks if a character is uppercase (ASCII only).
pub fn c_isupper(c: char) -> bool {
    c.is_ascii_uppercase()
}

/// Checks if a character is a hexadecimal digit (ASCII only).
pub fn c_isxdigit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

/// Converts a character to lowercase (ASCII only).
pub fn c_tolower(c: char) -> char {
    if c.is_ascii_uppercase() {
        (c as u8 + 32) as char
    } else {
        c
    }
}

/// Converts a character to uppercase (ASCII only).
pub fn c_toupper(c: char) -> char {
    if c.is_ascii_lowercase() {
        (c as u8 - 32) as char
    } else {
        c
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_isalnum() {
        assert!(c_isalnum('a'));
        assert!(c_isalnum('Z'));
        assert!(c_isalnum('0'));
        assert!(!c_isalnum('!'));
    }

    #[test]
    fn test_c_isalpha() {
        assert!(c_isalpha('a'));
        assert!(c_isalpha('Z'));
        assert!(!c_isalpha('0'));
    }

    #[test]
    fn test_c_isascii() {
        assert!(c_isascii('a'));
        assert!(c_isascii('\x7f'));
        assert!(!c_isascii('é'));
    }

    #[test]
    fn test_c_isblank() {
        assert!(c_isblank(' '));
        assert!(c_isblank('\t'));
        assert!(!c_isblank('a'));
    }

    #[test]
    fn test_c_iscntrl() {
        assert!(c_iscntrl('\n'));
        assert!(c_iscntrl('\x1f'));
        assert!(!c_iscntrl('a'));
    }

    #[test]
    fn test_c_isdigit() {
        assert!(c_isdigit('0'));
        assert!(c_isdigit('9'));
        assert!(!c_isdigit('a'));
    }

    #[test]
    fn test_c_isgraph() {
        assert!(c_isgraph('a'));
        assert!(c_isgraph('!'));
        assert!(!c_isgraph(' '));
    }

    #[test]
    fn test_c_islower() {
        assert!(c_islower('a'));
        assert!(!c_islower('A'));
    }

    #[test]
    fn test_c_isprint() {
        assert!(c_isprint('a'));
        assert!(c_isprint(' '));
        assert!(!c_isprint('\n'));
    }

    #[test]
    fn test_c_ispunct() {
        assert!(c_ispunct('!'));
        assert!(c_ispunct('@'));
        assert!(!c_ispunct('a'));
    }

    #[test]
    fn test_c_isspace() {
        assert!(c_isspace(' '));
        assert!(c_isspace('\n'));
        assert!(!c_isspace('a'));
    }

    #[test]
    fn test_c_isupper() {
        assert!(c_isupper('A'));
        assert!(!c_isupper('a'));
    }

    #[test]
    fn test_c_isxdigit() {
        assert!(c_isxdigit('a'));
        assert!(c_isxdigit('F'));
        assert!(c_isxdigit('0'));
        assert!(!c_isxdigit('g'));
    }

    #[test]
    fn test_c_tolower() {
        assert_eq!(c_tolower('A'), 'a');
        assert_eq!(c_tolower('a'), 'a');
    }

    #[test]
    fn test_c_toupper() {
        assert_eq!(c_toupper('a'), 'A');
        assert_eq!(c_toupper('A'), 'A');
    }
}