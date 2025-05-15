use std::convert::TryInto;

pub const CAST128_BLOCK_SIZE: usize = 8;
pub const CAST5_MIN_KEY_SIZE: usize = 5;
pub const CAST5_MAX_KEY_SIZE: usize = 16;
pub const CAST128_KEY_SIZE: usize = 16;
const CAST_SMALL_KEY: usize = 10;

pub struct Cast128Ctx {
    rounds: usize, // Number of rounds to use, 12 or 16
    Kr: [u8; 16],  // Expanded key rotations (5 bits only)
    Km: [u32; 16], // 32-bit masks
}

impl Cast128Ctx {
    pub fn new() -> Self {
        Cast128Ctx {
            rounds: 0,
            Kr: [0; 16],
            Km: [0; 16],
        }
    }

    pub fn set_key(&mut self, key: &[u8]) -> Result<(), &'static str> {
        if key.len() < CAST5_MIN_KEY_SIZE || key.len() > CAST5_MAX_KEY_SIZE {
            return Err("Invalid key size");
        }
        cast5_set_key(self, key.len(), key)
    }

    pub fn encrypt(&self, src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
        if src.len() % CAST128_BLOCK_SIZE != 0 || dst.len() < src.len() {
            return Err("Invalid buffer sizes");
        }
        cast128_encrypt(self, src, dst)
    }

    pub fn decrypt(&self, src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
        if src.len() % CAST128_BLOCK_SIZE != 0 || dst.len() < src.len() {
            return Err("Invalid buffer sizes");
        }
        cast128_decrypt(self, src, dst)
    }
}

fn read_u32_be(bytes: &[u8]) -> u32 {
    u32::from_be_bytes(bytes[..4].try_into().unwrap())
}

fn read_u24_be(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]])
}

fn read_u16_be(bytes: &[u8]) -> u32 {
    u32::from_be_bytes([0, 0, bytes[0], bytes[1]])
}

fn write_u32_be(bytes: &mut [u8], value: u32) {
    bytes[..4].copy_from_slice(&value.to_be_bytes());
}

fn rotl32(shift: u8, value: u32) -> u32 {
    value.rotate_left(shift as u32)
}

fn f1(ctx: &Cast128Ctx, l: &mut u32, r: u32, i: usize) {
    let t = ctx.Km[i].wrapping_add(r);
    let t = rotl32(ctx.Kr[i], t);
    *l ^= ((S1[(t >> 24) as usize] ^ S2[((t >> 16) & 0xff) as usize])
        .wrapping_sub(S3[((t >> 8) & 0xff) as usize]))
        .wrapping_add(S4[(t & 0xff) as usize]);
}

fn f2(ctx: &Cast128Ctx, l: &mut u32, r: u32, i: usize) {
    let t = ctx.Km[i] ^ r;
    let t = rotl32(ctx.Kr[i], t);
    *l ^= ((S1[(t >> 24) as usize].wrapping_sub(S2[((t >> 16) & 0xff) as usize]))
        .wrapping_add(S3[((t >> 8) & 0xff) as usize]))
        ^ S4[(t & 0xff) as usize];
}

fn f3(ctx: &Cast128Ctx, l: &mut u32, r: u32, i: usize) {
    let t = ctx.Km[i].wrapping_sub(r);
    let t = rotl32(ctx.Kr[i], t);
    *l ^= ((S1[(t >> 24) as usize].wrapping_add(S2[((t >> 16) & 0xff) as usize]))
        ^ S3[((t >> 8) & 0xff) as usize])
        .wrapping_sub(S4[(t & 0xff) as usize]);
}

fn cast128_encrypt(ctx: &Cast128Ctx, src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
    for (src_block, dst_block) in src.chunks(CAST128_BLOCK_SIZE).zip(dst.chunks_mut(CAST128_BLOCK_SIZE)) {
        let mut l = read_u32_be(&src_block[0..4]);
        let mut r = read_u32_be(&src_block[4..8]);

        f1(&ctx, &mut l, r, 0);
        f2(&ctx, &mut r, l, 1);
        f3(&ctx, &mut l, r, 2);
        f1(&ctx, &mut r, l, 3);
        f2(&ctx, &mut l, r, 4);
        f3(&ctx, &mut r, l, 5);
        f1(&ctx, &mut l, r, 6);
        f2(&ctx, &mut r, l, 7);
        f3(&ctx, &mut l, r, 8);
        f1(&ctx, &mut r, l, 9);
        f2(&ctx, &mut l, r, 10);
        f3(&ctx, &mut r, l, 11);

        if ctx.rounds > 12 {
            f1(&ctx, &mut l, r, 12);
            f2(&ctx, &mut r, l, 13);
            f3(&ctx, &mut l, r, 14);
            f1(&ctx, &mut r, l, 15);
        }

        write_u32_be(&mut dst_block[0..4], r);
        write_u32_be(&mut dst_block[4..8], l);
    }
    Ok(())
}

fn cast128_decrypt(ctx: &Cast128Ctx, src: &[u8], dst: &mut [u8]) -> Result<(), &'static str> {
    for (src_block, dst_block) in src.chunks(CAST128_BLOCK_SIZE).zip(dst.chunks_mut(CAST128_BLOCK_SIZE)) {
        let mut r = read_u32_be(&src_block[0..4]);
        let mut l = read_u32_be(&src_block[4..8]);

        if ctx.rounds > 12 {
            f1(&ctx, &mut r, l, 15);
            f3(&ctx, &mut l, r, 14);
            f2(&ctx, &mut r, l, 13);
            f1(&ctx, &mut l, r, 12);
        }

        f3(&ctx, &mut r, l, 11);
        f2(&ctx, &mut l, r, 10);
        f1(&ctx, &mut r, l, 9);
        f3(&ctx, &mut l, r, 8);
        f2(&ctx, &mut r, l, 7);
        f1(&ctx, &mut l, r, 6);
        f3(&ctx, &mut r, l, 5);
        f2(&ctx, &mut l, r, 4);
        f1(&ctx, &mut r, l, 3);
        f3(&ctx, &mut l, r, 2);
        f2(&ctx, &mut r, l, 1);
        f1(&ctx, &mut l, r, 0);

        write_u32_be(&mut dst_block[0..4], l);
        write_u32_be(&mut dst_block[4..8], r);
    }
    Ok(())
}

fn cast5_set_key(ctx: &mut Cast128Ctx, length: usize, key: &[u8]) -> Result<(), &'static str> {
    if length < CAST5_MIN_KEY_SIZE || length > CAST5_MAX_KEY_SIZE {
        return Err("Invalid key size");
    }

    let full = length > CAST_SMALL_KEY;
    let mut x0 = read_u32_be(&key[0..4]);

    let w = match length & 3 {
        0 => read_u32_be(&key[length - 4..length]),
        3 => read_u24_be(&key[length - 3..length]) << 8,
        2 => read_u16_be(&key[length - 2..length]) << 16,
        1 => (key[length - 1] as u32) << 24,
        _ => unreachable!(),
    };

    let (x1, x2, x3) = if length <= 8 {
        (w, 0, 0)
    } else {
        let x1 = read_u32_be(&key[4..8]);
        if length <= 12 {
            (x1, w, 0)
        } else {
            let x2 = read_u32_be(&key[8..12]);
            (x1, x2, w)
        }
    };

    expand_key(ctx, full, x0, x1, x2, x3);
    ctx.rounds = if full { 16 } else { 12 };
    Ok(())
}

fn expand_key(ctx: &mut Cast128Ctx, full: bool, mut x0: u32, mut x1: u32, mut x2: u32, mut x3: u32) {
    for round in 0..4 {
        let (z0, z1, z2, z3, x0_new, x1_new, x2_new, x3_new) = expand_round(x0, x1, x2, x3);
        
        ctx.Km[round * 4] = S5[(z2 >> 24) as usize] ^ S6[((z2 >> 16) & 0xff) as usize] 
            ^ S7[((z1 >> 8) & 0xff) as usize] ^ S8[((z1 >> 16) & 0xff) as usize] 
            ^ S5[((z0 >> 16) & 0xff) as usize];
        
        ctx.Kr[round * 4] = (S5[((z2 >> 16) & 0xff) as usize] ^ S6[((z2 >> 8) & 0xff) as usize] 
            ^ S7[((z1 >> 24) as usize] ^ S8[(z1 & 0xff) as usize] 
            ^ S6[((z1 >> 16) & 0xff) as usize]) & 0x1f;

        // ... (完整实现所有16轮的密钥扩展)

        x0 = x0_new;
        x1 = x1_new;
        x2 = x2_new;
        x3 = x3_new;
    }
}

// S-boxes definitions (从原C代码中的cast128_sboxes.h转换而来)
const S1: [u32; 256] = [ /* ... */ ];
const S2: [u32; 256] = [ /* ... */ ];
const S3: [u32; 256] = [ /* ... */ ];
const S4: [u32; 256] = [ /* ... */ ];
const S5: [u32; 256] = [ /* ... */ ];
const S6: [u32; 256] = [ /* ... */ ];
const S7: [u32; 256] = [ /* ... */ ];
const S8: [u32; 256] = [ /* ... */ ];