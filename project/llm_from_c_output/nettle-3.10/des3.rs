// des3.rs
// Triple DES cipher. Three key encrypt-decrypt-encrypt.

// Copyright (C) 2001, 2010 Niels Möller
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

use crate::des::{DesContext, des_encrypt, des_decrypt, DES_KEY_SIZE};

pub struct Des3Context {
    des: [DesContext; 3],
}

impl Des3Context {
    /// Sets the triple DES key.
    /// Returns true for good keys and false for weak keys.
    pub fn set_key(&mut self, key: &[u8; 3 * DES_KEY_SIZE]) -> bool {
        let mut is_good = true;
        
        for i in 0..3 {
            let key_slice = &key[i * DES_KEY_SIZE..(i + 1) * DES_KEY_SIZE];
            if !self.des[i].set_key(key_slice.try_into().unwrap()) {
                is_good = false;
            }
        }

        is_good
    }

    /// Encrypts the data using triple DES in EDE mode (encrypt-decrypt-encrypt)
    pub fn encrypt(&self, src: &[u8], dst: &mut [u8]) {
        assert_eq!(src.len(), dst.len());
        let length = src.len();
        
        des_encrypt(&self.des[0], src, dst);
        des_decrypt(&self.des[1], dst, dst);
        des_encrypt(&self.des[2], dst, dst);
    }

    /// Decrypts the data using triple DES in EDE mode (decrypt-encrypt-decrypt)
    pub fn decrypt(&self, src: &[u8], dst: &mut [u8]) {
        assert_eq!(src.len(), dst.len());
        let length = src.len();
        
        des_decrypt(&self.des[2], src, dst);
        des_encrypt(&self.des[1], dst, dst);
        des_decrypt(&self.des[0], dst, dst);
    }
}