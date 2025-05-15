// md4-meta.rs

// Copyright (C) 2002 Niels Möller
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
use crate::md4::Md4;

pub const NETTLE_MD4: NettleHash = NettleHash {
    name: "md4",
    context_size: std::mem::size_of::<Md4>(),
    digest_size: Md4::DIGEST_SIZE,
    block_size: Md4::BLOCK_SIZE,
    init: Md4::new,
    update: Md4::update,
    digest: Md4::digest,
};