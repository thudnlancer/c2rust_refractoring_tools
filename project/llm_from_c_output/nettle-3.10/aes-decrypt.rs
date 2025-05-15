// aes-decrypt.rs

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

use std::process;

pub fn aes_decrypt(
    ctx: &AesCtx,
    length: usize,
    dst: &mut [u8],
    src: &[u8],
) -> Result<(), &'static str> {
    match ctx.key_size {
        AES128_KEY_SIZE => aes128_decrypt(&ctx.u.ctx128, length, dst, src),
        AES192_KEY_SIZE => aes192_decrypt(&ctx.u.ctx192, length, dst, src),
        AES256_KEY_SIZE => aes256_decrypt(&ctx.u.ctx256, length, dst, src),
        _ => {
            process::abort();
        }
    }
    Ok(())
}

// Note: The following types and constants would need to be defined elsewhere in your Rust code:
// - AesCtx struct with key_size field and u union
// - AES128_KEY_SIZE, AES192_KEY_SIZE, AES256_KEY_SIZE constants
// - aes128_decrypt, aes192_decrypt, aes256_decrypt functions