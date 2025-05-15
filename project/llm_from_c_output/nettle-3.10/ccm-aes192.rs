// ccm-aes192.rs
// Counter with CBC-MAC mode using AES192 as the underlying cipher.

// Copyright (C) 2014 Exegin Technologies Limited
// Copyright (C) 2014 Owen Kirby

// This file is part of GNU Nettle.

// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:

//   * the GNU Lesser General Public License as published by the Free
//     Software Foundation; either version 3 of the License, or (at your
//     option) any later version.

// or

//   * the GNU General Public License as published by the Free
//     Software Foundation; either version 2 of the License, or (at your
//     option) any later version.

// or both in parallel, as here.

// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use aes::Aes192;
use ccm::{Ccm, CcmContext};
use block_cipher::BlockCipher;
use generic_array::GenericArray;

pub struct CcmAes192Ctx {
    cipher: Aes192,
    ccm: CcmContext,
}

impl CcmAes192Ctx {
    pub fn new() -> Self {
        Self {
            cipher: Aes192::new(GenericArray::default()),
            ccm: CcmContext::new(),
        }
    }

    pub fn set_key(&mut self, key: &[u8; 24]) {
        self.cipher = Aes192::new(GenericArray::from_slice(key));
    }

    pub fn set_nonce(
        &mut self,
        length: usize,
        nonce: &[u8],
        authlen: usize,
        msglen: usize,
        taglen: usize,
    ) {
        self.ccm.set_nonce(
            &self.cipher,
            length,
            nonce,
            authlen,
            msglen,
            taglen,
        );
    }

    pub fn update(&mut self, length: usize, data: &[u8]) {
        self.ccm.update(&self.cipher, length, data);
    }

    pub fn encrypt(&mut self, length: usize, dst: &mut [u8], src: &[u8]) {
        self.ccm.encrypt(&self.cipher, length, dst, src);
    }

    pub fn decrypt(&mut self, length: usize, dst: &mut [u8], src: &[u8]) {
        self.ccm.decrypt(&self.cipher, length, dst, src);
    }

    pub fn digest(&mut self, length: usize, digest: &mut [u8]) {
        self.ccm.digest(&self.cipher, length, digest);
    }

    pub fn encrypt_message(
        &mut self,
        nlength: usize,
        nonce: &[u8],
        alength: usize,
        adata: &[u8],
        tlength: usize,
        clength: usize,
        dst: &mut [u8],
        src: &[u8],
    ) {
        self.ccm.encrypt_message(
            &self.cipher,
            nlength,
            nonce,
            alength,
            adata,
            tlength,
            clength,
            dst,
            src,
        );
    }

    pub fn decrypt_message(
        &mut self,
        nlength: usize,
        nonce: &[u8],
        alength: usize,
        adata: &[u8],
        tlength: usize,
        mlength: usize,
        dst: &mut [u8],
        src: &[u8],
    ) -> Result<(), &'static str> {
        self.ccm.decrypt_message(
            &self.cipher,
            nlength,
            nonce,
            alength,
            adata,
            tlength,
            mlength,
            dst,
            src,
        )
    }
}