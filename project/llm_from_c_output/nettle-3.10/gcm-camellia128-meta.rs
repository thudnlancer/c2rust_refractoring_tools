/* gcm-camellia128-meta.rs

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
use crate::nettle_meta::{NettleAead, NettleSetKeyFunc, NettleHashUpdateFunc, NettleCryptFunc, NettleHashDigestFunc};
use crate::gcm::{GcmCamellia128Ctx, gcm_camellia128_set_iv, gcm_camellia128_set_key, gcm_camellia128_update, 
                 gcm_camellia128_encrypt, gcm_camellia128_decrypt, gcm_camellia128_digest};
use crate::constants::{GCM_BLOCK_SIZE, CAMELLIA128_KEY_SIZE, GCM_IV_SIZE, GCM_DIGEST_SIZE};

fn gcm_camellia128_set_nonce_wrapper(ctx: &mut GcmCamellia128Ctx, nonce: &[u8]) {
    gcm_camellia128_set_iv(ctx, GCM_IV_SIZE, nonce);
}

pub const NETTLE_GCM_CAMELLIA128: NettleAead = NettleAead {
    name: "gcm_camellia128",
    context_size: size_of::<GcmCamellia128Ctx>(),
    block_size: GCM_BLOCK_SIZE,
    key_size: CAMELLIA128_KEY_SIZE,
    iv_size: GCM_IV_SIZE,
    digest_size: GCM_DIGEST_SIZE,
    set_encrypt_key: gcm_camellia128_set_key as NettleSetKeyFunc,
    set_decrypt_key: gcm_camellia128_set_key as NettleSetKeyFunc,
    set_nonce: gcm_camellia128_set_nonce_wrapper as NettleSetKeyFunc,
    update: gcm_camellia128_update as NettleHashUpdateFunc,
    encrypt: gcm_camellia128_encrypt as NettleCryptFunc,
    decrypt: gcm_camellia128_decrypt as NettleCryptFunc,
    digest: gcm_camellia128_digest as NettleHashDigestFunc,
};