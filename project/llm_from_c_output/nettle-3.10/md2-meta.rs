// md2-meta.rs

// Copyright (C) 2003 Niels Möller
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
use crate::md2::Md2;

pub const NETTLE_MD2: NettleHash = NettleHash {
    name: "md2",
    context_size: std::mem::size_of::<Md2>(),
    digest_size: Md2::DIGEST_SIZE,
    block_size: Md2::BLOCK_SIZE,
    init: Md2::new,
    update: Md2::update,
    digest: Md2::digest,
};