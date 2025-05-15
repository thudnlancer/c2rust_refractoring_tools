use std::mem;

#[derive(Copy, Clone)]
#[repr(C)]
pub union NettleBlock16 {
    pub b: [u8; 16],
    pub w: [u64; 2],
    pub u64_0: [u64; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cmac128Key {
    pub k1: NettleBlock16,
    pub k2: NettleBlock16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Cmac128Ctx {
    pub x: NettleBlock16,
    pub block: NettleBlock16,
    pub index: usize,
}

pub type NettleCipherFunc = fn(key: &[u8], length: usize, dst: &mut [u8], src: &[u8]);

impl NettleBlock16 {
    fn xor(&mut self, other: &Self) {
        unsafe {
            self.u64_0[0] ^= other.u64_0[0];
            self.u64_0[1] ^= other.u64_0[1];
        }
    }

    fn xor3(&mut self, x: &Self, y: &Self) {
        unsafe {
            self.u64_0[0] = x.u64_0[0] ^ y.u64_0[0];
            self.u64_0[1] = x.u64_0[1] ^ y.u64_0[1];
        }
    }

    fn xor_bytes(&mut self, x: &Self, bytes: &[u8]) {
        assert!(bytes.len() <= 16);
        for (i, &b) in bytes.iter().enumerate() {
            self.b[i] ^= b;
        }
    }

    fn mulx_be(&mut self, src: &Self) {
        unsafe {
            let carry = (src.u64_0[0] & 0x80) >> 7;
            self.u64_0[0] = (src.u64_0[0] & 0x7f7f7f7f7f7f7f7f) << 1
                | (src.u64_0[0] & 0x8080808080808080) >> 15
                | (src.u64_0[1] & 0x80) << 49;
            self.u64_0[1] = ((src.u64_0[1] & 0x7f7f7f7f7f7f7f7f) << 1
                | (src.u64_0[1] & 0x8080808080808080) >> 15)
                ^ (0x87 << 56) & (!carry + 1);
        }
    }
}

pub fn cmac128_set_key(key: &mut Cmac128Key, cipher: &[u8], encrypt: NettleCipherFunc) {
    let zero_block = NettleBlock16 { b: [0; 16] };
    let mut l = NettleBlock16 { b: [0; 16] };
    
    encrypt(cipher, 16, unsafe { &mut l.b }, unsafe { &zero_block.b });
    
    key.k1 = l;
    key.k1.mulx_be(&l);
    
    key.k2 = key.k1;
    key.k2.mulx_be(&key.k1);
}

pub fn cmac128_init(ctx: &mut Cmac128Ctx) {
    ctx.x = NettleBlock16 { b: [0; 16] };
    ctx.block = NettleBlock16 { b: [0; 16] };
    ctx.index = 0;
}

pub fn cmac128_update(
    ctx: &mut Cmac128Ctx,
    cipher: &[u8],
    encrypt: NettleCipherFunc,
    msg: &[u8],
) {
    let mut y = NettleBlock16 { b: [0; 16] };
    
    if ctx.index < 16 {
        let len = std::cmp::min(16 - ctx.index, msg.len());
        unsafe {
            ctx.block.b[ctx.index..ctx.index + len].copy_from_slice(&msg[..len]);
        }
        ctx.index += len;
        
        if msg.len() == len {
            return;
        }
    }
    
    y.xor3(&ctx.x, &ctx.block);
    encrypt(cipher, 16, unsafe { &mut ctx.x.b }, unsafe { &y.b });
    
    let mut remaining = &msg[16 - ctx.index..];
    while remaining.len() > 16 {
        y.xor_bytes(&ctx.x, &remaining[..16]);
        encrypt(cipher, 16, unsafe { &mut ctx.x.b }, unsafe { &y.b });
        remaining = &remaining[16..];
    }
    
    unsafe {
        ctx.block.b[..remaining.len()].copy_from_slice(remaining);
    }
    ctx.index = remaining.len();
}

pub fn cmac128_digest(
    ctx: &mut Cmac128Ctx,
    key: &Cmac128Key,
    cipher: &[u8],
    encrypt: NettleCipherFunc,
    length: usize,
    dst: &mut [u8],
) {
    assert!(length <= 16);
    let mut y = NettleBlock16 { b: [0; 16] };
    
    if ctx.index < 16 {
        unsafe {
            ctx.block.b[ctx.index] = 0x80;
            for i in ctx.index + 1..16 {
                ctx.block.b[i] = 0;
            }
        }
        ctx.block.xor(&key.k2);
    } else {
        ctx.block.xor(&key.k1);
    }
    
    y.xor3(&ctx.block, &ctx.x);
    
    if length == 16 {
        encrypt(cipher, 16, dst, unsafe { &y.b });
    } else {
        encrypt(cipher, 16, unsafe { &mut ctx.block.b }, unsafe { &y.b });
        dst[..length].copy_from_slice(unsafe { &ctx.block.b[..length] });
    }
    
    cmac128_init(ctx);
}