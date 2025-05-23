// aes192-set-encrypt-key.rs

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

use crate::aes_internal::{Aes192Ctx, aes_set_key};
use std::convert::TryInto;

pub const AES192_ROUNDS: usize = 12;
pub const AES192_KEY_SIZE: usize = 24;

/// For fat builds
#[cfg(feature = "native_aes192_set_encrypt_key")]
pub fn _nettle_aes192_set_encrypt_key_c(ctx: &mut Aes192Ctx, key: &[u8; AES192_KEY_SIZE]) {
    aes192_set_encrypt_key(ctx, key);
}

pub fn aes192_set_encrypt_key(ctx: &mut Aes192Ctx, key: &[u8; AES192_KEY_SIZE]) {
    aes_set_key(
        AES192_ROUNDS,
        AES192_KEY_SIZE / 4,
        &mut ctx.keys,
        key.try_into().unwrap(),
    );
}