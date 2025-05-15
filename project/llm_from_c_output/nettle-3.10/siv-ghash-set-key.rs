// siv-ghash-set-key.rs

// POLYVAL implementation for AES-GCM-SIV, based on GHASH

// Copyright (C) 2011 Katholieke Universiteit Leuven
// Copyright (C) 2011, 2013, 2018, 2022 Niels Möller
// Copyright (C) 2018, 2022 Red Hat, Inc.

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

use crate::ghash_internal::GhashKey;
use crate::block_internal::{Block16, block16_bswap, block16_mulx_ghash};

pub fn siv_ghash_set_key(ctx: &mut GhashKey, key: &Block16) {
    let mut h = Block16::default();
    
    block16_bswap(&mut h, key);
    block16_mulx_ghash(&mut h, &h);
    
    ctx.set_key(&h);
}