// sha1-meta.rs

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
use sha1::{Sha1, Digest};

pub const NETTLE_SHA1: NettleHash = NettleHash {
    name: "sha1",
    digest_size: Sha1::output_size(),
    context_size: std::mem::size_of::<Sha1>(),
    init: sha1_init,
    update: sha1_update,
    digest: sha1_digest,
};

fn sha1_init(ctx: &mut Sha1) {
    *ctx = Sha1::new();
}

fn sha1_update(ctx: &mut Sha1, length: usize, data: &[u8]) {
    ctx.update(data);
}

fn sha1_digest(ctx: &mut Sha1, length: usize, digest: &mut [u8]) {
    let result = ctx.finalize_reset();
    digest.copy_from_slice(&result);
}