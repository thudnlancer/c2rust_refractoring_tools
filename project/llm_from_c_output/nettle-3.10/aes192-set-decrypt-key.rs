/*
   Original C code Copyright (C) 2013, Niels Möller

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
*/

use std::convert::TryInto;

const AES192_ROUNDS: usize = 12;

#[derive(Clone)]
pub struct Aes192Ctx {
    keys: [u32; 4 * (AES192_ROUNDS + 1)],
}

pub fn aes_invert(rounds: usize, dst: &mut [u32], src: &[u32]) {
    assert_eq!(dst.len(), src.len());
    assert_eq!(dst.len(), 4 * (rounds + 1));
    
    let mut i = 0;
    let n = 4 * rounds;
    
    dst[n..n+4].copy_from_slice(&src[0..4]);
    dst[0..4].copy_from_slice(&src[n..n+4]);
    
    i += 4;
    while i < n {
        dst[i..i+4].copy_from_slice(&src[n-i..n-i+4]);
        i += 4;
    }
}

pub fn aes192_invert_key(dst: &mut Aes192Ctx, src: &Aes192Ctx) {
    aes_invert(AES192_ROUNDS, &mut dst.keys, &src.keys);
}

pub fn aes192_set_decrypt_key(ctx: &mut Aes192Ctx, key: &[u8]) -> Result<(), &'static str> {
    if key.len() != 24 {
        return Err("Invalid key length");
    }
    
    // Assuming aes192_set_encrypt_key is implemented elsewhere
    aes192_set_encrypt_key(ctx, key)?;
    aes192_invert_key(ctx, ctx);
    Ok(())
}

// Note: The following functions would need to be implemented separately:
// - aes192_set_encrypt_key
// - The actual AES encryption/decryption operations