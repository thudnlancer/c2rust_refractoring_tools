/* gcm-camellia256-meta.rs

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
use crate::gcm::{GcmCamellia256Context, GCM_BLOCK_SIZE, GCM_IV_SIZE, GCM_DIGEST_SIZE};
use crate::camellia::CAMELLIA256_KEY_SIZE;

pub struct NettleAead {
    pub name: &'static str,
    pub context_size: usize,
    pub block_size: usize,
    pub key_size: usize,
    pub iv_size: usize,
    pub digest_size: usize,
    pub set_encrypt_key: fn(&mut GcmCamellia256Context, &[u8]) -> Result<(), &'static str>,
    pub set_decrypt_key: fn(&mut GcmCamellia256Context, &[u8]) -> Result<(), &'static str>,
    pub set_nonce: fn(&mut GcmCamellia256Context, &[u8]) -> Result<(), &'static str>,
    pub update: fn(&mut GcmCamellia256Context, &[u8]) -> Result<(), &'static str>,
    pub encrypt: fn(&mut GcmCamellia256Context, &mut [u8], &[u8]) -> Result<(), &'static str>,
    pub decrypt: fn(&mut GcmCamellia256Context, &mut [u8], &[u8]) -> Result<(), &'static str>,
    pub digest: fn(&mut GcmCamellia256Context, &mut [u8]) -> Result<(), &'static str>,
}

fn gcm_camellia256_set_nonce_wrapper(ctx: &mut GcmCamellia256Context, nonce: &[u8]) -> Result<(), &'static str> {
    gcm_camellia256_set_iv(ctx, GCM_IV_SIZE, nonce)
}

pub const NETTLE_GCM_CAMELLIA256: NettleAead = NettleAead {
    name: "gcm_camellia256",
    context_size: size_of::<GcmCamellia256Context>(),
    block_size: GCM_BLOCK_SIZE,
    key_size: CAMELLIA256_KEY_SIZE,
    iv_size: GCM_IV_SIZE,
    digest_size: GCM_DIGEST_SIZE,
    set_encrypt_key: gcm_camellia256_set_key,
    set_decrypt_key: gcm_camellia256_set_key,
    set_nonce: gcm_camellia256_set_nonce_wrapper,
    update: gcm_camellia256_update,
    encrypt: gcm_camellia256_encrypt,
    decrypt: gcm_camellia256_decrypt,
    digest: gcm_camellia256_digest,
};