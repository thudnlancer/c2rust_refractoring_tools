//! Character handling in C locale.
//!
//! These functions work like the corresponding functions in Rust's `char` methods,
//! except that they have the C (POSIX) locale hardwired, whereas Rust's `char` methods
//! may depend on the current locale.

#![allow(non_snake_case)]

/// Checks if a character is alphanumeric (a letter or a digit).
pub fn c_isalnum(c: char) -> bool {
    c.is_ascii_alphanumeric()
}

/// Checks if a character is alphabetic (a letter).
pub fn c_isalpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

/// Checks if a character is ASCII (in the range 0x00..=0x7F).
pub fn c_isascii(c: char) -> bool {
    c.is_ascii()
}

/// Checks if a character is a blank character (space or horizontal tab).
pub fn c_isblank(c: char) -> bool {
    c == ' ' || c == '\t'
}

/// Checks if a character is a control character.
pub fn c_iscntrl(c: char) -> bool {
    c.is_ascii_control()
}

/// Checks if a character is a digit (0-9).
pub fn c_isdigit(c: char) -> bool {
    c.is_ascii_digit()
}

/// Checks if a character has a graphical representation.
pub fn c_isgraph(c: char) -> bool {
    c.is_ascii_graphic()
}

/// Checks if a character is lowercase.
pub fn c_islower(c: char) -> bool {
    c.is_ascii_lowercase()
}

/// Checks if a character is printable (including space).
pub fn c_isprint(c: char) -> bool {
    c.is_ascii() && (c.is_ascii_graphic() || c == ' ')
}

/// Checks if a character is punctuation.
pub fn c_ispunct(c: char) -> bool {
    c.is_ascii_punctuation()
}

/// Checks if a character is whitespace.
pub fn c_isspace(c: char) -> bool {
    c.is_ascii_whitespace()
}

/// Checks if a character is uppercase.
pub fn c_isupper(c: char) -> bool {
    c.is_ascii_uppercase()
}

/// Checks if a character is a hexadecimal digit (0-9, a-f, A-F).
pub fn c_isxdigit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

/// Converts a character to lowercase.
pub fn c_tolower(c: char) -> char {
    c.to_ascii_lowercase()
}

/// Converts a character to uppercase.
pub fn c_toupper(c: char) -> char {
    c.to_ascii_uppercase()
}