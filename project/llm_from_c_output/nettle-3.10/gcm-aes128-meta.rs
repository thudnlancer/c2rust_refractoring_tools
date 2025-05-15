/* gcm-aes128-meta.rs

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

pub const GCM_BLOCK_SIZE: usize = 16;
pub const AES128_KEY_SIZE: usize = 16;
pub const GCM_IV_SIZE: usize = 12;
pub const GCM_DIGEST_SIZE: usize = 16;

pub struct GcmAes128Ctx {
    // Implementation details of the GCM AES128 context
}

pub fn gcm_aes128_set_key(ctx: &mut GcmAes128Ctx, key: &[u8; AES128_KEY_SIZE]) {
    // Implementation of key setting
}

pub fn gcm_aes128_set_iv(ctx: &mut GcmAes128Ctx, iv_len: usize, iv: &[u8]) {
    // Implementation of IV setting
}

pub fn gcm_aes128_update(ctx: &mut GcmAes128Ctx, data: &[u8]) {
    // Implementation of update
}

pub fn gcm_aes128_encrypt(ctx: &mut GcmAes128Ctx, length: usize, dst: &mut [u8], src: &[u8]) {
    // Implementation of encryption
}

pub fn gcm_aes128_decrypt(ctx: &mut GcmAes128Ctx, length: usize, dst: &mut [u8], src: &[u8]) {
    // Implementation of decryption
}

pub fn gcm_aes128_digest(ctx: &mut GcmAes128Ctx, length: usize, digest: &mut [u8]) {
    // Implementation of digest
}

fn gcm_aes128_set_nonce_wrapper(ctx: &mut GcmAes128Ctx, nonce: &[u8; GCM_IV_SIZE]) {
    gcm_aes128_set_iv(ctx, GCM_IV_SIZE, nonce);
}

pub struct NettleAead {
    pub name: &'static str,
    pub context_size: usize,
    pub block_size: usize,
    pub key_size: usize,
    pub iv_size: usize,
    pub digest_size: usize,
    pub set_encrypt_key: fn(&mut GcmAes128Ctx, &[u8; AES128_KEY_SIZE]),
    pub set_decrypt_key: fn(&mut GcmAes128Ctx, &[u8; AES128_KEY_SIZE]),
    pub set_nonce: fn(&mut GcmAes128Ctx, &[u8; GCM_IV_SIZE]),
    pub update: fn(&mut GcmAes128Ctx, &[u8]),
    pub encrypt: fn(&mut GcmAes128Ctx, usize, &mut [u8], &[u8]),
    pub decrypt: fn(&mut GcmAes128Ctx, usize, &mut [u8], &[u8]),
    pub digest: fn(&mut GcmAes128Ctx, usize, &mut [u8]),
}

pub const NETTLE_GCM_AES128: NettleAead = NettleAead {
    name: "gcm_aes128",
    context_size: size_of::<GcmAes128Ctx>(),
    block_size: GCM_BLOCK_SIZE,
    key_size: AES128_KEY_SIZE,
    iv_size: GCM_IV_SIZE,
    digest_size: GCM_DIGEST_SIZE,
    set_encrypt_key: gcm_aes128_set_key,
    set_decrypt_key: gcm_aes128_set_key,
    set_nonce: gcm_aes128_set_nonce_wrapper,
    update: gcm_aes128_update,
    encrypt: gcm_aes128_encrypt,
    decrypt: gcm_aes128_decrypt,
    digest: gcm_aes128_digest,
};