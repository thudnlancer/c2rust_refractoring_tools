/*
 * Case-insensitive searching in a string in C locale.
 * Translated from C to Rust with equivalent functionality.
 * Original license:
 * 
 * This file is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation; either version 2.1 of the
 * License, or (at your option) any later version.
 *
 * This file is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use std::ascii;

/// Find the first occurrence of `needle` in `haystack`, using case-insensitive
/// comparison from the C locale, regardless of the current locale.
pub fn c_strcasestr(haystack: &str, needle: &str) -> Option<&str> {
    if needle.is_empty() {
        return Some(haystack);
    }

    let needle_len = needle.len();
    let haystack_len = haystack.len();
    
    if haystack_len < needle_len {
        return None;
    }

    let needle_lower: Vec<u8> = needle.bytes().map(|c| ascii::ToAsciiLowercase::to_ascii_lowercase(c)).collect();

    for i in 0..=(haystack_len - needle_len) {
        let window = &haystack[i..i + needle_len];
        let matches = window.bytes()
            .zip(needle_lower.iter())
            .all(|(h, &n)| ascii::ToAsciiLowercase::to_ascii_lowercase(h) == n);
        
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
        assert_eq!(c_strcasestr("Hello World", "foo"), None);
        assert_eq!(c_strcasestr("Hello World", ""), Some("Hello World"));
        assert_eq!(c_strcasestr("", ""), Some(""));
        assert_eq!(c_strcasestr("", "foo"), None);
    }
}