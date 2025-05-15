/* camellia256-meta.rs

   Copyright (C) 2010, 2013, 2014 Niels Möller

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
use crate::camellia::{Camellia256Ctx, CAMELLIA_BLOCK_SIZE, CAMELLIA256_KEY_SIZE};

pub struct NettleCipher {
    pub name: &'static str,
    pub context_size: usize,
    pub block_size: usize,
    pub key_size: usize,
    pub set_encrypt_key: fn(&mut Camellia256Ctx, &[u8]) -> Result<(), String>,
    pub set_decrypt_key: fn(&mut Camellia256Ctx, &[u8]) -> Result<(), String>,
    pub encrypt: fn(&Camellia256Ctx, &mut [u8], &[u8]) -> Result<(), String>,
    pub decrypt: fn(&Camellia256Ctx, &mut [u8], &[u8]) -> Result<(), String>,
}

pub const NETTLE_CAMELLIA256: NettleCipher = NettleCipher {
    name: "camellia256",
    context_size: size_of::<Camellia256Ctx>(),
    block_size: CAMELLIA_BLOCK_SIZE,
    key_size: CAMELLIA256_KEY_SIZE,
    set_encrypt_key: camellia256_set_encrypt_key,
    set_decrypt_key: camellia256_set_decrypt_key,
    encrypt: camellia256_crypt,
    decrypt: camellia256_crypt,
};

// These functions should be defined elsewhere in the camellia module
fn camellia256_set_encrypt_key(_ctx: &mut Camellia256Ctx, _key: &[u8]) -> Result<(), String> {
    unimplemented!()
}

fn camellia256_set_decrypt_key(_ctx: &mut Camellia256Ctx, _key: &[u8]) -> Result<(), String> {
    unimplemented!()
}

fn camellia256_crypt(_ctx: &Camellia256Ctx, _dst: &mut [u8], _src: &[u8]) -> Result<(), String> {
    unimplemented!()
}