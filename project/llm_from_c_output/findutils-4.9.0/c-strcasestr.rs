/*
 * Case-insensitive searching in a string in C locale.
 * Translated from C to Rust while maintaining original functionality.
 */

use std::cmp;
use std::ascii;

/// Find the first occurrence of `needle` in `haystack`, using case-insensitive
/// comparison from the C locale.
pub fn c_strcasestr(haystack: &str, needle: &str) -> Option<&str> {
    if needle.is_empty() {
        return Some(haystack);
    }

    let needle_len = needle.len();
    let haystack_len = haystack.len();

    if haystack_len < needle_len {
        return None;
    }

    let needle_bytes = needle.as_bytes();
    let haystack_bytes = haystack.as_bytes();

    // Convert first character of needle to lowercase for comparison
    let first_char_lower = needle_bytes[0].to_ascii_lowercase();

    for i in 0..=(haystack_len - needle_len) {
        // Check if first character matches (case-insensitive)
        if haystack_bytes[i].to_ascii_lowercase() != first_char_lower {
            continue;
        }

        // Check remaining characters
        let mut matches = true;
        for j in 1..needle_len {
            if haystack_bytes[i + j].to_ascii_lowercase() != needle_bytes[j].to_ascii_lowercase() {
                matches = false;
                break;
            }
        }

        if matches {
            return Some(&haystack[i..]);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_strcasestr() {
        assert_eq!(c_strcasestr("Hello World", "world"), Some("World"));
        assert_eq!(c_strcasestr("Hello World", "WORLD"), Some("World"));
        assert_eq!(c_strcasestr("Hello World", "ello"), Some("ello World"));
        assert_eq!(c_strcasestr("Hello World", "foo"), None);
        assert_eq!(c_strcasestr("Hello World", ""), Some("Hello World"));
        assert_eq!(c_strcasestr("", ""), Some(""));
        assert_eq!(c_strcasestr("", "hello"), None);
    }
}