/* Case-insensitive string comparison function.
   Copyright (C) 1998-1999, 2005-2023 Free Software Foundation, Inc.
   Written by Bruno Haible <bruno@clisp.org>, 2005,
   based on earlier glibc code.

   This file is free software: you can redistribute it and/or modify
   it under the terms of the GNU Lesser General Public License as
   published by the Free Software Foundation, either version 3 of the
   License, or (at your option) any later version.

   This file is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU Lesser General Public License for more details.

   You should have received a copy of the GNU Lesser General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

use std::cmp::Ordering;
use std::char;
use std::str::Chars;

/// Compare the character strings s1 and s2, ignoring case, returning less than,
/// equal to or greater than zero if s1 is lexicographically less than, equal to
/// or greater than s2.
/// Note: This function may, in multibyte locales, return 0 for strings of
/// different lengths!
pub fn mbscasecmp(s1: &str, s2: &str) -> i32 {
    if s1.as_ptr() == s2.as_ptr() {
        return 0;
    }

    // For simplicity, we'll use Rust's built-in Unicode case folding
    // This handles both single-byte and multi-byte cases correctly
    let mut iter1 = s1.chars().flat_map(|c| c.to_lowercase());
    let mut iter2 = s2.chars().flat_map(|c| c.to_lowercase());

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(c1), Some(c2)) => {
                let cmp = c1.cmp(&c2);
                if cmp != Ordering::Equal {
                    return cmp as i32;
                }
            }
            (Some(_), None) => return 1,    // s2 terminated before s1
            (None, Some(_)) => return -1,   // s1 terminated before s2
            (None, None) => return 0,       // both terminated at same time
        }
    }
}

// Helper function to convert Ordering to i32
trait OrderingExt {
    fn as_i32(self) -> i32;
}

impl OrderingExt for Ordering {
    fn as_i32(self) -> i32 {
        match self {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}