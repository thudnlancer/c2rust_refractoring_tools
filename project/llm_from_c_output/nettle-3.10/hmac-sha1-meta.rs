// hmac-sha1-meta.rs

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

use sha1::{Sha1, Digest};
use hmac::{Hmac, Mac};

pub struct HmacSha1 {
    inner: Hmac<Sha1>,
}

impl HmacSha1 {
    pub fn new() -> Self {
        HmacSha1 {
            inner: Hmac::new_from_slice(&[]).expect("HMAC can accept any key size"),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        self.inner = Hmac::new_from_slice(key).expect("HMAC can accept any key size");
    }

    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    pub fn finalize(self) -> [u8; 20] {
        let result = self.inner.finalize();
        result.into_bytes().into()
    }
}

pub const HMAC_SHA1: HmacSha1 = HmacSha1::new();