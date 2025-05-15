// write-le64.rs

// Copyright (C) 2001, 2011, 2012 Niels Möller
//
// This file is part of GNU Nettle.
//
// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:
//
//   * the GNU Lesser General Public License as published by the Free
//     Software Foundation; either version 3 of the License, or (at your
//     option) any later version.
//
// or
//
//   * the GNU General Public License as published by the Free
//     Software Foundation; either version 2 of the License, or (at your
//     option) any later version.
//
// or both in parallel, as here.
//
// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use std::convert::TryInto;

/// Writes a slice of 64-bit integers to a byte slice in little-endian format.
/// 
/// # Arguments
/// * `dst` - The destination byte slice to write to
/// * `src` - The source slice of 64-bit integers to read from
/// 
/// # Panics
/// Panics if the destination slice is not large enough to hold all the bytes.
pub fn write_le64(dst: &mut [u8], src: &[u64]) {
    let length = dst.len();
    let words = length / 8;
    let leftover = length % 8;

    for (i, chunk) in dst.chunks_exact_mut(8).take(words).enumerate() {
        chunk.copy_from_slice(&src[i].to_le_bytes());
    }

    if leftover > 0 {
        let word = src[words];
        let mut remaining_word = word;
        for byte in dst[words * 8..].iter_mut() {
            *byte = (remaining_word & 0xff) as u8;
            remaining_word >>= 8;
        }
    }
}