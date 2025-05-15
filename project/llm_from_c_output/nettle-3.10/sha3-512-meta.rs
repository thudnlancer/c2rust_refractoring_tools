// sha3-512-meta.rs

// Copyright (C) 2012 Niels Möller

// This file is part of Rust adaptation of GNU Nettle.

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

use sha3::{Sha3_512, Digest};
use std::hash::Hasher;

pub struct NettleSha3_512 {
    hasher: Sha3_512,
}

impl NettleSha3_512 {
    pub fn new() -> Self {
        NettleSha3_512 {
            hasher: Sha3_512::new(),
        }
    }
}

impl Default for NettleSha3_512 {
    fn default() -> Self {
        Self::new()
    }
}

impl Hasher for NettleSha3_512 {
    fn write(&mut self, bytes: &[u8]) {
        self.hasher.update(bytes);
    }

    fn finish(&self) -> u64 {
        // SHA3-512 produces 512-bit (64-byte) output, but Rust's Hasher trait
        // expects u64. This is a limitation of the trait for cryptographic hashes.
        // For full hash output, use the Digest trait directly.
        let mut result = [0u8; 8];
        result.copy_from_slice(&self.hasher.clone().finalize()[..8]);
        u64::from_be_bytes(result)
    }
}

pub const NETTLE_SHA3_512: NettleSha3_512 = NettleSha3_512 {
    hasher: Sha3_512::new(),
};