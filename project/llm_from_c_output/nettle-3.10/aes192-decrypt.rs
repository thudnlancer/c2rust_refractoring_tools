// aes192-decrypt.rs

// Decryption function for aes/rijndael block cipher.

// Copyright (C) 2002, 2013 Niels Möller

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

use crate::aes_internal::{Aes192Ctx, AesBlockSize, aes_decrypt, AES192_ROUNDS};

/// AES-192 decryption function
///
/// # Arguments
/// * `ctx` - AES-192 context containing the key schedule
/// * `length` - Length of the data to decrypt (must be multiple of AES block size)
/// * `dst` - Destination buffer for decrypted data
/// * `src` - Source buffer containing encrypted data
///
/// # Panics
/// Panics if length is not a multiple of AES block size
pub fn nettle_aes192_decrypt(
    ctx: &Aes192Ctx,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) -> Result<(), &'static str> {
    if length % AesBlockSize::SIZE != 0 {
        return Err("Input length must be multiple of AES block size");
    }

    if dst.len() < length || src.len() < length {
        return Err("Output or input buffer too small");
    }

    aes_decrypt(
        AES192_ROUNDS,
        &ctx.keys[4 * AES192_ROUNDS..],
        length,
        dst,
        src,
    )
}