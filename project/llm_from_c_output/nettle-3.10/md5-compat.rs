// md5_compat.rs

// The md5 hash function, RFC 1321-style interface.

// Copyright (C) 2001 Niels Möller

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

use md5::{Md5, Digest};
use std::convert::TryInto;

pub struct MD5_CTX {
    hasher: Md5,
}

impl MD5_CTX {
    pub fn new() -> Self {
        MD5_CTX {
            hasher: Md5::new(),
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.hasher.update(data);
    }

    pub fn finalize(self, out: &mut [u8; 16]) {
        let result = self.hasher.finalize();
        out.copy_from_slice(&result);
    }
}

pub fn md5_init() -> MD5_CTX {
    MD5_CTX::new()
}

pub fn md5_update(ctx: &mut MD5_CTX, data: &[u8]) {
    ctx.update(data);
}

pub fn md5_digest(ctx: MD5_CTX, out: &mut [u8; 16]) {
    ctx.finalize(out);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5_compat() {
        let mut ctx = md5_init();
        let data = b"hello world";
        md5_update(&mut ctx, data);
        let mut out = [0u8; 16];
        md5_digest(ctx, &mut out);
        assert_eq!(hex::encode(out), "5eb63bbbe01eeed093cb22bb8f5acdc3");
    }
}