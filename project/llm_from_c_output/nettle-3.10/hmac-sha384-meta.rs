/* hmac-sha384-meta.rs

   Copyright (C) 2020 Daiki Ueno

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

use sha2::{Sha384, Digest};
use hmac::{Hmac, Mac};
use std::convert::TryInto;

type HmacSha384 = Hmac<Sha384>;

pub struct NettleHmacSha384 {
    hmac: HmacSha384,
}

impl NettleHmacSha384 {
    pub fn new() -> Self {
        NettleHmacSha384 {
            hmac: HmacSha384::new_from_slice(&[]).expect("HMAC initialization failed"),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        self.hmac = HmacSha384::new_from_slice(key).expect("HMAC key initialization failed");
    }

    pub fn update(&mut self, data: &[u8]) {
        self.hmac.update(data);
    }

    pub fn digest(&mut self, output: &mut [u8]) -> Result<(), String> {
        let result = self.hmac.finalize_reset();
        let code_bytes = result.into_bytes();
        if output.len() < code_bytes.len() {
            return Err("Output buffer too small".to_string());
        }
        output[..code_bytes.len()].copy_from_slice(&code_bytes);
        Ok(())
    }
}

pub const NETTLE_HMAC_SHA384: NettleHmacSha384 = NettleHmacSha384 {
    hmac: HmacSha384::new_from_slice(&[]).expect("HMAC initialization failed"),
};