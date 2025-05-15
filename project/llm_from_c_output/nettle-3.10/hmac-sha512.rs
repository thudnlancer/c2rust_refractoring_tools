// hmac-sha512.rs
//
// HMAC-SHA512 message authentication code.
//
// Original C code Copyright (C) 2003, 2010 Niels Möller
//
// This Rust translation is free software: you can redistribute it and/or
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
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

use sha2::{Sha512, Digest};
use hmac::{Hmac, Mac};

type HmacSha512 = Hmac<Sha512>;

pub struct HmacSha512Ctx {
    inner: HmacSha512,
}

impl HmacSha512Ctx {
    pub fn new() -> Self {
        HmacSha512Ctx {
            inner: HmacSha512::new_from_slice(&[]).expect("HMAC initialization should not fail"),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        self.inner = HmacSha512::new_from_slice(key)
            .expect("HMAC key initialization should not fail");
    }

    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    pub fn digest(&mut self) -> Vec<u8> {
        let result = self.inner.finalize_reset();
        result.into_bytes().to_vec()
    }
}