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

/// Case-insensitive string comparison for ASCII characters only (C locale)
/// Compares up to `n` bytes of strings `s1` and `s2`
pub fn c_strncasecmp(s1: &str, s2: &str, n: usize) -> i32 {
    if s1.as_ptr() == s2.as_ptr() || n == 0 {
        return 0;
    }

    let mut p1 = s1.bytes();
    let mut p2 = s2.bytes();
    let mut remaining = n;
    
    loop {
        let c1 = p1.next().map_or(b'\0', |c| c.to_ascii_lowercase());
        let c2 = p2.next().map_or(b'\0', |c| c.to_ascii_lowercase());
        
        remaining -= 1;
        if remaining == 0 || c1 == b'\0' {
            break;
        }
        
        if c1 != c2 {
            return if c1 < c2 { -1 } else { 1 };
        }
    }
    
    let c1 = p1.next().map_or(b'\0', |c| c.to_ascii_lowercase());
    let c2 = p2.next().map_or(b'\0', |c| c.to_ascii_lowercase());
    
    match c1.cmp(&c2) {
        Ordering::Equal => 0,
        Ordering::Less => -1,
        Ordering::Greater => 1,
    }
}