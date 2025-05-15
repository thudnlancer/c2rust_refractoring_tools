// hmac-sha224.rs

// HMAC-SHA224 message authentication code.

// Copyright (C) 2003, 2010 Niels Möller

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

use sha2::{Sha224, Digest};
use hmac::{Hmac, Mac};

type HmacSha224 = Hmac<Sha224>;

pub struct HmacSha224Ctx {
    inner: HmacSha224,
}

impl HmacSha224Ctx {
    pub fn new() -> Self {
        HmacSha224Ctx {
            inner: HmacSha224::new_from_slice(&[]).expect("HMAC can take key of any size"),
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        self.inner = HmacSha224::new_from_slice(key).expect("HMAC can take key of any size");
    }

    pub fn digest(&mut self, data: &[u8]) -> Vec<u8> {
        self.inner.update(data);
        self.inner.finalize_reset().into_bytes().to_vec()
    }
}

pub fn hmac_sha224_set_key(ctx: &mut HmacSha224Ctx, key: &[u8]) {
    ctx.set_key(key);
}

pub fn hmac_sha224_digest(ctx: &mut HmacSha224Ctx, data: &[u8]) -> Vec<u8> {
    ctx.digest(data)
}