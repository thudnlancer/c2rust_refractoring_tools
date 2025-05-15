/* sha512-256-meta.rs

   Copyright (C) 2014 Niels Möller

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

use std::mem::size_of;
use sha2::{Sha512_256, Digest};

pub struct NettleHash {
    pub name: &'static str,
    pub context_size: usize,
    pub digest_size: usize,
    pub block_size: usize,
    pub init: fn() -> Sha512_256,
    pub update: fn(&mut Sha512_256, &[u8]),
    pub digest: fn(Sha512_256, &mut [u8]),
}

fn sha512_256_init() -> Sha512_256 {
    Sha512_256::new()
}

fn sha512_256_update(hasher: &mut Sha512_256, data: &[u8]) {
    hasher.update(data);
}

fn sha512_256_digest(hasher: Sha512_256, digest: &mut [u8]) {
    let result = hasher.finalize();
    digest.copy_from_slice(&result);
}

pub const NETTLE_SHA512_256: NettleHash = NettleHash {
    name: "sha512_256",
    context_size: size_of::<Sha512_256>(),
    digest_size: 32, // SHA512_256_DIGEST_SIZE
    block_size: 128, // SHA512_256_BLOCK_SIZE
    init: sha512_256_init,
    update: sha512_256_update,
    digest: sha512_256_digest,
};