/* aes256-set-encrypt-key.rs

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

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see http://www.gnu.org/licenses/.
*/

use crate::aes_internal::{Aes256Ctx, aes_set_key};
use std::convert::TryInto;

pub const AES256_ROUNDS: usize = 14;
pub const AES256_KEY_SIZE: usize = 32;

/// For fat builds (placeholder, would need platform-specific implementation)
#[cfg(feature = "native_aes256")]
pub fn _nettle_aes256_set_encrypt_key_c(ctx: &mut Aes256Ctx, key: &[u8; AES256_KEY_SIZE]) {
    nettle_aes256_set_encrypt_key(ctx, key)
}

/// Sets the AES-256 encryption key
pub fn nettle_aes256_set_encrypt_key(ctx: &mut Aes256Ctx, key: &[u8; AES256_KEY_SIZE]) {
    aes_set_key(AES256_ROUNDS, AES256_KEY_SIZE / 4, &mut ctx.keys, key);
}