/* umac128.rs

   Copyright (C) 2013 Niels Möller

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

use crate::umac::*;
use crate::umac_internal::*;
use crate::macros::*;

pub struct UMac128Ctx {
    l1_key: [u64; UMAC_KEY_SIZE],
    l2_key: [u64; UMAC_L2_KEY_SIZE],
    l3_key1: [u64; UMAC_L3_KEY1_SIZE],
    l3_key2: [u32; UMAC_L3_KEY2_SIZE],
    pdf_key: Aes128,
    nonce: [u8; AES_BLOCK_SIZE],
    nonce_length: usize,
    count: u64,
    index: usize,
    block: [u8; UMAC_BLOCK_SIZE],
    l2_state: [u64; UMAC_L2_STATE_SIZE],
}

impl UMac128Ctx {
    pub fn set_key(&mut self, key: &[u8]) {
        _nettle_umac_set_key(
            &mut self.l1_key,
            &mut self.l2_key,
            &mut self.l3_key1,
            &mut self.l3_key2,
            &mut self.pdf_key,
            key,
            4,
        );

        // Clear nonce
        self.nonce.fill(0);
        self.nonce_length = self.nonce.len();

        // Initialize buffer
        self.count = 0;
        self.index = 0;
    }

    pub fn set_nonce(&mut self, nonce: &[u8]) {
        assert!(nonce.len() > 0);
        assert!(nonce.len() <= AES_BLOCK_SIZE);

        self.nonce[..nonce.len()].copy_from_slice(nonce);
        self.nonce[nonce.len()..].fill(0);

        self.nonce_length = nonce.len();
    }

    pub fn update(&mut self, data: &[u8]) {
        md_update(
            self,
            data,
            |ctx, block| {
                let mut y = [0u64; 4];
                _nettle_umac_nh_n(&mut y, 4, &ctx.l1_key, UMAC_BLOCK_SIZE, block);
                y[0] += 8 * UMAC_BLOCK_SIZE as u64;
                y[1] += 8 * UMAC_BLOCK_SIZE as u64;
                y[2] += 8 * UMAC_BLOCK_SIZE as u64;
                y[3] += 8 * UMAC_BLOCK_SIZE as u64;
                _nettle_umac_l2(&ctx.l2_key, &mut ctx.l2_state, 4, ctx.count, &y);
                ctx.count += 1;
            },
            || {},
        );
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        assert!(digest.len() > 0);
        assert!(digest.len() <= 16);

        if self.index > 0 || self.count == 0 {
            // Zero pad to multiple of 32
            let pad = if self.index > 0 {
                (31 & !(self.index - 1)) - self.index
            } else {
                32
            };
            self.block[self.index..self.index + pad].fill(0);

            let mut y = [0u64; 4];
            _nettle_umac_nh_n(&mut y, 4, &self.l1_key, self.index + pad, &self.block);
            y[0] += 8 * self.index as u64;
            y[1] += 8 * self.index as u64;
            y[2] += 8 * self.index as u64;
            y[3] += 8 * self.index as u64;
            _nettle_umac_l2(&self.l2_key, &mut self.l2_state, 4, self.count, &y);
            self.count += 1;
        }
        assert!(self.count > 0);

        let mut tag = [0u32; 4];
        self.pdf_key.encrypt(&self.nonce, &mut tag.as_mut());

        increment(&mut self.nonce, self.nonce_length);

        _nettle_umac_l2_final(&self.l2_key, &mut self.l2_state, 4, self.count);
        for i in 0..4 {
            tag[i] ^= self.l3_key2[i] ^ _nettle_umac_l3(&self.l3_key1[8*i..], &self.l2_state[2*i..]);
        }

        digest.copy_from_slice(&tag.as_bytes()[..digest.len()]);

        // Reinitialize
        self.count = 0;
        self.index = 0;
    }
}

fn md_update<F, G>(ctx: &mut UMac128Ctx, data: &[u8], process_block: F, process_partial: G)
where
    F: Fn(&mut UMac128Ctx, &[u8; UMAC_BLOCK_SIZE]),
    G: Fn(),
{
    let mut remaining = data;
    let block_size = UMAC_BLOCK_SIZE;

    if ctx.index > 0 {
        let available = block_size - ctx.index;
        let take = available.min(remaining.len());
        ctx.block[ctx.index..ctx.index + take].copy_from_slice(&remaining[..take]);
        ctx.index += take;
        remaining = &remaining[take..];

        if ctx.index == block_size {
            process_block(ctx, &ctx.block);
            ctx.index = 0;
        }
    }

    while remaining.len() >= block_size {
        let block = remaining[..block_size].try_into().unwrap();
        process_block(ctx, block);
        remaining = &remaining[block_size..];
    }

    if !remaining.is_empty() {
        ctx.block[..remaining.len()].copy_from_slice(remaining);
        ctx.index = remaining.len();
    }

    if remaining.is_empty() {
        process_partial();
    }
}

fn increment(nonce: &mut [u8; AES_BLOCK_SIZE], len: usize) {
    for i in (0..len).rev() {
        nonce[i] = nonce[i].wrapping_add(1);
        if nonce[i] != 0 {
            break;
        }
    }
}