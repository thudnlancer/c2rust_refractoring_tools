// sha3-256.rs

// The sha3 hash function, 256 bit output.

// Copyright (C) 2012 Niels Möller

// This file is part of GNU Nettle.

// GNU Nettle is free software: you can redistribute it and/or
// modify it under the terms of either:

//     * the GNU Lesser General Public License as published by the Free
//       Software Foundation; either version 3 of the License, or (at your
//       option) any later version.

// or

//     * the GNU General Public License as published by the Free
//       Software Foundation; either version 2 of the License, or (at your
//       option) any later version.

// or both in parallel, as here.

// GNU Nettle is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// General Public License for more details.

// You should have received copies of the GNU General Public License and
// the GNU Lesser General Public License along with this program.  If
// not, see http://www.gnu.org/licenses/.

use std::mem::MaybeUninit;

pub struct Sha3_256Ctx {
    state: Sha3State,
    block: [u8; SHA3_256_BLOCK_SIZE],
    index: usize,
}

impl Sha3_256Ctx {
    pub fn new() -> Self {
        Self {
            state: Sha3State::new(),
            block: [0u8; SHA3_256_BLOCK_SIZE],
            index: 0,
        }
    }

    pub fn init(&mut self) {
        self.state = Sha3State::new();
        self.index = 0;
    }

    pub fn update(&mut self, data: &[u8]) {
        self.index = sha3_update(
            &mut self.state,
            SHA3_256_BLOCK_SIZE,
            &mut self.block,
            self.index,
            data,
        );
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        sha3_pad_hash(
            &mut self.state,
            SHA3_256_BLOCK_SIZE,
            &mut self.block,
            self.index,
        );
        write_le64(digest, &self.state.a);
        self.init();
    }
}

const SHA3_256_BLOCK_SIZE: usize = 136;

struct Sha3State {
    a: [u64; 25],
}

impl Sha3State {
    fn new() -> Self {
        Self { a: [0u64; 25] }
    }
}

fn sha3_update(
    state: &mut Sha3State,
    block_size: usize,
    block: &mut [u8],
    index: usize,
    data: &[u8],
) -> usize {
    // Implementation of _nettle_sha3_update
    unimplemented!()
}

fn sha3_pad_hash(
    state: &mut Sha3State,
    block_size: usize,
    block: &mut [u8],
    index: usize,
) {
    // Implementation of _sha3_pad_hash
    unimplemented!()
}

fn write_le64(dest: &mut [u8], data: &[u64]) {
    // Implementation of _nettle_write_le64
    unimplemented!()
}