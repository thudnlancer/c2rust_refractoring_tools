// hmac-sha512-meta.rs

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

use sha2::{Sha512, Digest};
use hmac::{Hmac, Mac};

pub struct HmacSha512 {
    hmac: Hmac<Sha512>,
}

impl HmacSha512 {
    pub fn new() -> Self {
        HmacSha512 {
            hmac: Hmac::<Sha512>::new_from_slice(&[]).unwrap(),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) -> Result<(), String> {
        self.hmac = Hmac::<Sha512>::new_from_slice(key)
            .map_err(|e| format!("Invalid key length: {}", e))?;
        Ok(())
    }

    pub fn update(&mut self, data: &[u8]) {
        self.hmac.update(data);
    }

    pub fn finalize(self) -> [u8; 64] {
        let result = self.hmac.finalize();
        let mut output = [0u8; 64];
        output.copy_from_slice(&result.into_bytes());
        output
    }
}

pub const HMAC_SHA512: HmacSha512 = HmacSha512 {
    hmac: Hmac::<Sha512>::new_from_slice(&[]).unwrap(),
};