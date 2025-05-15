/* salsa20-128-set-key.rs

   The Salsa20 stream cipher. Key setup for 128-bit keys.

   Copyright (C) 2012 Simon Josefsson
   Copyright (C) 2012-2014 Niels Möller

   This file is part of GNU Nettle.

   GNU Nettle is free software: you can redistribute it and/or
   modify it under the terms of either:

     * the GNU Lesser General Public License as published by the Free
       Software Foundation; either version 3 of the License, or (at your
       option) any later version.

   or

     * the GNU General Public License as published by the Free
       Software Foundation; either version 2 of the License, or (at your
       option) any later version.

   or both in parallel, as here.

   GNU Nettle is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see http://www.gnu.org/licenses/.
*/

/* Based on:
   salsa20-ref.c version 20051118
   D. J. Bernstein
   Public domain.
*/

use std::convert::TryInto;

pub struct Salsa20Ctx {
    pub input: [u32; 16],
}

impl Salsa20Ctx {
    pub fn new() -> Self {
        Salsa20Ctx {
            input: [0; 16],
        }
    }
}

pub fn salsa20_128_set_key(ctx: &mut Salsa20Ctx, key: &[u8]) -> Result<(), &'static str> {
    if key.len() != 16 {
        return Err("Key length must be 16 bytes");
    }

    ctx.input[11] = ctx.input[1] = u32::from_le_bytes(key[0..4].try_into().unwrap());
    ctx.input[12] = ctx.input[2] = u32::from_le_bytes(key[4..8].try_into().unwrap());
    ctx.input[13] = ctx.input[3] = u32::from_le_bytes(key[8..12].try_into().unwrap());
    ctx.input[14] = ctx.input[4] = u32::from_le_bytes(key[12..16].try_into().unwrap());

    /* "expand 16-byte k" */
    ctx.input[0]  = 0x61707865;
    ctx.input[5]  = 0x3120646e;
    ctx.input[10] = 0x79622d36;
    ctx.input[15] = 0x6b206574;

    Ok(())
}