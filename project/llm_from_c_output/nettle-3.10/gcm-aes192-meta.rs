/* gcm-aes192-meta.rs

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
use crate::gcm::{GcmAes192Ctx, GCM_BLOCK_SIZE, GCM_IV_SIZE, GCM_DIGEST_SIZE};
use crate::aes::{AES192_KEY_SIZE};

pub struct NettleAead {
    pub name: &'static str,
    pub context_size: usize,
    pub block_size: usize,
    pub key_size: usize,
    pub nonce_size: usize,
    pub digest_size: usize,
    pub set_encrypt_key: fn(&mut GcmAes192Ctx, &[u8]),
    pub set_decrypt_key: fn(&mut GcmAes192Ctx, &[u8]),
    pub set_nonce: fn(&mut GcmAes192Ctx, &[u8]),
    pub update: fn(&mut GcmAes192Ctx, &[u8]),
    pub encrypt: fn(&mut GcmAes192Ctx, &mut [u8], &[u8]),
    pub decrypt: fn(&mut GcmAes192Ctx, &mut [u8], &[u8]),
    pub digest: fn(&mut GcmAes192Ctx, &mut [u8]),
}

fn gcm_aes192_set_nonce_wrapper(ctx: &mut GcmAes192Ctx, nonce: &[u8]) {
    gcm_aes192_set_iv(ctx, GCM_IV_SIZE, nonce);
}

pub const NETTLE_GCM_AES192: NettleAead = NettleAead {
    name: "gcm_aes192",
    context_size: size_of::<GcmAes192Ctx>(),
    block_size: GCM_BLOCK_SIZE,
    key_size: AES192_KEY_SIZE,
    nonce_size: GCM_IV_SIZE,
    digest_size: GCM_DIGEST_SIZE,
    set_encrypt_key: gcm_aes192_set_key,
    set_decrypt_key: gcm_aes192_set_key,
    set_nonce: gcm_aes192_set_nonce_wrapper,
    update: gcm_aes192_update,
    encrypt: gcm_aes192_encrypt,
    decrypt: gcm_aes192_decrypt,
    digest: gcm_aes192_digest,
};