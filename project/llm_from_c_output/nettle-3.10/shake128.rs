// shake128.rs

// The SHAKE128 hash function, arbitrary length output.

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

use std::mem::MaybeUninit;

pub struct Sha3_128Ctx {
    state: Sha3State,
    block: [u8; SHA3_128_BLOCK_SIZE],
    index: usize,
}

impl Sha3_128Ctx {
    pub fn new() -> Self {
        Self {
            state: Sha3State::new(),
            block: [0u8; SHA3_128_BLOCK_SIZE],
            index: 0,
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        self.index = sha3_update(
            &mut self.state,
            SHA3_128_BLOCK_SIZE,
            &mut self.block,
            self.index,
            data,
        );
    }

    pub fn shake(&mut self, output: &mut [u8]) {
        sha3_shake(
            &mut self.state,
            self.block.len(),
            &mut self.block,
            self.index,
            output,
        );
        *self = Self::new();
    }

    pub fn shake_output(&mut self, digest: &mut [u8]) -> usize {
        self.index = sha3_shake_output(
            &mut self.state,
            self.block.len(),
            &mut self.block,
            self.index,
            digest,
        );
        self.index
    }
}

// Constants and helper functions would be defined elsewhere
const SHA3_128_BLOCK_SIZE: usize = /* appropriate value */;

struct Sha3State {
    // Implementation details of SHA-3 state
}

impl Sha3State {
    fn new() -> Self {
        // Initialize SHA-3 state
        unimplemented!()
    }
}

fn sha3_update(
    state: &mut Sha3State,
    block_size: usize,
    block: &mut [u8],
    index: usize,
    data: &[u8],
) -> usize {
    // Implementation of SHA-3 update
    unimplemented!()
}

fn sha3_shake(
    state: &mut Sha3State,
    block_size: usize,
    block: &mut [u8],
    index: usize,
    output: &mut [u8],
) {
    // Implementation of SHAKE output
    unimplemented!()
}

fn sha3_shake_output(
    state: &mut Sha3State,
    block_size: usize,
    block: &mut [u8],
    index: usize,
    digest: &mut [u8],
) -> usize {
    // Implementation of incremental SHAKE output
    unimplemented!()
}