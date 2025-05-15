// hmac-sha224-meta.rs

// Copyright (C) 2020 Daiki Ueno
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

use sha2::{Sha224, Digest};
use hmac::{Hmac, Mac};

pub struct HmacSha224 {
    inner: Hmac<Sha224>,
}

impl HmacSha224 {
    pub fn new() -> Self {
        HmacSha224 {
            inner: Hmac::new_from_slice(&[]).expect("HMAC can take key of any size"),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        self.inner = Hmac::new_from_slice(key).expect("HMAC can take key of any size");
    }

    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    pub fn finalize(self) -> [u8; 28] {
        let result = self.inner.finalize();
        let mut output = [0u8; 28];
        output.copy_from_slice(&result.into_bytes()[..28]);
        output
    }
}

pub const HMAC_SHA224: HmacSha224 = HmacSha224 {
    inner: Hmac::new_from_slice(&[]).expect("HMAC can take key of any size"),
};