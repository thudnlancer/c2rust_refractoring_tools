// gcm-camellia256.rs

// Copyright (C) 2011, 2014 Niels Möller
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

use crate::gcm::Gcm;
use camellia::Camellia256;

pub struct GcmCamellia256 {
    inner: Gcm<Camellia256>,
}

impl GcmCamellia256 {
    pub fn new() -> Self {
        GcmCamellia256 {
            inner: Gcm::new(Camellia256::new()),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) -> Result<(), &'static str> {
        self.inner.set_key(key)
    }

    pub fn set_iv(&mut self, iv: &[u8]) -> Result<(), &'static str> {
        self.inner.set_iv(iv)
    }

    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    pub fn encrypt(&mut self, src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
        self.inner.encrypt(src, dst)
    }

    pub fn decrypt(&mut self, src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
        self.inner.decrypt(src, dst)
    }

    pub fn digest(&mut self, digest: &mut [u8]) -> Result<(), &'static str> {
        self.inner.digest(digest)
    }
}