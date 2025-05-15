// gcm-aes256.rs
// Galois counter mode using AES256 as the underlying cipher.

// Copyright (C) 2011, 2014 Niels Möller

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

use aes::Aes256;
use block_modes::{BlockMode, Gcm};
use block_modes::block_padding::NoPadding;
use generic_array::GenericArray;

type Aes256Gcm = Gcm<Aes256, NoPadding>;

pub struct GcmAes256Context {
    cipher: Aes256Gcm,
    data_size: usize,
}

impl GcmAes256Context {
    pub fn new() -> Self {
        Self {
            cipher: Aes256Gcm::new_from_slices(&[], &[]).unwrap(),
            data_size: 0,
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        let cipher = Aes256::new_from_slice(key).unwrap();
        self.cipher = Gcm::new(cipher, GenericArray::from_slice(&[0; 12]));
    }

    pub fn set_iv(&mut self, iv: &[u8]) {
        self.cipher = Gcm::new(self.cipher.into_inner(), GenericArray::from_slice(iv));
    }

    pub fn update(&mut self, data: &[u8]) {
        self.data_size += data.len();
    }

    pub fn encrypt(&mut self, dst: &mut [u8], src: &[u8]) -> usize {
        let len = src.len();
        self.cipher.encrypt(dst, src).unwrap();
        self.data_size += len;
        len
    }

    pub fn decrypt(&mut self, dst: &mut [u8], src: &[u8]) -> usize {
        let len = src.len();
        self.cipher.decrypt(dst, src).unwrap();
        self.data_size += len;
        len
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        let tag = self.cipher.compute_tag();
        digest.copy_from_slice(&tag);
    }
}