/* chacha-poly1305-meta.rs

   Copyright (C) 2014 Niels Möller

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

use std::mem::size_of;

use crate::nettle_meta::NettleAead;
use crate::chacha_poly1305::{
    ChachaPoly1305Ctx,
    CHACHA_POLY1305_BLOCK_SIZE,
    CHACHA_POLY1305_KEY_SIZE,
    CHACHA_POLY1305_NONCE_SIZE,
    CHACHA_POLY1305_DIGEST_SIZE,
    chacha_poly1305_set_key,
    chacha_poly1305_set_nonce,
    chacha_poly1305_update,
    chacha_poly1305_encrypt,
    chacha_poly1305_decrypt,
    chacha_poly1305_digest,
};

pub const NETTLE_CHACHA_POLY1305: NettleAead = NettleAead {
    name: "chacha_poly1305",
    context_size: size_of::<ChachaPoly1305Ctx>(),
    block_size: CHACHA_POLY1305_BLOCK_SIZE,
    key_size: CHACHA_POLY1305_KEY_SIZE,
    nonce_size: CHACHA_POLY1305_NONCE_SIZE,
    digest_size: CHACHA_POLY1305_DIGEST_SIZE,
    set_encrypt_key: chacha_poly1305_set_key,
    set_decrypt_key: chacha_poly1305_set_key,
    set_nonce: chacha_poly1305_set_nonce,
    update: chacha_poly1305_update,
    encrypt: chacha_poly1305_encrypt,
    decrypt: chacha_poly1305_decrypt,
    digest: chacha_poly1305_digest,
};