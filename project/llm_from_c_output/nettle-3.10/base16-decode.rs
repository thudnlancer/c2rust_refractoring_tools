// base16-encode.rs

// Hex decoding.

// Copyright (C) 2002 Niels Möller

// This file is part of GNU Nettle.

// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:

//   * the GNU Lesser General Public License as published by the Free
//     Software Foundation; either version 3 of the License, or (at your
//     option) any later version.

// or

//   * the GNU General Public License as published by the Free
//     Software Foundation; either version 2 of the License, or (at your
//     option) any later version.

// or both in parallel, as here.

// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

#[derive(Debug, Default)]
pub struct Base16DecodeCtx {
    word: u8,
    bits: u8,
}

impl Base16DecodeCtx {
    pub fn new() -> Self {
        Self { word: 0, bits: 0 }
    }
}

const HEX_INVALID: i8 = -1;
const HEX_SPACE: i8 = -2;

const HEX_DECODE_TABLE: [i8; 0x80] = [
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -2, -2, -1, -1, -2, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -2, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
     0,  1,  2,  3,  4,  5,  6,  7,  8,  9, -1, -1, -1, -1, -1, -1,
    -1, 10, 11, 12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, 10, 11, 12, 13, 14, 15, -1, -1, -1, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
];

/// Decodes a single byte. Returns amount of output (0 or 1), or -1 on errors.
pub fn base16_decode_single(ctx: &mut Base16DecodeCtx, dst: &mut [u8], src: char) -> i8 {
    let usrc = src as u32;
    if usrc >= 0x80 {
        return HEX_INVALID;
    }

    let digit = HEX_DECODE_TABLE[usrc as usize];
    match digit {
        HEX_INVALID => HEX_INVALID,
        HEX_SPACE => 0,
        _ => {
            debug_assert!(digit >= 0);
            debug_assert!(digit < 0x10);

            if ctx.bits != 0 {
                dst[0] = (ctx.word << 4) | (digit as u8);
                ctx.bits = 0;
                1
            } else {
                ctx.word = digit as u8;
                ctx.bits = 4;
                0
            }
        }
    }
}

pub fn base16_decode_update(
    ctx: &mut Base16DecodeCtx,
    dst: &mut [u8],
    src: &str,
) -> Result<usize, &'static str> {
    let mut done = 0;

    for c in src.chars() {
        match base16_decode_single(ctx, &mut dst[done..], c) {
            -1 => return Err("Invalid hex character"),
            1 => {
                done += 1;
            }
            0 => {}
            _ => unreachable!(),
        }
    }

    Ok(done)
}

pub fn base16_decode_final(ctx: &Base16DecodeCtx) -> bool {
    ctx.bits == 0
}

/// Helper function to calculate maximum decode length
pub fn base16_decode_length(src_length: usize) -> usize {
    src_length / 2
}