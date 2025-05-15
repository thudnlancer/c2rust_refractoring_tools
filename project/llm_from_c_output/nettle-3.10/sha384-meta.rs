// sha384-meta.rs

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

use sha2::{Sha384, Digest};

pub struct NettleHash {
    pub name: &'static str,
    pub context_size: usize,
    pub digest_size: usize,
    pub block_size: usize,
    pub init: fn() -> Box<dyn Digest>,
    pub update: fn(digest: &mut Box<dyn Digest>, data: &[u8]),
    pub digest: fn(digest: &mut Box<dyn Digest>, result: &mut [u8]),
}

pub const NETTLE_SHA384: NettleHash = NettleHash {
    name: "sha384",
    context_size: std::mem::size_of::<Sha384>(),
    digest_size: 48, // SHA384 output is 48 bytes
    block_size: 128, // SHA384 block size is 128 bytes
    init: || Box::new(Sha384::new()),
    update: |digest, data| {
        if let Some(sha) = digest.as_mut().downcast_mut::<Sha384>() {
            sha.update(data);
        }
    },
    digest: |digest, result| {
        if let Some(sha) = digest.as_mut().downcast_mut::<Sha384>() {
            let hash = sha.finalize_reset();
            result.copy_from_slice(&hash);
        }
    },
};