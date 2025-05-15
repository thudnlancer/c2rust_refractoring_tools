// chacha-set-key.rs

// Copyright (C) 2014 Niels Möller
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

use std::convert::TryInto;

pub struct ChaChaCtx {
    pub state: [u32; 16],
}

impl ChaChaCtx {
    pub fn set_key(&mut self, key: &[u8]) -> Result<(), &'static str> {
        const SIGMA: [u32; 4] = [
            // "expand 32-byte k"
            0x61707865, 0x3320646e, 0x79622d32, 0x6b206574,
        ];

        if key.len() != 32 {
            return Err("Key length must be 32 bytes");
        }

        for i in 0..4 {
            self.state[i] = SIGMA[i];
        }

        for i in 0..8 {
            let offset = i * 4;
            let bytes = key[offset..offset + 4].try_into().unwrap();
            self.state[4 + i] = u32::from_le_bytes(bytes);
        }

        Ok(())
    }
}