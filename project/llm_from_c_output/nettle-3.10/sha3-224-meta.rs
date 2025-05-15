/* sha3-224-meta.rs

   Original C code Copyright (C) 2012 Niels Möller

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

use crate::sha3::{Sha3_224, sha3_224_hash};
use crate::nettle_meta::NettleHash;

pub const NETTLE_SHA3_224: NettleHash = NettleHash {
    name: "sha3_224",
    context_size: std::mem::size_of::<Sha3_224>(),
    digest_size: sha3_224_hash::DIGEST_SIZE,
    block_size: sha3_224_hash::BLOCK_SIZE,
    init: Sha3_224::new,
    update: Sha3_224::update,
    digest: Sha3_224::digest,
};