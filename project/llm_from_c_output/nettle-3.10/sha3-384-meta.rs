// sha3-384-meta.rs

// Copyright (C) 2012 Niels Möller
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

use sha3::Sha3_384;
use digest::Digest;

pub struct NettleSha3_384 {
    hasher: Sha3_384,
}

impl NettleSha3_384 {
    pub fn new() -> Self {
        NettleSha3_384 {
            hasher: Sha3_384::new(),
        }
    }
}

impl Default for NettleSha3_384 {
    fn default() -> Self {
        Self::new()
    }
}

pub const NETTLE_SHA3_384: NettleSha3_384 = NettleSha3_384 {
    hasher: Sha3_384::new(),
};