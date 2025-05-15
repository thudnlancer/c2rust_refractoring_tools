// hash_pjw.rs -- compute a hash value from a NUL-terminated string.

// This file is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as
// published by the Free Software Foundation; either version 2.1 of the
// License, or (at your option) any later version.

// This file is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.

// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

// A hash function for NUL-terminated strings using
// the method described by Bruno Haible.
// See https://www.haible.de/bruno/hashfunc.html.

use std::convert::TryInto;

const SIZE_BITS: usize = std::mem::size_of::<usize>() * 8;

/// Compute a hash code for a NUL-terminated string starting at x,
/// and return the hash code modulo tablesize.
/// The result is platform dependent: it depends on the size of the 'usize'
/// type and on the signedness of the 'char' type.
#[must_use]
pub fn hash_pjw(x: &str, tablesize: usize) -> usize {
    let mut h: usize = 0;

    for c in x.chars() {
        let c_val = c as usize;
        h = c_val.wrapping_add((h << 9) | (h >> (SIZE_BITS - 9)));
    }

    h % tablesize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_pjw() {
        assert_eq!(hash_pjw("test", 100), hash_pjw("test", 100));
        assert_ne!(hash_pjw("test1", 100), hash_pjw("test2", 100));
    }
}