// aes128-encrypt.rs

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
    Aes128Ctx,
    AES_BLOCK_SIZE,
    aes_encrypt,
    AesEncryptTable,
    AES128_ROUNDS
};

/// AES-128 encryption function
///
/// # Arguments
/// * `ctx` - AES-128 context containing the encryption key
/// * `length` - Length of the input/output buffers (must be multiple of AES_BLOCK_SIZE)
/// * `dst` - Destination buffer for encrypted data
/// * `src` - Source buffer with plaintext data
///
/// # Panics
/// Panics if length is not a multiple of AES_BLOCK_SIZE
pub fn nettle_aes128_encrypt(
    ctx: &Aes128Ctx,
    length: usize,
    dst: &mut [u8],
    src: &[u8]
) {
    assert!(length % AES_BLOCK_SIZE == 0, "Length must be a multiple of AES_BLOCK_SIZE");
    assert!(dst.len() >= length, "Destination buffer too small");
    assert!(src.len() >= length, "Source buffer too small");
    
    aes_encrypt(
        AES128_ROUNDS,
        &ctx.keys,
        &AesEncryptTable,
        length,
        dst,
        src
    );
}