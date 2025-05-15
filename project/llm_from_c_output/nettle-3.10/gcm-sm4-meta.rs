/* gcm-sm4-meta.rs

   Copyright (C) 2022 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>

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
use std::ptr;

pub const GCM_BLOCK_SIZE: usize = 16;
pub const SM4_KEY_SIZE: usize = 16;
pub const GCM_IV_SIZE: usize = 12;
pub const GCM_DIGEST_SIZE: usize = 16;

pub struct GcmSm4Context {
    // Context fields would be defined here
}

pub type SetKeyFunc = fn(ctx: &mut GcmSm4Context, key: &[u8]);
pub type HashUpdateFunc = fn(ctx: &mut GcmSm4Context, length: usize, data: &[u8]);
pub type CryptFunc = fn(ctx: &mut GcmSm4Context, length: usize, dst: &mut [u8], src: &[u8]);
pub type HashDigestFunc = fn(ctx: &mut GcmSm4Context, length: usize, digest: &mut [u8]);

fn gcm_sm4_set_nonce_wrapper(ctx: &mut GcmSm4Context, nonce: &[u8]) {
    assert_eq!(nonce.len(), GCM_IV_SIZE);
    gcm_sm4_set_iv(ctx, GCM_IV_SIZE, nonce);
}

pub struct NettleAead {
    pub name: &'static str,
    pub context_size: usize,
    pub block_size: usize,
    pub key_size: usize,
    pub nonce_size: usize,
    pub digest_size: usize,
    pub set_encrypt_key: SetKeyFunc,
    pub set_decrypt_key: SetKeyFunc,
    pub set_nonce: fn(&mut GcmSm4Context, &[u8]),
    pub update: HashUpdateFunc,
    pub encrypt: CryptFunc,
    pub decrypt: CryptFunc,
    pub digest: HashDigestFunc,
}

pub const NETTLE_GCM_SM4: NettleAead = NettleAead {
    name: "gcm_sm4",
    context_size: size_of::<GcmSm4Context>(),
    block_size: GCM_BLOCK_SIZE,
    key_size: SM4_KEY_SIZE,
    nonce_size: GCM_IV_SIZE,
    digest_size: GCM_DIGEST_SIZE,
    set_encrypt_key: gcm_sm4_set_key,
    set_decrypt_key: gcm_sm4_set_key,
    set_nonce: gcm_sm4_set_nonce_wrapper,
    update: gcm_sm4_update,
    encrypt: gcm_sm4_encrypt,
    decrypt: gcm_sm4_decrypt,
    digest: gcm_sm4_digest,
};

// These functions would be implemented elsewhere
fn gcm_sm4_set_key(_ctx: &mut GcmSm4Context, _key: &[u8]) {}
fn gcm_sm4_set_iv(_ctx: &mut GcmSm4Context, _iv_len: usize, _iv: &[u8]) {}
fn gcm_sm4_update(_ctx: &mut GcmSm4Context, _length: usize, _data: &[u8]) {}
fn gcm_sm4_encrypt(_ctx: &mut GcmSm4Context, _length: usize, _dst: &mut [u8], _src: &[u8]) {}
fn gcm_sm4_decrypt(_ctx: &mut GcmSm4Context, _length: usize, _dst: &mut [u8], _src: &[u8]) {}
fn gcm_sm4_digest(_ctx: &mut GcmSm4Context, _length: usize, _digest: &mut [u8]) {}