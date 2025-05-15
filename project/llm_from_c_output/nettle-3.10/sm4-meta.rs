/* sm4-meta.rs

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
use crate::sm4::{Sm4Ctx, SM4_BLOCK_SIZE, SM4_KEY_SIZE};

pub struct NettleCipher {
    pub name: &'static str,
    pub context_size: usize,
    pub block_size: usize,
    pub key_size: usize,
    pub set_encrypt_key: fn(&mut Sm4Ctx, &[u8]) -> Result<(), String>,
    pub set_decrypt_key: fn(&mut Sm4Ctx, &[u8]) -> Result<(), String>,
    pub encrypt: fn(&Sm4Ctx, &mut [u8], &[u8]) -> Result<(), String>,
    pub decrypt: fn(&Sm4Ctx, &mut [u8], &[u8]) -> Result<(), String>,
}

pub const NETTLE_SM4: NettleCipher = NettleCipher {
    name: "sm4",
    context_size: size_of::<Sm4Ctx>(),
    block_size: SM4_BLOCK_SIZE,
    key_size: SM4_KEY_SIZE,
    set_encrypt_key: sm4_set_encrypt_key,
    set_decrypt_key: sm4_set_decrypt_key,
    encrypt: sm4_crypt,
    decrypt: sm4_crypt,
};