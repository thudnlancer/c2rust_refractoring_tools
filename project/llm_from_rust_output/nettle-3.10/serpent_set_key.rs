use std::convert::TryInto;

const SERPENT_MAX_KEY_SIZE: usize = 32;

#[derive(Copy, Clone)]
pub struct SerpentCtx {
    pub keys: [[u32; 4]; 33],
}

fn serpent_key_pad(key: &[u8], w: &mut [u32; 8]) {
    assert!(key.len() <= SERPENT_MAX_KEY_SIZE, "key_length <= SERPENT_MAX_KEY_SIZE");

    let mut i = 0;
    let mut key_chunks = key.chunks_exact(4);
    
    for chunk in key_chunks.by_ref() {
        w[i] = u32::from_le_bytes(chunk.try_into().unwrap());
        i += 1;
    }

    let remainder = key_chunks.remainder();
    if i < 8 && !remainder.is_empty() {
        let mut pad = 1u32;
        for &byte in remainder.iter().rev() {
            pad = (pad << 8) | byte as u32;
        }
        w[i] = pad;
        i += 1;
        
        while i < 8 {
            w[i] = 0;
            i += 1;
        }
    }
}

pub fn nettle_serpent_set_key(ctx: &mut SerpentCtx, key: &[u8]) {
    let mut w = [0u32; 8];
    serpent_key_pad(key, &mut w);

    let mut keys = &mut ctx.keys[..];
    let mut k = 0u32;

    loop {
        for i in 0..4 {
            let wn = w[i] ^ w[(i + 3) & 7] ^ w[(i + 5) & 7] ^ w[(i + 7) & 7] ^ 0x9e3779b9 ^ k;
            w[i] = wn.rotate_left(11);
            k += 1;
        }

        let [w0, w1, w2, w3] = [w[0], w[1], w[2], w[3];
        
        let t01 = w0 ^ w2;
        let t02 = w0 | w3;
        let t03 = w0 & w3;
        let t04 = t01 & t02;
        let t05 = w1 | t03;
        let t06 = w0 & w1;
        let t07 = w3 ^ t04;
        let t08 = w2 | t06;
        let t09 = w1 ^ t07;
        let t10 = w3 & t05;
        let t11 = t02 ^ t10;
        
        keys[0][3] = t08 ^ t09;
        let t13 = w3 | keys[0][3];
        let t14 = w0 | t07;
        let t15 = w1 & t13;
        
        keys[0][2] = t08 ^ t11;
        keys[0][0] = t14 ^ t15;
        keys[0][1] = t05 ^ t04;
        
        if k == 132 { break; }
        keys = &mut keys[1..];

        for i in 4..8 {
            let wn = w[i] ^ w[(i + 3) & 7] ^ w[(i + 5) & 7] ^ w[(i + 7) & 7] ^ 0x9e3779b9 ^ k;
            w[i] = wn.rotate_left(11);
            k += 1;
        }

        let [w4, w5, w6, w7] = [w[4], w[5], w[6], w[7]];
        
        let t01 = w4 | w6;
        let t02 = w4 ^ w5;
        let t03 = w7 ^ t01;
        keys[0][0] = t02 ^ t03;
        let t05 = w6 ^ keys[0][0];
        let t06 = w5 ^ t05;
        let t07 = w5 | t05;
        let t08 = t01 & t06;
        let t09 = t03 ^ t07;
        let t10 = t02 | t09;
        keys[0][1] = t10 ^ t08;
        let t12 = w4 | w7;
        let t13 = t09 ^ keys[0][1];
        let t14 = w5 ^ t13;
        keys[0][3] = !t09;
        keys[0][2] = t12 ^ t14;
        
        keys = &mut keys[1..];
    }

    assert!(keys.len() == 1, "keys == ctx.keys + 33");
}

pub fn nettle_serpent128_set_key(ctx: &mut SerpentCtx, key: &[u8]) {
    nettle_serpent_set_key(ctx, key);
}

pub fn nettle_serpent192_set_key(ctx: &mut SerpentCtx, key: &[u8]) {
    nettle_serpent_set_key(ctx, key);
}

pub fn nettle_serpent256_set_key(ctx: &mut SerpentCtx, key: &[u8]) {
    nettle_serpent_set_key(ctx, key);
}