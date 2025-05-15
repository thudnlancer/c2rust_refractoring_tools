// gosthash94-meta.rs

// Copyright (C) 2012 Nikos Mavrogiannopoulos, Niels Möller
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

use crate::nettle_meta::NettleHash;
use crate::gosthash94::{Gosthash94, Gosthash94cp};

pub const NETTLE_GOSTHASH94: NettleHash = NettleHash {
    name: "gosthash94",
    context_size: std::mem::size_of::<Gosthash94>(),
    digest_size: GOSTHASH94_DIGEST_SIZE,
    block_size: GOSTHASH94_BLOCK_SIZE,
    init: Gosthash94::init,
    update: Gosthash94::update,
    digest: Gosthash94::digest,
};

pub const NETTLE_GOSTHASH94CP: NettleHash = NettleHash {
    name: "gosthash94cp",
    context_size: std::mem::size_of::<Gosthash94cp>(),
    digest_size: GOSTHASH94_DIGEST_SIZE,
    block_size: GOSTHASH94_BLOCK_SIZE,
    init: Gosthash94cp::init,
    update: Gosthash94cp::update,
    digest: Gosthash94cp::digest,
};

pub const GOSTHASH94_DIGEST_SIZE: usize = 32;
pub const GOSTHASH94_BLOCK_SIZE: usize = 32;