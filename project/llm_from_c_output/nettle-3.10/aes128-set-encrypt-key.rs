// aes128-set-encrypt-key.rs

// Copyright (C) 2013, Niels Möller
//
// This file is part of GNU Nettle.
//
// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:
//
//   * the GNU Lesser General Public License as published by the Free
//     Software Foundation; either version 3 of the License, or (at your
//     option) any later version.
//
// or
//
//   * the GNU General Public License as published by the Free
//     Software Foundation; either version 2 of the License, or (at your
//     option) any later version.
//
// or both in parallel, as here.
//
// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use crate::aes_internal::{Aes128Ctx, aes_set_key};
use std::convert::TryInto;

const AES128_ROUNDS: usize = 10;
const AES128_KEY_SIZE: usize = 16;

/// Sets the encryption key for AES-128
pub fn aes128_set_encrypt_key(ctx: &mut Aes128Ctx, key: &[u8; AES128_KEY_SIZE]) {
    aes_set_key(
        AES128_ROUNDS,
        AES128_KEY_SIZE / 4,
        &mut ctx.keys,
        key,
    );
}