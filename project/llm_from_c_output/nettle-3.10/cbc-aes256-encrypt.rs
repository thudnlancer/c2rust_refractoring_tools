/*
   Copyright (C) 2021 Niels Möller

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

use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use std::convert::TryInto;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub struct Aes256Ctx {
    cipher: Aes256Cbc,
}

impl Aes256Ctx {
    pub fn new(key: &[u8], iv: &[u8]) -> Result<Self, block_modes::InvalidKeyIvLength> {
        let cipher = Aes256::new_from_slice(key)?;
        Ok(Self {
            cipher: Cbc::new(cipher, iv.try_into().unwrap()),
        })
    }

    pub fn encrypt(&mut self, data: &[u8]) -> Result<Vec<u8>, block_modes::BlockModeError> {
        self.cipher.encrypt_vec(data)
    }
}

pub fn cbc_aes256_encrypt(
    ctx: &mut Aes256Ctx,
    iv: &mut [u8],
    src: &[u8],
) -> Result<Vec<u8>, block_modes::BlockModeError> {
    ctx.cipher.reset(iv.try_into().unwrap());
    ctx.encrypt(src)
}