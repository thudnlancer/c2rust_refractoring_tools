// gcm-aes128.rs
// Galois counter mode using AES128 as the underlying cipher.

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

use aes::Aes128;
use cipher::{BlockCipher, BlockEncrypt, NewBlockCipher};
use gcm::{Gcm, GcmConfig};

pub struct GcmAes128 {
    inner: Gcm<Aes128>,
}

impl GcmAes128 {
    pub fn new(key: &[u8]) -> Self {
        let cipher = Aes128::new_from_slice(key).expect("Invalid key length");
        let config = GcmConfig::default();
        Self {
            inner: Gcm::new(cipher, config),
        }
    }

    pub fn set_iv(&mut self, iv: &[u8]) {
        self.inner.set_iv(iv);
    }

    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    pub fn encrypt(&mut self, src: &[u8], dst: &mut [u8]) {
        let done = self.inner.encrypt(src, dst);
        self.inner.data_size += done;
        let remaining = src.len() - done;
        if remaining > 0 {
            self.inner.encrypt(&src[done..], &mut dst[done..]);
        }
    }

    pub fn decrypt(&mut self, src: &[u8], dst: &mut [u8]) {
        let done = self.inner.decrypt(src, dst);
        self.inner.data_size += done;
        let remaining = src.len() - done;
        if remaining > 0 {
            self.inner.decrypt(&src[done..], &mut dst[done..]);
        }
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        self.inner.digest(digest);
    }
}