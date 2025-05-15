/*
   AES-SIV, RFC5297

   Copyright (C) 2017 Nikos Mavrogiannopoulos

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
use cmac::{Cmac, Mac};
use ctr::Ctr128;
use cipher::{BlockEncrypt, NewBlockCipher, StreamCipher};
use std::convert::TryInto;

pub struct SivCmacAes256Ctx {
    cmac_key: Cmac<Aes256>,
    cmac_cipher: Aes256,
    ctr_cipher: Aes256,
}

impl SivCmacAes256Ctx {
    pub fn new(key: &[u8; 32]) -> Self {
        let cmac_cipher = Aes256::new(key.into());
        let ctr_cipher = Aes256::new(key.into());
        let cmac_key = Cmac::<Aes256>::new(key.into());
        
        Self {
            cmac_key,
            cmac_cipher,
            ctr_cipher,
        }
    }

    pub fn encrypt_message(
        &self,
        nonce: &[u8],
        adata: &[u8],
        src: &[u8],
        dst: &mut [u8],
    ) -> Result<(), &'static str> {
        if dst.len() < src.len() {
            return Err("Destination buffer too small");
        }

        siv_cmac_encrypt_message(
            &self.cmac_key,
            &self.cmac_cipher,
            &self.ctr_cipher,
            nonce,
            adata,
            src,
            dst,
        )
    }

    pub fn decrypt_message(
        &self,
        nonce: &[u8],
        adata: &[u8],
        src: &[u8],
        dst: &mut [u8],
    ) -> Result<(), &'static str> {
        if dst.len() < src.len() {
            return Err("Destination buffer too small");
        }

        siv_cmac_decrypt_message(
            &self.cmac_key,
            &self.cmac_cipher,
            &self.ctr_cipher,
            nonce,
            adata,
            src,
            dst,
        )
    }
}

fn siv_cmac_encrypt_message(
    cmac_key: &Cmac<Aes256>,
    cmac_cipher: &Aes256,
    ctr_cipher: &Aes256,
    nonce: &[u8],
    adata: &[u8],
    src: &[u8],
    dst: &mut [u8],
) -> Result<(), &'static str> {
    // Implementation of SIV-CMAC encryption
    // ... actual implementation would go here
    Ok(())
}

fn siv_cmac_decrypt_message(
    cmac_key: &Cmac<Aes256>,
    cmac_cipher: &Aes256,
    ctr_cipher: &Aes256,
    nonce: &[u8],
    adata: &[u8],
    src: &[u8],
    dst: &mut [u8],
) -> Result<(), &'static str> {
    // Implementation of SIV-CMAC decryption
    // ... actual implementation would go here
    Ok(())
}