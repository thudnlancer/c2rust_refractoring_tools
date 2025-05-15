use std::convert::TryInto;

const CAST_SBOX1: [u32; 256] = [
    0x30fb40d4, 0x9fa0ff0b, 0x6beccd2f, 0x3f258c7a, 0x1e213f2f, 0x9c004dd3, 0x6003e540, 0xcf9fc949,
    // ... (完整S盒数据)
];

const CAST_SBOX2: [u32; 256] = [
    0x1f201094, 0xef0ba75b, 0x69e3cf7e, 0x393f4380, 0xfe61cf7a, 0xeec5207a, 0x55889c94, 0x72fc0651,
    // ... (完整S盒数据)
];

const CAST_SBOX3: [u32; 256] = [
    0x8defc240, 0x25fa5d9f, 0xeb903dbf, 0xe810c907, 0x47607fff, 0x369fe44b, 0x8c1fc644, 0xaececa90,
    // ... (完整S盒数据)
];

const CAST_SBOX4: [u32; 256] = [
    0x9db30420, 0x1fb6e9de, 0xa7be7bef, 0xd273a298, 0x4a4f7bdb, 0x64ad8c57, 0x85510443, 0xfa020ed1,
    // ... (完整S盒数据)
];

const CAST_SBOX5: [u32; 256] = [
    0x7ec90c04, 0x2c6e74b9, 0x9b0e66df, 0xa6337911, 0xb86a7fff, 0x1dd358f5, 0x44dd9d44, 0x1731167f,
    // ... (完整S盒数据)
];

const CAST_SBOX6: [u32; 256] = [
    0xf6fa8f9d, 0x2cac6ce1, 0x4ca34867, 0xe2337f7c, 0x95db08e7, 0x16843b4, 0xeced5cbc, 0x325553ac,
    // ... (完整S盒数据)
];

const CAST_SBOX7: [u32; 256] = [
    0x85e04019, 0x332bf567, 0x662dbfff, 0xcfc65693, 0x2a8d7f6f, 0xab9bc912, 0xde6008a1, 0x2028da1f,
    // ... (完整S盒数据)
];

const CAST_SBOX8: [u32; 256] = [
    0xe216300d, 0xbbddfffc, 0xa7ebdabd, 0x35648095, 0x7789f8b7, 0xe6c1121b, 0xe241600, 0x52ce8b5,
    // ... (完整S盒数据)
];

#[derive(Clone, Copy)]
pub struct Cast128Ctx {
    rounds: u32,
    kr: [u8; 16],
    km: [u32; 16],
}

impl Cast128Ctx {
    pub fn encrypt(&self, length: usize, dst: &mut [u8], src: &[u8]) {
        assert_eq!(length % 8, 0);
        
        for chunk in src.chunks_exact(8).take(length / 8) {
            let mut l = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let mut r = u32::from_be_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
            
            for i in 0..16 {
                if i == 12 && self.rounds != 16 {
                    break;
                }
                
                let (t, f) = self.f(i, r);
                l ^= f;
                
                if i < 15 {
                    std::mem::swap(&mut l, &mut r);
                }
            }
            
            dst[..4].copy_from_slice(&r.to_be_bytes());
            dst[4..8].copy_from_slice(&l.to_be_bytes());
        }
    }
    
    pub fn decrypt(&self, length: usize, dst: &mut [u8], src: &[u8]) {
        assert_eq!(length % 8, 0);
        
        for chunk in src.chunks_exact(8).take(length / 8) {
            let mut r = u32::from_be_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]);
            let mut l = u32::from_be_bytes([chunk[4], chunk[5], chunk[6], chunk[7]]);
            
            for i in (0..16).rev() {
                if i == 11 && self.rounds != 16 {
                    break;
                }
                
                let (t, f) = self.f(i, l);
                r ^= f;
                
                if i > 0 {
                    std::mem::swap(&mut l, &mut r);
                }
            }
            
            dst[..4].copy_from_slice(&l.to_be_bytes());
            dst[4..8].copy_from_slice(&r.to_be_bytes());
        }
    }
    
    fn f(&self, i: usize, x: u32) -> (u32, u32) {
        let op = match i % 3 {
            0 => Self::f1,
            1 => Self::f2,
            _ => Self::f3,
        };
        
        let t = (self.km[i].wrapping_add(x)).rotate_left(self.kr[i] as u32);
        (t, op(t))
    }
    
    fn f1(x: u32) -> u32 {
        (CAST_SBOX1[(x >> 24) as usize] ^ CAST_SBOX2[((x >> 16) & 0xff) as usize])
            .wrapping_sub(CAST_SBOX3[((x >> 8) & 0xff) as usize])
            .wrapping_add(CAST_SBOX4[(x & 0xff) as usize])
    }
    
    fn f2(x: u32) -> u32 {
        CAST_SBOX1[(x >> 24) as usize]
            .wrapping_sub(CAST_SBOX2[((x >> 16) & 0xff) as usize])
            .wrapping_add(CAST_SBOX3[((x >> 8) & 0xff) as usize])
            ^ CAST_SBOX4[(x & 0xff) as usize]
    }
    
    fn f3(x: u32) -> u32 {
        (CAST_SBOX1[(x >> 24) as usize].wrapping_add(CAST_SBOX2[((x >> 16) & 0xff) as usize])
            ^ CAST_SBOX3[((x >> 8) & 0xff) as usize])
            .wrapping_sub(CAST_SBOX4[(x & 0xff) as usize])
    }
}

pub fn cast128_set_key(ctx: &mut Cast128Ctx, key: &[u8]) {
    cast5_set_key(ctx, 16, key);
}

pub fn cast5_set_key(ctx: &mut Cast128Ctx, length: usize, key: &[u8]) {
    assert!(length >= 5 && length <= 16);
    
    let full = length > 10;
    
    let mut x = [0u32; 4];
    for (i, chunk) in key.chunks(4).enumerate().take(4) {
        let mut val = 0u32;
        for (j, &b) in chunk.iter().enumerate() {
            val |= (b as u32) << (24 - 8 * j);
        }
        x[i] = val;
    }
    
    // Key schedule calculation...
    // (完整密钥调度算法实现)
    
    ctx.rounds = if full { 16 } else { 12 };
}