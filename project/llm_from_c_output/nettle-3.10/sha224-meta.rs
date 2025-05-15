// sha224-meta.rs

// Copyright (C) 2002, 2010 Niels Möller
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

pub struct NettleHash {
    pub name: &'static str,
    pub context_size: usize,
    pub digest_size: usize,
    pub block_size: usize,
    pub init: fn() -> Sha224,
    pub update: fn(context: &mut Sha224, length: usize, data: &[u8]),
    pub digest: fn(context: &mut Sha224, length: usize, digest: &mut [u8]),
}

pub const NETTLE_SHA224: NettleHash = NettleHash {
    name: "sha224",
    context_size: std::mem::size_of::<Sha224>(),
    digest_size: sha2::Sha224::output_size(),
    block_size: sha2::Sha224::block_size(),
    init: || Sha224::new(),
    update: |context, length, data| {
        context.update(&data[..length]);
    },
    digest: |context, length, digest| {
        let result = context.finalize_reset();
        digest[..length].copy_from_slice(&result[..length]);
    },
};