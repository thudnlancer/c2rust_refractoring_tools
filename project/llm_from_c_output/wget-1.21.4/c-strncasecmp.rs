// c-strncasecmp.rs -- case insensitive string comparator in C locale
// Copyright (C) 1998-1999, 2005-2006, 2009-2023 Free Software Foundation, Inc.
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
use std::convert::TryInto;

/// Case insensitive string comparison for C locale
pub fn c_strncasecmp(s1: &str, s2: &str, n: usize) -> i32 {
    if s1.as_ptr() == s2.as_ptr() || n == 0 {
        return 0;
    }

    let mut p1 = s1.chars();
    let mut p2 = s2.chars();
    let mut remaining = n;
    
    loop {
        let c1 = p1.next().map(c_tolower);
        let c2 = p2.next().map(c_tolower);
        
        match (c1, c2) {
            (None, None) => return 0,
            (None, _) => return -1,
            (_, None) => return 1,
            (Some(c1), Some(c2)) => {
                remaining -= 1;
                if remaining == 0 || c1 == '\0' {
                    return compare_chars(c1, c2);
                }
                if c1 != c2 {
                    return compare_chars(c1, c2);
                }
            }
        }
    }
}

fn compare_chars(c1: char, c2: char) -> i32 {
    if c1 == c2 {
        0
    } else if c1 < c2 {
        -1
    } else {
        1
    }
}

fn c_tolower(c: char) -> char {
    if c.is_ascii() {
        c.to_ascii_lowercase()
    } else {
        c
    }
}