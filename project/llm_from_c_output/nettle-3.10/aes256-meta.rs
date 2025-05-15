/* aes256-meta.rs

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
use crate::aes::{Aes256Ctx, AES_BLOCK_SIZE, AES256_KEY_SIZE};

pub struct NettleCipher {
    pub name: &'static str,
    pub context_size: usize,
    pub block_size: usize,
    pub key_size: usize,
    pub set_encrypt_key: fn(&mut Aes256Ctx, &[u8]) -> Result<(), String>,
    pub set_decrypt_key: fn(&mut Aes256Ctx, &[u8]) -> Result<(), String>,
    pub encrypt: fn(&Aes256Ctx, &mut [u8], &[u8]),
    pub decrypt: fn(&Aes256Ctx, &mut [u8], &[u8]),
}

pub const NETTLE_AES256: NettleCipher = NettleCipher {
    name: "aes256",
    context_size: size_of::<Aes256Ctx>(),
    block_size: AES_BLOCK_SIZE,
    key_size: AES256_KEY_SIZE,
    set_encrypt_key: aes256_set_encrypt_key,
    set_decrypt_key: aes256_set_decrypt_key,
    encrypt: aes256_encrypt,
    decrypt: aes256_decrypt,
};