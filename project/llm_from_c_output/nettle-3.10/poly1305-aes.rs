/* poly1305-aes.rs

   Copyright (C) 2013 Nikos Mavrogiannopoulos
   Copyright (C) 2014 Niels Möller

   This file is part of GNU Nettle.

   GNU Nettle is free software: you can redistribute it and/or
   modify it under the terms of either:

     * the GNU Lesser General Public License as published by the Free
       Software Foundation; either version 3 of the License, or (at your
       option) any later version.

   or

     * the GNU General Public License as published by the Free
       Software Foundation; either version 2 of the License, or (at your
       option) any later version.

   or both in parallel, as here.

   GNU Nettle is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
   General Public License for more details.

   You should have received copies of the GNU General Public License and
   the GNU Lesser General Public License along with this program.  If
   not, see http://www.gnu.org/licenses/.
*/

use std::mem;
use std::convert::TryInto;
use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit};
use cipher::generic_array::GenericArray;

pub struct Poly1305AesCtx {
    aes: Aes128,
    pctx: Poly1305Ctx,
    nonce: [u8; POLY1305_AES_NONCE_SIZE],
    block: [u8; POLY1305_BLOCK_SIZE],
    index: usize,
}

impl Poly1305AesCtx {
    pub fn set_key(&mut self, key: &[u8]) {
        let key_array = GenericArray::from_slice(&key[..16]);
        self.aes = Aes128::new(key_array);
        self.pctx.set_key(&key[16..]);
        self.index = 0;
    }

    pub fn set_nonce(&mut self, nonce: &[u8]) {
        self.nonce.copy_from_slice(nonce);
    }

    pub fn update(&mut self, data: &[u8]) {
        self.index = self.pctx.update(&mut self.block, self.index, data);
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        let mut s = [0u8; POLY1305_BLOCK_SIZE];
        
        if self.index > 0 {
            assert!(self.index < POLY1305_BLOCK_SIZE);

            self.block[self.index] = 1;
            for i in self.index + 1..POLY1305_BLOCK_SIZE {
                self.block[i] = 0;
            }

            self.pctx.block(&self.block, 0);
        }

        let nonce_array = GenericArray::from_slice(&self.nonce);
        let mut output = GenericArray::default();
        self.aes.encrypt_block_b2b(nonce_array, &mut output);
        s.copy_from_slice(output.as_slice());

        self.pctx.digest(&s);
        digest[..s.len().min(digest.len())].copy_from_slice(&s[..digest.len()]);

        increment(&mut self.nonce);
        self.index = 0;
    }
}

fn increment(nonce: &mut [u8; 16]) {
    for i in 0..16 {
        nonce[i] = nonce[i].wrapping_add(1);
        if nonce[i] != 0 {
            break;
        }
    }
}

// Note: The following are placeholder implementations for the Poly1305 context.
// In a real implementation, these would need to be properly defined.
struct Poly1305Ctx {
    // Poly1305 state fields
}

impl Poly1305Ctx {
    fn set_key(&mut self, key: &[u8]) {
        // Implementation of Poly1305 key setup
    }

    fn update(&mut self, block: &mut [u8], index: usize, data: &[u8]) -> usize {
        // Implementation of Poly1305 update
        index
    }

    fn block(&mut self, block: &[u8], flag: u8) {
        // Implementation of Poly1305 block processing
    }

    fn digest(&mut self, s: &[u8]) {
        // Implementation of Poly1305 digest
    }
}

const POLY1305_AES_NONCE_SIZE: usize = 16;
const POLY1305_BLOCK_SIZE: usize = 16;