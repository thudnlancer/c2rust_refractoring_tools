// des.rs

use std::convert::TryInto;

pub const DES_KEY_SIZE: usize = 8;
pub const DES_BLOCK_SIZE: usize = 8;
const _DES_KEY_LENGTH: usize = 32;

pub struct DesCtx {
    key: [u32; _DES_KEY_LENGTH],
}

pub const DES3_KEY_SIZE: usize = 24;
pub const DES3_BLOCK_SIZE: usize = DES_BLOCK_SIZE;

pub struct Des3Ctx {
    des: [DesCtx; 3],
}

static PARITY_16: [u8; 16] = [0, 1, 1, 0, 1, 0, 0, 1, 1, 0, 0, 1, 0, 1, 1, 0];

fn parity(x: u8) -> u8 {
    PARITY_16[(x & 0xf) as usize] ^ PARITY_16[((x >> 4) & 0xf) as usize]
}

pub fn des_check_parity(key: &[u8]) -> bool {
    key.iter().all(|&b| parity(b) == 1)
}

pub fn des_fix_parity(dst: &mut [u8], src: &[u8]) {
    for (d, &s) in dst.iter_mut().zip(src.iter()) {
        *d = s ^ parity(s) ^ 1;
    }
}

fn des_weak_p(key: &[u8]) -> bool {
    static ASSO_VALUES: [u8; 0x81] = [
        16,  9, 26, 26, 26, 26, 26, 26, 26, 26,
        26, 26, 26, 26, 26,  6,  2, 26, 26, 26,
        26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
        26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
        26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
        26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
        26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
        26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
        26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
        26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
        26, 26, 26, 26, 26, 26, 26, 26, 26, 26,
        26, 26,  3,  1, 26, 26, 26, 26, 26, 26,
        26, 26, 26, 26, 26, 26, 26,  0,  0
    ];

    static WEAK_KEY_HASH: [[i8; 4]; 26] = [
        [0x7f,0x7f, 0x7f,0x7f],
        [0x7f,0x70, 0x7f,0x78],
        [0x7f,0x0f, 0x7f,0x07],
        [0x70,0x7f, 0x78,0x7f],
        [0x70,0x70, 0x78,0x78],
        [0x70,0x0f, 0x78,0x07],
        [0x0f,0x7f, 0x07,0x7f],
        [0x0f,0x70, 0x07,0x78],
        [0x0f,0x0f, 0x07,0x07],
        [0x7f,0x00, 0x7f,0x00],
        [-1,-1,-1,-1],
        [-1,-1,-1,-1],
        [0x70,0x00, 0x78,0x00],
        [-1,-1,-1,-1],
        [-1,-1,-1,-1],
        [0x0f,0x00, 0x07,0x00],
        [0x00,0x7f, 0x00,0x7f],
        [0x00,0x70, 0x00,0x78],
        [0x00,0x0f, 0x00,0x07],
        [-1,-1,-1,-1],
        [-1,-1,-1,-1],
        [-1,-1,-1,-1],
        [-1,-1,-1,-1],
        [-1,-1,-1,-1],
        [-1,-1,-1,-1],
        [0x00,0x00, 0x00,0x00]
    ];

    let k0 = (key[0] >> 1) as i8;
    let k1 = (key[1] >> 1) as i8;

    let hash = (ASSO_VALUES[(k1 + 1) as usize] + ASSO_VALUES[k0 as usize]) as usize;

    if hash > 25 {
        return false;
    }

    let candidate = &WEAK_KEY_HASH[hash];

    if k0 != candidate[0] || k1 != candidate[1] {
        return false;
    }

    if (key[2] >> 1) as i8 != k0 || (key[3] >> 1) as i8 != k1 {
        return false;
    }

    let k0 = (key[4] >> 1) as i8;
    let k1 = (key[5] >> 1) as i8;
    if k0 != candidate[2] || k1 != candidate[3] {
        return false;
    }
    if (key[6] >> 1) as i8 != k0 || (key[7] >> 1) as i8 != k1 {
        return false;
    }

    true
}

pub fn des_set_key(ctx: &mut DesCtx, key: &[u8]) -> bool {
    let mut bits0 = [0u8; 56];
    let mut bits1 = [0u8; 56];
    let mut n = 56;
    let mut k_iter = key.iter();

    while n > 0 {
        let w = (256 | k_iter.next().unwrap()) << 2;
        let mut w = w;
        while w >= 16 && n > 0 {
            n -= 1;
            bits1[n] = (8 & w) as u8;
            w >>= 1;
            bits0[n] = (4 & w) as u8;
        }
    }

    let rotors: &[u8] = include!("rotors.rs");
    let mut k = rotors.iter();
    let method = &mut ctx.key;
    let mut method_idx = 0;

    for _ in 0..16 {
        let mut w = ((bits1[*k.next().unwrap() as usize] | bits0[*k.next().unwrap() as usize]) as u32) << 4;
        w |= ((bits1[*k.next().unwrap() as usize] | bits0[*k.next().unwrap() as usize]) as u32) << 2;
        w |= (bits1[*k.next().unwrap() as usize] | bits0[*k.next().unwrap() as usize]) as u32;
        w <<= 8;
        w |= ((bits1[*k.next().unwrap() as usize] | bits0[*k.next().unwrap() as usize]) as u32) << 4;
        w |= ((bits1[*k.next().unwrap() as usize] | bits0[*k.next().unwrap() as usize]) as u32) << 2;
        w |= (bits1[*k.next().unwrap() as usize] | bits0[*k.next().unwrap() as usize]) as u32;
        w <<= 8;
        w |= ((bits1[*k.next().unwrap() as usize] | bits0[*k.next().unwrap() as usize]) as u32) << 4;
        w |= ((bits1[*k.next().unwrap() as usize] | bits0[*k.next().unwrap() as usize]) as u32) << 2;
        w |= (bits1[*k.next().unwrap() as usize] | bits0[*k.next().unwrap() as usize]) as u32;
        w <<= 8;
        w |= ((bits1[*k.next().unwrap() as usize] | bits0[*k.next().unwrap() as usize]) as u32) << 4;
        w |= ((bits1[*k.next().unwrap() as usize] | bits0[*k.next().unwrap() as usize]) as u32) << 2;
        w |= (bits1[*k.next().unwrap() as usize] | bits0[*k.next().unwrap() as usize]) as u32;

        method[method_idx] = w;
        method_idx += 1;

        let mut w = ((bits1[k.next().unwrap() + 24] | bits0[k.next().unwrap() + 24]) as u32) << 4;
        w |= ((bits1[k.next().unwrap() + 24] | bits0[k.next().unwrap() + 24]) as u32) << 2;
        w |= (bits1[k.next().unwrap() + 24] | bits0[k.next().unwrap() + 24]) as u32;
        w <<= 8;
        w |= ((bits1[k.next().unwrap() + 24] | bits0[k.next().unwrap() + 24]) as u32) << 4;
        w |= ((bits1[k.next().unwrap() + 24] | bits0[k.next().unwrap() + 24]) as u32) << 2;
        w |= (bits1[k.next().unwrap() + 24] | bits0[k.next().unwrap() + 24]) as u32;
        w <<= 8;
        w |= ((bits1[k.next().unwrap() + 24] | bits0[k.next().unwrap() + 24]) as u32) << 4;
        w |= ((bits1[k.next().unwrap() + 24] | bits0[k.next().unwrap() + 24]) as u32) << 2;
        w |= (bits1[k.next().unwrap() + 24] | bits0[k.next().unwrap() + 24]) as u32;
        w <<= 8;
        w |= ((bits1[k.next().unwrap() + 24] | bits0[k.next().unwrap() + 24]) as u32) << 4;
        w |= ((bits1[k.next().unwrap() + 24] | bits0[k.next().unwrap() + 24]) as u32) << 2;
        w |= (bits1[k.next().unwrap() + 24] | bits0[k.next().unwrap() + 24]) as u32;

        w = (w >> 4) | ((w & 0xf) << 28);
        method[method_idx] = w;
        method_idx += 1;
    }

    !des_weak_p(key)
}

pub fn des_encrypt(ctx: &DesCtx, dst: &mut [u8], src: &[u8]) {
    assert!(src.len() % DES_BLOCK_SIZE == 0);
    assert!(dst.len() >= src.len());

    for (d, s) in dst.chunks_mut(DES_BLOCK_SIZE).zip(src.chunks(DES_BLOCK_SIZE)) {
        des_small_fips_encrypt(d, &ctx.key, s);
    }
}

pub fn des_decrypt(ctx: &DesCtx, dst: &mut [u8], src: &[u8]) {
    assert!(src.len() % DES_BLOCK_SIZE == 0);
    assert!(dst.len() >= src.len());

    for (d, s) in dst.chunks_mut(DES_BLOCK_SIZE).zip(src.chunks(DES_BLOCK_SIZE)) {
        des_small_fips_decrypt(d, &ctx.key, s);
    }
}

pub fn des3_set_key(ctx: &mut Des3Ctx, key: &[u8]) -> bool {
    des_set_key(&mut ctx.des[0], &key[0..8]) &&
    des_set_key(&mut ctx.des[1], &key[8..16]) &&
    des_set_key(&mut ctx.des[2], &key[16..24])
}

pub fn des3_encrypt(ctx: &Des3Ctx, dst: &mut [u8], src: &[u8]) {
    assert!(src.len() % DES3_BLOCK_SIZE == 0);
    assert!(dst.len() >= src.len());

    for (d, s) in dst.chunks_mut(DES3_BLOCK_SIZE).zip(src.chunks(DES3_BLOCK_SIZE)) {
        des_small_fips_encrypt(d, &ctx.des[0].key, s);
        des_small_fips_decrypt(d, &ctx.des[1].key, d);
        des_small_fips_encrypt(d, &ctx.des[2].key, d);
    }
}

pub fn des3_decrypt(ctx: &Des3Ctx, dst: &mut [u8], src: &[u8]) {
    assert!(src.len() % DES3_BLOCK_SIZE == 0);
    assert!(dst.len() >= src.len());

    for (d, s) in dst.chunks_mut(DES3_BLOCK_SIZE).zip(src.chunks(DES3_BLOCK_SIZE)) {
        des_small_fips_decrypt(d, &ctx.des[2].key, s);
        des_small_fips_encrypt(d, &ctx.des[1].key, d);
        des_small_fips_decrypt(d, &ctx.des[0].key, d);
    }
}

// Include the DES implementation details
mod des_impl {
    include!("des_code.rs");
}

fn des_small_fips_encrypt(dst: &mut [u8], key: &[u32], src: &[u8]) {
    des_impl::des_small_fips_encrypt(dst, key, src);
}

fn des_small_fips_decrypt(dst: &mut [u8], key: &[u32], src: &[u8]) {
    des_impl::des_small_fips_decrypt(dst, key, src);
}