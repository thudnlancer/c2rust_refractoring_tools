// md5-meta.rs

// Copyright (C) 2002 Niels Möller
//
// This file is part of Rust translation of GNU Nettle.
//
// This program is free software: you can redistribute it and/or
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
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.
//
// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use crate::nettle_meta::NettleHash;
use md5::{Md5, Digest};

pub const NETTLE_MD5: NettleHash = NettleHash {
    name: "md5",
    context_size: std::mem::size_of::<Md5>(),
    digest_size: md5::Md5::output_size(),
    block_size: 64,
    init: Md5::new,
    update: |ctx, data| ctx.update(data),
    digest: |mut ctx, digest| {
        let result = ctx.finalize();
        digest.copy_from_slice(&result);
    },
};