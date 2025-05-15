// write-le32.rs

// Copyright (C) 2001, 2011 Niels Möller
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

pub fn write_le32(dst: &mut [u8], src: &[u32]) {
    let length = dst.len();
    let words = length / 4;
    let leftover = length % 4;

    for (i, chunk) in dst.chunks_exact_mut(4).take(words).enumerate() {
        let bytes = src[i].to_le_bytes();
        chunk.copy_from_slice(&bytes);
    }

    if leftover > 0 {
        let mut word = src[words];
        for byte in dst[words * 4..].iter_mut() {
            *byte = (word & 0xff) as u8;
            word >>= 8;
        }
    }
}