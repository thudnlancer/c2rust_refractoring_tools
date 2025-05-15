/*
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

use aes::Aes128;
use block_modes::{BlockMode, Eax};
use block_modes::block_padding::Pkcs7;
use generic_array::GenericArray;

type Aes128Eax = Eax<Aes128, Pkcs7>;

pub struct EaxAes128Ctx {
    cipher: Aes128Eax,
}

impl EaxAes128Ctx {
    pub fn set_key(&mut self, key: &[u8; 16]) -> Result<(), block_modes::InvalidKeyIvLength> {
        let cipher = Aes128Eax::new_from_slices(key, &[0; 16])?;
        self.cipher = cipher;
        Ok(())
    }

    pub fn set_nonce(&mut self, iv: &[u8]) -> Result<(), block_modes::InvalidKeyIvLength> {
        let cipher = Aes128Eax::new_from_slices(&[0; 16], iv)?;
        self.cipher = cipher;
        Ok(())
    }

    pub fn update(&mut self, data: &[u8]) {
        // EAX mode doesn't support incremental updates in the same way as C version
        // This would need to be implemented differently for a real application
    }

    pub fn encrypt(&mut self, dst: &mut [u8], src: &[u8]) -> Result<usize, block_modes::BlockModeError> {
        let ciphertext = self.cipher.encrypt_vec(src);
        dst.copy_from_slice(&ciphertext);
        Ok(ciphertext.len())
    }

    pub fn decrypt(&mut self, dst: &mut [u8], src: &[u8]) -> Result<usize, block_modes::BlockModeError> {
        let plaintext = self.cipher.decrypt_vec(src)?;
        dst.copy_from_slice(&plaintext);
        Ok(plaintext.len())
    }

    pub fn digest(&self, digest: &mut [u8]) {
        // In EAX mode, the tag is produced during encryption
        // This would need to be implemented according to specific digest requirements
    }
}