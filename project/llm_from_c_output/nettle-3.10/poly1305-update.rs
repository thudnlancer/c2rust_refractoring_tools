// poly1305-update.rs

// Copyright (C) 2022 Niels Möller
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

use std::cmp;

const POLY1305_BLOCK_SIZE: usize = 16;

pub struct Poly1305Ctx {
    // Context fields would be defined here
}

impl Poly1305Ctx {
    fn block(&mut self, m: &[u8], partial: u8) {
        // Implementation of _nettle_poly1305_block
    }
}

#[cfg(feature = "native_fat_poly1305_blocks")]
fn poly1305_blocks_c(ctx: &mut Poly1305Ctx, blocks: usize, m: &[u8]) -> &[u8] {
    let mut m = m;
    for _ in 0..blocks {
        ctx.block(m, 1);
        m = &m[POLY1305_BLOCK_SIZE..];
    }
    m
}

pub fn poly1305_update(
    ctx: &mut Poly1305Ctx,
    block: &mut [u8],
    index: usize,
    length: usize,
    m: &[u8],
) -> usize {
    if length == 0 {
        return index;
    }

    let mut m = m;
    let mut length = length;
    let mut index = index;

    if index > 0 {
        // Try to fill partial block
        let fill = cmp::min(POLY1305_BLOCK_SIZE - index, length);
        block[index..index + fill].copy_from_slice(&m[..fill]);
        m = &m[fill..];
        length -= fill;
        index += fill;

        if index == POLY1305_BLOCK_SIZE {
            ctx.block(block, 1);
            index = 0;
        } else {
            return index;
        }
    }

    #[cfg(feature = "native_poly1305_blocks")]
    {
        let blocks = length >> 4;
        m = poly1305_blocks_c(ctx, blocks, m);
        length &= 15;
    }
    #[cfg(not(feature = "native_poly1305_blocks"))]
    {
        while length >= POLY1305_BLOCK_SIZE {
            ctx.block(m, 1);
            m = &m[POLY1305_BLOCK_SIZE..];
            length -= POLY1305_BLOCK_SIZE;
        }
    }

    if length > 0 {
        block[..length].copy_from_slice(&m[..length]);
    }

    length
}