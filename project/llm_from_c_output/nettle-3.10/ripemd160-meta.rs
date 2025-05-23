// ripemd160_meta.rs

// Copyright (C) 2011 Andres Mejia
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
use ripemd160::Ripemd160;

pub const NETTLE_RIPEMD160: NettleHash = NettleHash {
    name: "ripemd160",
    context_size: std::mem::size_of::<Ripemd160>(),
    digest_size: Ripemd160::output_size(),
    block_size: Ripemd160::block_size(),
};