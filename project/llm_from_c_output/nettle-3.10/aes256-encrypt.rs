// aes256-encrypt.rs

// Encryption function for the aes/rijndael block cipher.

// Copyright (C) 2002, 2013 Niels Möller

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

use std::assert;

use crate::aes_internal::{
    aes_encrypt, 
    AesEncryptTable, 
    AES_BLOCK_SIZE, 
    AES256_ROUNDS
};

pub struct Aes256Ctx {
    keys: [u32; 60], // Assuming 60 keys based on AES-256 key schedule
}

pub fn aes256_encrypt(
    ctx: &Aes256Ctx,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) -> Result<(), &'static str> {
    if length % AES_BLOCK_SIZE != 0 {
        return Err("Input length must be a multiple of AES_BLOCK_SIZE");
    }
    if dst.len() < length || src.len() < length {
        return Err("Output or input buffer too small");
    }

    aes_encrypt(
        AES256_ROUNDS,
        &ctx.keys,
        &AesEncryptTable::default(),
        length,
        dst,
        src,
    );
    Ok(())
}