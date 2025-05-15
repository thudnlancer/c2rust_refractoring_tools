// siv-gcm-aes128.rs
//
// AES-GCM-SIV, RFC8452
//
// Ported from GNU Nettle's C implementation to Rust
//
// Original license:
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

use aes::Aes128;
use cipher::{KeyInit, BlockEncrypt};
use std::result::Result;

pub struct Aes128Ctx {
    cipher: Aes128,
}

impl Aes128Ctx {
    pub fn new(key: &[u8; 16]) -> Self {
        Aes128Ctx {
            cipher: Aes128::new_from_slice(key).unwrap(),
        }
    }
}

pub fn siv_gcm_aes128_encrypt_message(
    ctx: &Aes128Ctx,
    nlength: usize,
    nonce: &[u8],
    alength: usize,
    adata: &[u8],
    clength: usize,
    dst: &mut [u8],
    src: &[u8],
) -> Result<(), &'static str> {
    let mut ctr_ctx = Aes128Ctx::new(&[0u8; 16]); // Initialize with zero key, will be set by siv_gcm_encrypt_message
    siv_gcm_encrypt_message(
        &Aes128::new_from_slice,
        ctx,
        &mut ctr_ctx,
        nlength,
        nonce,
        alength,
        adata,
        clength,
        dst,
        src,
    )
}

pub fn siv_gcm_aes128_decrypt_message(
    ctx: &Aes128Ctx,
    nlength: usize,
    nonce: &[u8],
    alength: usize,
    adata: &[u8],
    mlength: usize,
    dst: &mut [u8],
    src: &[u8],
) -> Result<(), &'static str> {
    let mut ctr_ctx = Aes128Ctx::new(&[0u8; 16]); // Initialize with zero key, will be set by siv_gcm_decrypt_message
    siv_gcm_decrypt_message(
        &Aes128::new_from_slice,
        ctx,
        &mut ctr_ctx,
        nlength,
        nonce,
        alength,
        adata,
        mlength,
        dst,
        src,
    )
}