// hmac-sha256-meta.rs

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

use sha2::{Sha256, Digest};
use hmac::{Hmac, Mac};

pub struct HmacSha256 {
    hmac: Hmac<Sha256>,
}

impl HmacSha256 {
    pub fn new() -> Self {
        HmacSha256 {
            hmac: Hmac::new_from_slice(&[]).expect("HMAC can take key of any size"),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        self.hmac = Hmac::new_from_slice(key).expect("HMAC can take key of any size");
    }
}

pub fn hmac_sha256_set_key_wrapper(ctx: &mut HmacSha256, key: &[u8]) {
    ctx.set_key(key);
}

pub const NETTLE_HMAC_SHA256: HmacSha256 = HmacSha256::new();