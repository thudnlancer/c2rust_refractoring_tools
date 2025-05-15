// hash_pjw.rs -- compute a hash value from a NUL-terminated string.
//
// Copyright (C) 2001, 2003, 2006, 2009-2023 Free Software Foundation, Inc.
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

//! A hash function for strings using the method described by Bruno Haible.
//! See <https://www.haible.de/bruno/hashfunc.html>.

use std::mem;

const SIZE_BITS: usize = mem::size_of::<usize>() * 8;

/// Compute a hash code for a string, and return the hash code modulo tablesize.
/// The result is platform dependent: it depends on the size of the 'usize' type.
#[must_use]
pub fn hash_pjw(s: &str, tablesize: usize) -> usize {
    let mut h: usize = 0;
    
    for &c in s.as_bytes() {
        h = c as usize + ((h << 9) | (h >> (SIZE_BITS - 9)));
    }
    
    h % tablesize
}