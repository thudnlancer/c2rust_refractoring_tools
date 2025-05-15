/* sha512-224-meta.rs

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
use crate::sha2::{Sha512_224, sha512_224_init, sha512_224_update, sha512_224_digest};

pub const SHA512_224_DIGEST_SIZE: usize = 28;
pub const SHA512_224_BLOCK_SIZE: usize = 128;

pub struct NettleHash {
    pub name: &'static str,
    pub context_size: usize,
    pub digest_size: usize,
    pub block_size: usize,
    pub init: fn(&mut Sha512_224),
    pub update: fn(&mut Sha512_224, data: &[u8]),
    pub digest: fn(&mut Sha512_224, digest: &mut [u8]),
}

pub const NETTLE_SHA512_224: NettleHash = NettleHash {
    name: "sha512_224",
    context_size: size_of::<Sha512_224>(),
    digest_size: SHA512_224_DIGEST_SIZE,
    block_size: SHA512_224_BLOCK_SIZE,
    init: sha512_224_init,
    update: sha512_224_update,
    digest: sha512_224_digest,
};