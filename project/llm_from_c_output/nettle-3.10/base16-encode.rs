// base16-encode.rs

// Hex encoding.

// Copyright (C) 2002 Niels Möller

// This file is part of GNU Nettle.

// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:

//     * the GNU Lesser General Public License as published by the Free
//       Software Foundation; either version 3 of the License, or (at your
//       option) any later version.

// or

//     * the GNU General Public License as published by the Free
//       Software Foundation; either version 2 of the License, or (at your
//       option) any later version.

// or both in parallel, as here.

// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

/// Returns the hexadecimal digit for the given 4-bit value
fn digit(x: u8) -> u8 {
    HEX_DIGITS[(x & 0xf) as usize]
}

/// Encodes a single byte. Always stores two digits in dst[0] and dst[1].
pub fn base16_encode_single(dst: &mut [u8; 2], src: u8) {
    dst[0] = digit(src / 0x10);
    dst[1] = digit(src);
}

/// Always stores BASE16_ENCODE_LENGTH(length) digits in dst.
pub fn base16_encode_update(dst: &mut [u8], src: &[u8]) -> Result<(), &'static str> {
    if dst.len() < src.len() * 2 {
        return Err("Destination buffer too small");
    }

    for (i, &byte) in src.iter().enumerate() {
        let chunk = &mut dst[i * 2..i * 2 + 2];
        base16_encode_single(chunk.try_into().unwrap(), byte);
    }

    Ok(())
}