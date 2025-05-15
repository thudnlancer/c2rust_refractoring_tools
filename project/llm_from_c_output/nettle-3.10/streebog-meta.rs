// streebog-meta.rs

// Copyright (C) 2020 Dmitry Baryshkov
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

use nettle_hash::NettleHash;
use streebog::{Streebog256, Streebog512};

pub const NETTLE_STREEBOG512: NettleHash = NettleHash {
    name: "streebog512",
    context_size: std::mem::size_of::<Streebog512>(),
    digest_size: Streebog512::DIGEST_SIZE,
    init: Streebog512::new,
    update: Streebog512::update,
    digest: Streebog512::digest,
};

pub const NETTLE_STREEBOG256: NettleHash = NettleHash {
    name: "streebog256",
    context_size: std::mem::size_of::<Streebog256>(),
    digest_size: Streebog256::DIGEST_SIZE,
    init: Streebog256::new,
    update: Streebog256::update,
    digest: Streebog256::digest,
};