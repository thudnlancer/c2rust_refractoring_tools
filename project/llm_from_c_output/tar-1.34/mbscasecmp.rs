// Case-insensitive string comparison function.
// Copyright (C) 1998-1999, 2005-2021 Free Software Foundation, Inc.
// Written by Bruno Haible <bruno@clisp.org>, 2005,
// based on earlier glibc code.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::cmp::Ordering;
use std::char;
use unicode_normalization::UnicodeNormalization;

/// Compare the character strings s1 and s2, ignoring case, returning less than,
/// equal to or greater than zero if s1 is lexicographically less than, equal to
/// or greater than s2.
/// Note: This function may, in multibyte locales, return 0 for strings of
/// different lengths!
pub fn mbscasecmp(s1: &str, s2: &str) -> Ordering {
    if s1.as_ptr() == s2.as_ptr() {
        return Ordering::Equal;
    }

    // For simplicity, we'll handle both single-byte and multi-byte cases
    // using Rust's built-in Unicode handling. This differs from the C version
    // which has special handling for MB_CUR_MAX > 1.

    let mut iter1 = s1.chars().flat_map(|c| c.to_lowercase());
    let mut iter2 = s2.chars().flat_map(|c| c.to_lowercase());

    loop {
        match (iter1.next(), iter2.next()) {
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(c1), Some(c2)) => {
                let cmp = c1.cmp(&c2);
                if cmp != Ordering::Equal {
                    return cmp;
                }
            }
        }
    }
}