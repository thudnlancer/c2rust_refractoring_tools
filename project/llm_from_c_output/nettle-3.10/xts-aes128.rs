/*
   XTS Mode using AES128 as the underlying cipher.

   Copyright (C) 2018 Red Hat, Inc.

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
use cipher::{BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit};
use xts_mode::{Xts128, XtsMode};

pub struct XtsAes128Key {
    cipher: Aes128,
    tweak_cipher: Aes128,
}

impl XtsAes128Key {
    pub fn new_encrypt_key(key: &[u8; 32]) -> Self {
        let cipher = Aes128::new_from_slice(&key[..16]).unwrap();
        let tweak_cipher = Aes128::new_from_slice(&key[16..]).unwrap();
        Self { cipher, tweak_cipher }
    }

    pub fn new_decrypt_key(key: &[u8; 32]) -> Self {
        let cipher = Aes128::new_from_slice(&key[..16]).unwrap();
        let tweak_cipher = Aes128::new_from_slice(&key[16..]).unwrap();
        Self { cipher, tweak_cipher }
    }

    pub fn encrypt_message(&self, tweak: &[u8; 16], data: &[u8]) -> Vec<u8> {
        let mut xts = Xts128::<Aes128>::new(self.cipher.clone(), self.tweak_cipher.clone());
        let mut buffer = data.to_vec();
        xts.encrypt_area(tweak, &mut buffer).unwrap();
        buffer
    }

    pub fn decrypt_message(&self, tweak: &[u8; 16], data: &[u8]) -> Vec<u8> {
        let mut xts = Xts128::<Aes128>::new(self.cipher.clone(), self.tweak_cipher.clone());
        let mut buffer = data.to_vec();
        xts.decrypt_area(tweak, &mut buffer).unwrap();
        buffer
    }
}