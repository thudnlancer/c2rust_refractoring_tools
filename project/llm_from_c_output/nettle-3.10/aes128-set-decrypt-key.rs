// aes128-set-decrypt-key.rs

// Key setup for the aes/rijndael block cipher.

// Copyright (C) 2013, Niels Möller

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

use crate::aes_internal::{aes_invert, Aes128Ctx, AES128_ROUNDS};
use crate::macros::*;

/// For fat builds
#[cfg(feature = "native_aes128_invert_key")]
pub fn _nettle_aes128_invert_key_c(dst: &mut Aes128Ctx, src: &Aes128Ctx) {
    aes_invert(AES128_ROUNDS, &mut dst.keys, &src.keys);
}

#[cfg(feature = "native_aes128_set_decrypt_key")]
pub fn _nettle_aes128_set_decrypt_key_c(ctx: &mut Aes128Ctx, key: &[u8; 16]) {
    aes128_set_encrypt_key(ctx, key);
    aes128_invert_key(ctx, ctx);
}

pub fn nettle_aes128_invert_key(dst: &mut Aes128Ctx, src: &Aes128Ctx) {
    aes_invert(AES128_ROUNDS, &mut dst.keys, &src.keys);
}

pub fn nettle_aes128_set_decrypt_key(ctx: &mut Aes128Ctx, key: &[u8; 16]) {
    aes128_set_encrypt_key(ctx, key);
    nettle_aes128_invert_key(ctx, ctx);
}