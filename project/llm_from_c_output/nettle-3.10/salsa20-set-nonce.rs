// salsa20-set-nonce.rs

// The Salsa20 stream cipher.
//
// Copyright (C) 2012 Simon Josefsson, Niels Möller
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

// Based on:
// salsa20-ref.c version 20051118
// D. J. Bernstein
// Public domain.

use std::convert::TryInto;

pub struct Salsa20Ctx {
    input: [u32; 16],
}

impl Salsa20Ctx {
    pub fn set_nonce(&mut self, nonce: &[u8]) -> Result<(), &'static str> {
        if nonce.len() < 8 {
            return Err("Nonce must be at least 8 bytes long");
        }

        self.input[6] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
        self.input[7] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());
        self.input[8] = 0;
        self.input[9] = 0;

        Ok(())
    }
}