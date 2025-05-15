// c-strcasecmp.rs -- case insensitive string comparator in C locale
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

/// Case insensitive string comparison in C locale
pub fn c_strcasecmp(s1: &str, s2: &str) -> i32 {
    let mut p1 = s1.bytes();
    let mut p2 = s2.bytes();

    if s1.as_ptr() == s2.as_ptr() {
        return 0;
    }

    let mut c1;
    let mut c2;

    loop {
        c1 = p1.next().map_or(b'\0', |c| c.to_ascii_lowercase());
        c2 = p2.next().map_or(b'\0', |c| c.to_ascii_lowercase());

        if c1 == b'\0' || c1 != c2 {
            break;
        }
    }

    // Rust guarantees usize is at least as large as u32, which can hold u8 difference
    i32::from(c1) - i32::from(c2)
}