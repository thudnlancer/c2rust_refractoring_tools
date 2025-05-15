// c-strncasecmp.rs -- case insensitive string comparator in C locale
// Copyright (C) 1998-1999, 2005-2006, 2009-2022 Free Software Foundation, Inc.
//
// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.
//
// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::cmp::Ordering;

/// Case-insensitive string comparison for C locale
/// Compares at most `n` bytes of `s1` and `s2` ignoring case
pub fn c_strncasecmp(s1: &str, s2: &str, n: usize) -> i32 {
    if s1.as_ptr() == s2.as_ptr() || n == 0 {
        return 0;
    }

    let mut p1 = s1.chars();
    let mut p2 = s2.chars();
    let mut remaining = n;
    
    loop {
        let c1 = p1.next().map(|c| c.to_ascii_lowercase());
        let c2 = p2.next().map(|c| c.to_ascii_lowercase());
        
        remaining = remaining.saturating_sub(1);
        
        match (c1, c2) {
            (Some(c1), Some(c2)) => {
                if remaining == 0 || c1 == '\0' {
                    break;
                }
                if c1 != c2 {
                    return (c1 as i32) - (c2 as i32);
                }
            }
            (None, None) => return 0,
            (None, _) => return -1,
            (_, None) => return 1,
        }
    }
    
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal() {
        assert_eq!(c_strncasecmp("hello", "HELLO", 5), 0);
    }

    #[test]
    fn test_different() {
        assert!(c_strncasecmp("hello", "world", 5) < 0);
    }

    #[test]
    fn test_partial() {
        assert_eq!(c_strncasecmp("hello", "HELL", 4), 0);
    }

    #[test]
    fn test_zero_length() {
        assert_eq!(c_strncasecmp("hello", "world", 0), 0);
    }

    #[test]
    fn test_same_pointer() {
        let s = "test";
        assert_eq!(c_strncasecmp(s, s, 4), 0);
    }
}