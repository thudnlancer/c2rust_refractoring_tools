// balloon-sha256.rs

// Balloon password-hashing algorithm.

// Copyright (C) 2022 Zoltan Fridrich
// Copyright (C) 2022 Red Hat, Inc.

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

use sha2::{Sha256, Digest};
use crate::balloon::balloon;

pub fn balloon_sha256(
    s_cost: usize,
    t_cost: usize,
    passwd: &[u8],
    salt: &[u8],
    scratch: &mut [u8],
    dst: &mut [u8],
) {
    let mut hasher = Sha256::new();
    balloon(
        &mut hasher,
        |ctx, data| {
            ctx.update(data);
            Ok(())
        },
        |ctx, output| {
            let result = ctx.finalize_reset();
            output.copy_from_slice(&result);
            Ok(())
        },
        sha2::Sha256::output_size(),
        s_cost,
        t_cost,
        passwd,
        salt,
        scratch,
        dst,
    );
}