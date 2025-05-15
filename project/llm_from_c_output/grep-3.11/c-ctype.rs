//! Character handling in C locale.
//!
//! These functions work like the corresponding functions in <ctype.h>,
//! except that they have the C (POSIX) locale hardwired.

#![allow(non_snake_case)]

/// Checks if the character is alphanumeric (a-z, A-Z, 0-9).
pub fn c_isalnum(c: char) -> bool {
    match c {
        '0'..='9' | 'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
}

/// Checks if the character is alphabetic (a-z, A-Z).
pub fn c_isalpha(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' => true,
        _ => false,
    }
}

/// Checks if the character is ASCII (0-127).
pub fn c_isascii(c: char) -> bool {
    c.is_ascii()
}

/// Checks if the character is blank (space or tab).
pub fn c_isblank(c: char) -> bool {
    c == ' ' || c == '\t'
}

/// Checks if the character is a control character.
pub fn c_iscntrl(c: char) -> bool {
    match c {
        '\x00'..='\x1F' | '\x7F' => true,
        _ => false,
    }
}

/// Checks if the character is a digit (0-9).
pub fn c_isdigit(c: char) -> bool {
    c.is_ascii_digit()
}

/// Checks if the character has graphical representation.
pub fn c_isgraph(c: char) -> bool {
    match c {
        '0'..='9' | 'a'..='z' | 'A'..='Z' | 
        '!'..='/' | ':'..='@' | '['..='`' | '{'..='~' => true,
        _ => false,
    }
}

/// Checks if the character is lowercase (a-z).
pub fn c_islower(c: char) -> bool {
    c.is_ascii_lowercase()
}

/// Checks if the character is printable (including space).
pub fn c_isprint(c: char) -> bool {
    match c {
        ' '..='~' => true,
        _ => false,
    }
}

/// Checks if the character is punctuation.
pub fn c_ispunct(c: char) -> bool {
    match c {
        '!'..='/' | ':'..='@' | '['..='`' | '{'..='~' => true,
        _ => false,
    }
}

/// Checks if the character is whitespace.
pub fn c_isspace(c: char) -> bool {
    match c {
        ' ' | '\t' | '\n' | '\r' | '\x0B' | '\x0C' => true,
        _ => false,
    }
}

/// Checks if the character is uppercase (A-Z).
pub fn c_isupper(c: char) -> bool {
    c.is_ascii_uppercase()
}

/// Checks if the character is a hexadecimal digit (0-9, a-f, A-F).
pub fn c_isxdigit(c: char) -> bool {
    match c {
        '0'..='9' | 'a'..='f' | 'A'..='F' => true,
        _ => false,
    }
}

/// Converts the character to lowercase.
pub fn c_tolower(c: char) -> char {
    c.to_ascii_lowercase()
}

/// Converts the character to uppercase.
pub fn c_toupper(c: char) -> char {
    c.to_ascii_uppercase()
}