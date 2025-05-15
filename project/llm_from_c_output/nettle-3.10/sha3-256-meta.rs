// sha3-256-meta.rs

// Copyright (C) 2012 Niels Möller
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

use sha3::{Sha3_256, Digest};
use std::fmt;

#[derive(Clone)]
pub struct NettleHash {
    pub name: &'static str,
    pub digest_size: usize,
    pub context_size: usize,
    pub block_size: usize,
    pub init: fn() -> Sha3_256,
    pub update: fn(context: &mut Sha3_256, length: usize, data: &[u8]),
    pub digest: fn(context: &mut Sha3_256, length: usize, digest: &mut [u8]),
}

impl fmt::Debug for NettleHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NettleHash")
            .field("name", &self.name)
            .field("digest_size", &self.digest_size)
            .field("context_size", &self.context_size)
            .field("block_size", &self.block_size)
            .finish()
    }
}

pub const NETTLE_SHA3_256: NettleHash = NettleHash {
    name: "sha3_256",
    digest_size: 32,
    context_size: std::mem::size_of::<Sha3_256>(),
    block_size: 136,
    init: || Sha3_256::new(),
    update: |context, _length, data| {
        context.update(data);
    },
    digest: |context, _length, digest| {
        let result = context.finalize_reset();
        digest.copy_from_slice(&result);
    },
};