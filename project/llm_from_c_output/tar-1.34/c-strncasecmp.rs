// rust-c-strncasecmp.rs -- case insensitive string comparator in C locale
// Copyright (C) 1998-1999, 2005-2006, 2009-2021 Free Software Foundation, Inc.
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3, or (at your option)
// any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, see <https://www.gnu.org/licenses/>.

use std::cmp::Ordering;
use std::convert::TryInto;

/// Case-insensitive string comparison for C locale
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
            (None, None) => return 0,
            (None, _) => return -1,
            (_, None) => return 1,
            (Some(ch1), Some(ch2)) => {
                if remaining == 0 || ch1 == '\0' {
                    break;
                }
                if ch1 != ch2 {
                    return (ch1 as i32) - (ch2 as i32);
                }
            }
        }
    }
    
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_strings() {
        assert_eq!(c_strncasecmp("hello", "hello", 5), 0);
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(c_strncasecmp("HeLlO", "hElLo", 5), 0);
    }

    #[test]
    fn test_different_strings() {
        assert!(c_strncasecmp("hello", "world", 5) < 0);
    }

    #[test]
    fn test_partial_comparison() {
        assert_eq!(c_strncasecmp("hello", "heaven", 2), 0);
        assert!(c_strncasecmp("hello", "heaven", 3) > 0);
    }

    #[test]
    fn test_zero_length() {
        assert_eq!(c_strncasecmp("hello", "world", 0), 0);
    }
}