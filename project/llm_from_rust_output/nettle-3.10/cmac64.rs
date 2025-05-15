use std::mem;

#[derive(Copy, Clone, Default)]
#[repr(C)]
pub struct Block8 {
    pub b: [u8; 8],
    pub u64_0: u64,
}

impl Block8 {
    pub fn xor(&mut self, other: &Block8) {
        self.u64_0 ^= other.u64_0;
    }

    pub fn xor3(&mut self, a: &Block8, b: &Block8) {
        self.u64_0 = a.u64_0 ^ b.u64_0;
    }

    pub fn xor_bytes(&mut self, other: &Block8, bytes: &[u8]) {
        assert!(bytes.len() == 8);
        for i in 0..8 {
            self.b[i] = other.b[i] ^ bytes[i];
        }
    }

    pub fn mulx_be(&mut self, src: &Block8) {
        let carry = (src.u64_0 & 0x80) >> 7;
        self.u64_0 = ((src.u64_0 & 0x7f7f7f7f7f7f7f7f) << 1 | (src.u64_0 & 0x8080808080808080) >> 15)
            ^ (0x1b << 56).wrapping_neg() & carry;
    }
}

#[derive(Copy, Clone)]
pub struct Cmac64Key {
    pub k1: Block8,
    pub k2: Block8,
}

#[derive(Default)]
pub struct Cmac64Ctx {
    pub x: Block8,
    pub block: Block8,
    pub index: usize,
}

pub type CipherFunc = fn(cipher: &[u8], length: usize, dst: &mut [u8], src: &[u8]);

pub fn cmac64_set_key(key: &mut Cmac64Key, cipher: &[u8], encrypt: CipherFunc) {
    let zero_block = Block8::default();
    let mut l = Block8::default();
    encrypt(cipher, 8, &mut l.b, &zero_block.b);
    
    key.k1 = l;
    key.k1.mulx_be(&l);
    
    key.k2 = key.k1;
    key.k2.mulx_be(&key.k1);
}

pub fn cmac64_init(ctx: &mut Cmac64Ctx) {
    *ctx = Cmac64Ctx::default();
}

pub fn cmac64_update(ctx: &mut Cmac64Ctx, cipher: &[u8], encrypt: CipherFunc, msg: &[u8]) {
    let mut y = Block8::default();
    let mut msg = msg;
    
    if ctx.index < 8 {
        let len = std::cmp::min(8 - ctx.index, msg.len());
        ctx.block.b[ctx.index..ctx.index + len].copy_from_slice(&msg[..len]);
        msg = &msg[len..];
        ctx.index += len;
    }
    
    if msg.is_empty() {
        return;
    }
    
    y.xor3(&ctx.x, &ctx.block);
    encrypt(cipher, 8, &mut ctx.x.b, &y.b);
    
    while msg.len() > 8 {
        y.xor_bytes(&ctx.x, &msg[..8]);
        encrypt(cipher, 8, &mut ctx.x.b, &y.b);
        msg = &msg[8..];
    }
    
    ctx.block.b[..msg.len()].copy_from_slice(msg);
    ctx.index = msg.len();
}

pub fn cmac64_digest(
    ctx: &mut Cmac64Ctx,
    key: &Cmac64Key,
    cipher: &[u8],
    encrypt: CipherFunc,
    length: usize,
    dst: &mut [u8],
) {
    assert!(length <= 8);
    
    let mut y = Block8::default();
    let block_len = ctx.index;
    
    if block_len < 8 {
        ctx.block.b[block_len..].fill(0);
        ctx.block.b[block_len] = 0x80;
        ctx.block.xor(&key.k2);
    } else {
        ctx.block.xor(&key.k1);
    }
    
    y.xor3(&ctx.block, &ctx.x);
    
    if length == 8 {
        encrypt(cipher, 8, dst, &y.b);
    } else {
        encrypt(cipher, 8, &mut ctx.block.b, &y.b);
        dst[..length].copy_from_slice(&ctx.block.b[..length]);
    }
    
    *ctx = Cmac64Ctx::default();
}