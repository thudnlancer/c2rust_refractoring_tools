// shake256.rs

// The SHAKE256 hash function, arbitrary length output.

// Copyright (C) 2017 Daiki Ueno
// Copyright (C) 2017 Red Hat, Inc.

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

use crate::sha3::{Sha3_256Ctx, sha3_256_init};
use crate::sha3_internal::nettle_sha3_shake;
use crate::sha3_internal::nettle_sha3_shake_output;

pub fn sha3_256_shake(ctx: &mut Sha3_256Ctx, length: usize, dst: &mut [u8]) {
    nettle_sha3_shake(
        &mut ctx.state,
        ctx.block.len(),
        &mut ctx.block,
        ctx.index,
        length,
        dst,
    );
    sha3_256_init(ctx);
}

pub fn sha3_256_shake_output(
    ctx: &mut Sha3_256Ctx,
    length: usize,
    digest: &mut [u8],
) -> usize {
    ctx.index = nettle_sha3_shake_output(
        &mut ctx.state,
        ctx.block.len(),
        &mut ctx.block,
        ctx.index,
        length,
        digest,
    );
    ctx.index
}