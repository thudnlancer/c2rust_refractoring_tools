/*
   CMAC-64, NIST SP 800-38B
   Copyright (C) Stefan Metzmacher 2012
   Copyright (C) Jeremy Allison 2012
   Copyright (C) Michael Adam 2012
   Copyright (C) 2017, Red Hat Inc.
   Copyright (C) 2019, Dmitry Eremin-Solenikov

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

use std::cmp::min;

#[derive(Clone, Copy)]
pub union Block8 {
    b: [u8; 8],
    u: [u64; 1],
}

impl Default for Block8 {
    fn default() -> Self {
        Block8 { b: [0; 8] }
    }
}

impl Block8 {
    pub fn xor3(&mut self, a: &Block8, b: &Block8) {
        for i in 0..8 {
            self.b[i] = a.b[i] ^ b.b[i];
        }
    }

    pub fn xor_bytes(&mut self, a: &Block8, b: &[u8]) {
        for i in 0..min(8, b.len()) {
            self.b[i] = a.b[i] ^ b[i];
        }
    }

    pub fn xor(&mut self, other: &Block8) {
        for i in 0..8 {
            self.b[i] ^= other.b[i];
        }
    }

    pub fn mulx_be(&mut self) {
        let carry = (self.b[0] & 0x80) != 0;
        
        for i in 0..7 {
            self.b[i] = (self.b[i] << 1) | ((self.b[i+1] & 0x80) >> 7;
        }
        self.b[7] <<= 1;
        
        if carry {
            self.b[7] ^= 0x1b;
        }
    }
}

pub struct Cmac64Key {
    pub K1: Block8,
    pub K2: Block8,
}

pub struct Cmac64Ctx {
    pub X: Block8,
    pub block: Block8,
    pub index: usize,
}

pub type CipherFunc = fn(cipher: &[u8], length: usize, dst: &mut [u8], src: &[u8]);

pub fn cmac64_set_key(key: &mut Cmac64Key, cipher: &[u8], encrypt: CipherFunc) {
    let zero_block = Block8::default();
    let mut L = Block8::default();

    encrypt(cipher, 8, &mut L.b, &zero_block.b);

    key.K1 = L;
    key.K1.mulx_be();

    key.K2 = key.K1;
    key.K2.mulx_be();
}

pub fn cmac64_init(ctx: &mut Cmac64Ctx) {
    ctx.X = Block8::default();
    ctx.block = Block8::default();
    ctx.index = 0;
}

pub fn cmac64_update(
    ctx: &mut Cmac64Ctx,
    cipher: &[u8],
    encrypt: CipherFunc,
    msg_len: usize,
    msg: &[u8],
) {
    let mut Y = Block8::default();

    if ctx.index < 8 {
        let len = min(8 - ctx.index, msg_len);
        ctx.block.b[ctx.index..ctx.index + len].copy_from_slice(&msg[..len]);
        ctx.index += len;
        let remaining_msg = &msg[len..];
        if remaining_msg.is_empty() {
            return;
        }
        return cmac64_update(ctx, cipher, encrypt, remaining_msg.len(), remaining_msg);
    }

    Y.xor3(&ctx.X, &ctx.block);
    encrypt(cipher, 8, &mut ctx.X.b, &Y.b);

    let mut chunks = msg.chunks_exact(8);
    for chunk in chunks.by_ref() {
        Y.xor_bytes(&ctx.X, chunk);
        encrypt(cipher, 8, &mut ctx.X.b, &Y.b);
    }

    let remainder = chunks.remainder();
    if !remainder.is_empty() {
        ctx.block.b[..remainder.len()].copy_from_slice(remainder);
        ctx.index = remainder.len();
    }
}

pub fn cmac64_digest(
    ctx: &mut Cmac64Ctx,
    key: &Cmac64Key,
    cipher: &[u8],
    encrypt: CipherFunc,
    length: usize,
    dst: &mut [u8],
) {
    let mut Y = Block8::default();

    for i in ctx.index..8 {
        ctx.block.b[i] = 0;
    }

    if ctx.index < 8 {
        ctx.block.b[ctx.index] = 0x80;
        ctx.block.xor(&key.K2);
    } else {
        ctx.block.xor(&key.K1);
    }

    Y.xor3(&ctx.block, &ctx.X);

    assert!(length <= 8);
    if length == 8 {
        encrypt(cipher, 8, dst, &Y.b);
    } else {
        encrypt(cipher, 8, &mut ctx.block.b, &Y.b);
        dst[..length].copy_from_slice(&ctx.block.b[..length]);
    }

    cmac64_init(ctx);
}