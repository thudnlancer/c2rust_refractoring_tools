// aes192-encrypt.rs

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
    Aes192Ctx,
    AES_BLOCK_SIZE,
    _AES192_ROUNDS,
    _nettle_aes_encrypt,
    _nettle_aes_encrypt_table,
};

/// For fat builds
#[cfg(feature = "native_aes192_encrypt")]
fn _nettle_aes192_encrypt_c(
    ctx: &Aes192Ctx,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) {
    // Native implementation would go here
}

/// Main encryption function
pub fn nettle_aes192_encrypt(
    ctx: &Aes192Ctx,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) -> Result<(), &'static str> {
    assert!(length % AES_BLOCK_SIZE == 0, "Length must be a multiple of AES block size");
    
    if dst.len() < length || src.len() < length {
        return Err("Destination or source buffer too small");
    }

    _nettle_aes_encrypt(
        _AES192_ROUNDS,
        &ctx.keys,
        &_nettle_aes_encrypt_table,
        length,
        dst,
        src,
    );

    Ok(())
}