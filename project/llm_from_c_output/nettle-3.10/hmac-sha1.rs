// hmac-sha1.rs
// 
// HMAC-SHA1 message authentication code.
// 
// Translated from C code originally by Niels Möller
// 
// Original copyright notice:
// 
// Copyright (C) 2002 Niels Möller
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

type HmacSha1 = Hmac<Sha1>;

pub struct HmacSha1Ctx {
    inner: HmacSha1,
}

impl HmacSha1Ctx {
    pub fn new() -> Self {
        HmacSha1Ctx {
            inner: HmacSha1::new_from_slice(&[]).expect("HMAC initialization failed"),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) -> Result<(), String> {
        self.inner = HmacSha1::new_from_slice(key)
            .map_err(|e| format!("HMAC key initialization failed: {}", e))?;
        Ok(())
    }

    pub fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    pub fn digest(&mut self, digest: &mut [u8]) -> Result<(), String> {
        let result = self.inner.finalize_reset();
        if digest.len() != result.len() {
            return Err(format!(
                "Invalid digest length: expected {}, got {}",
                result.len(),
                digest.len()
            ));
        }
        digest.copy_from_slice(&result);
        Ok(())
    }
}