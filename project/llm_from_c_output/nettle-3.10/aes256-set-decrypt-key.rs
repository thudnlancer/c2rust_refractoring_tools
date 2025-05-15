// aes256-set-decrypt-key.rs

/*
   Copyright (C) 2013, Niels Möller

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

use crate::aes_internal::{aes256_set_encrypt_key, aes_invert, Aes256Ctx, AES256_ROUNDS};

/// Inverts the AES-256 key schedule
pub fn aes256_invert_key(dst: &mut Aes256Ctx, src: &Aes256Ctx) {
    aes_invert(AES256_ROUNDS, &mut dst.keys, &src.keys);
}

/// Sets up an AES-256 decryption key from the given encryption key
pub fn aes256_set_decrypt_key(ctx: &mut Aes256Ctx, key: &[u8; 32]) {
    aes256_set_encrypt_key(ctx, key);
    aes256_invert_key(ctx, ctx);
}