// hash_pjw.rs -- compute a hash value from a NUL-terminated string.
//
// Copyright (C) 2001, 2003, 2006, 2009-2020 Free Software Foundation, Inc.
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

use std::mem;

const SIZE_BITS: usize = mem::size_of::<usize>() * 8;

/// Compute a hash code for a NUL-terminated string starting at s,
/// and return the hash code modulo tablesize.
/// The result is platform dependent: it depends on the size of the 'usize'
/// type and on the signedness of the 'char' type.
/// 
/// # Arguments
/// * `s` - A reference to a NUL-terminated string (C-style string)
/// * `tablesize` - The size of the hash table to modulo the result by
/// 
/// # Returns
/// The hash code modulo tablesize
pub fn hash_pjw(s: &str, tablesize: usize) -> usize {
    let mut h: usize = 0;
    
    for c in s.chars() {
        h = c as usize + ((h << 9) | (h >> (SIZE_BITS - 9)));
    }
    
    h % tablesize
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hash_pjw() {
        let test_str = "test";
        let tablesize = 100;
        let hash = hash_pjw(test_str, tablesize);
        assert!(hash < tablesize);
    }
}