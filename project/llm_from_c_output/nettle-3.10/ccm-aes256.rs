/*
   Counter with CBC-MAC mode using AES256 as the underlying cipher.

   Copyright (C) 2014 Exegin Technologies Limited
   Copyright (C) 2014 Owen Kirby

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
use ccm::aead::{Aead, NewAead, generic_array::GenericArray};

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub struct CcmAes256Context {
    cipher: Aes256Cbc,
    ccm: ccm::Ccm<Aes256>,
}

impl CcmAes256Context {
    pub fn new() -> Self {
        Self {
            cipher: Aes256Cbc::new_from_slices(&[0; 32], &[0; 16]).unwrap(),
            ccm: ccm::Ccm::new(GenericArray::from_slice(&[0; 32])),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        let key_array = GenericArray::from_slice(key);
        self.cipher = Aes256Cbc::new_from_slices(key, &[0; 16]).unwrap();
        self.ccm = ccm::Ccm::new(key_array);
    }

    pub fn set_nonce(&mut self, nonce: &[u8], auth_len: usize, msg_len: usize, tag_len: usize) {
        self.ccm.set_nonce(nonce, auth_len, msg_len, tag_len);
    }

    pub fn update(&mut self, data: &[u8]) {
        self.ccm.update(data);
    }

    pub fn encrypt(&mut self, dst: &mut [u8], src: &[u8]) {
        self.ccm.encrypt(dst, src);
    }

    pub fn decrypt(&mut self, dst: &mut [u8], src: &[u8]) -> Result<(), ccm::Error> {
        self.ccm.decrypt(dst, src)
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        self.ccm.digest(digest);
    }

    pub fn encrypt_message(
        &mut self,
        nonce: &[u8],
        adata: &[u8],
        tag_len: usize,
        dst: &mut [u8],
        src: &[u8],
    ) {
        self.ccm.encrypt_message(nonce, adata, tag_len, dst, src);
    }

    pub fn decrypt_message(
        &mut self,
        nonce: &[u8],
        adata: &[u8],
        tag_len: usize,
        dst: &mut [u8],
        src: &[u8],
    ) -> Result<(), ccm::Error> {
        self.ccm.decrypt_message(nonce, adata, tag_len, dst, src)
    }
}