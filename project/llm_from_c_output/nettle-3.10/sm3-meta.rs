// sm3-meta.rs

// Copyright (C) 2017 Jia Zhang
// Copyright (C) 2021 Tianjia Zhang <tianjia.zhang@linux.alibaba.com>

// This file is part of GNU Nettle.

// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:

//   * the GNU Lesser General Public License as published by the Free
//     Software Foundation; either version 3 of the License, or (at your
//     option) any later version.

// or

//   * the GNU General Public License as published by the Free
//     Software Foundation; either version 2 of the License, or (at your
//     option) any later version.

// or both in parallel, as here.

// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use crate::nettle_meta::NettleHash;
use crate::sm3::Sm3;

pub const NETTLE_SM3: NettleHash = NettleHash {
    name: "sm3",
    context_size: std::mem::size_of::<Sm3>(),
    digest_size: Sm3::DIGEST_SIZE,
    block_size: Sm3::BLOCK_SIZE,
    init: Sm3::init,
    update: Sm3::update,
    digest: Sm3::digest,
};