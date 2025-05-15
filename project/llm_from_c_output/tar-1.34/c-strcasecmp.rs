// c-strcasecmp.rs -- case insensitive string comparator in C locale
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

/// Case insensitive string comparison using C locale
pub fn c_strcasecmp(s1: &str, s2: &str) -> i32 {
    let mut p1 = s1.bytes();
    let mut p2 = s2.bytes();

    if s1.as_ptr() == s2.as_ptr() {
        return 0;
    }

    let mut c1: u8;
    let mut c2: u8;

    loop {
        c1 = p1.next().unwrap_or(0);
        c2 = p2.next().unwrap_or(0);

        c1 = c_tolower(c1);
        c2 = c_tolower(c2);

        if c1 == b'\0' {
            break;
        }

        if c1 != c2 {
            break;
        }
    }

    if u8::MAX <= i32::MAX as u64 {
        (c1 as i32) - (c2 as i32)
    } else {
        // On machines where 'char' and 'int' are types of the same size, the
        // difference of two 'unsigned char' values - including the sign bit -
        // doesn't fit in an 'int'.
        if c1 < c2 {
            -1
        } else if c1 > c2 {
            1
        } else {
            0
        }
    }
}

/// Convert character to lowercase using C locale
fn c_tolower(c: u8) -> u8 {
    if c.is_ascii_uppercase() {
        c.to_ascii_lowercase()
    } else {
        c
    }
}