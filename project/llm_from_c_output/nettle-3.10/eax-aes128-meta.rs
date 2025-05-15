/* eax-aes128-meta.rs

   Copyright (C) 2013, 2014 Niels Möller

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

pub const EAX_BLOCK_SIZE: usize = 16;
pub const AES128_KEY_SIZE: usize = 16;
pub const EAX_IV_SIZE: usize = 16;
pub const EAX_DIGEST_SIZE: usize = 16;

pub struct EaxAes128Ctx {
    // Context fields would be defined here
}

pub type SetKeyFunc = fn(ctx: &mut EaxAes128Ctx, key: &[u8]);
pub type SetNonceFunc = fn(ctx: &mut EaxAes128Ctx, nonce: &[u8]);
pub type UpdateFunc = fn(ctx: &mut EaxAes128Ctx, data: &[u8]);
pub type CryptFunc = fn(ctx: &mut EaxAes128Ctx, length: usize, dst: &mut [u8], src: &[u8]);
pub type DigestFunc = fn(ctx: &mut EaxAes128Ctx, length: usize, digest: &mut [u8]);

pub struct NettleAead {
    pub name: &'static str,
    pub context_size: usize,
    pub block_size: usize,
    pub key_size: usize,
    pub nonce_size: usize,
    pub digest_size: usize,
    pub set_encrypt_key: SetKeyFunc,
    pub set_decrypt_key: SetKeyFunc,
    pub set_nonce: SetNonceFunc,
    pub update: UpdateFunc,
    pub encrypt: CryptFunc,
    pub decrypt: CryptFunc,
    pub digest: DigestFunc,
}

fn eax_aes128_set_nonce_wrapper(ctx: &mut EaxAes128Ctx, nonce: &[u8]) {
    eax_aes128_set_nonce(ctx, EAX_IV_SIZE, nonce);
}

pub const NETTLE_EAX_AES128: NettleAead = NettleAead {
    name: "eax_aes128",
    context_size: size_of::<EaxAes128Ctx>(),
    block_size: EAX_BLOCK_SIZE,
    key_size: AES128_KEY_SIZE,
    nonce_size: EAX_IV_SIZE,
    digest_size: EAX_DIGEST_SIZE,
    set_encrypt_key: eax_aes128_set_key,
    set_decrypt_key: eax_aes128_set_key,
    set_nonce: eax_aes128_set_nonce_wrapper,
    update: eax_aes128_update,
    encrypt: eax_aes128_encrypt,
    decrypt: eax_aes128_decrypt,
    digest: eax_aes128_digest,
};

// These would be actual implementations of the functions
fn eax_aes128_set_key(_ctx: &mut EaxAes128Ctx, _key: &[u8]) {}
fn eax_aes128_set_nonce(_ctx: &mut EaxAes128Ctx, _length: usize, _nonce: &[u8]) {}
fn eax_aes128_update(_ctx: &mut EaxAes128Ctx, _data: &[u8]) {}
fn eax_aes128_encrypt(_ctx: &mut EaxAes128Ctx, _length: usize, _dst: &mut [u8], _src: &[u8]) {}
fn eax_aes128_decrypt(_ctx: &mut EaxAes128Ctx, _length: usize, _dst: &mut [u8], _src: &[u8]) {}
fn eax_aes128_digest(_ctx: &mut EaxAes128Ctx, _length: usize, _digest: &mut [u8]) {}