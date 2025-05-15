use std::convert::TryInto;

const CAMELIA_TABLE: CamelliaTable = CamelliaTable {
    sp1110: include!("camellia_sp1110.rs"),
    sp0222: include!("camellia_sp0222.rs"),
    sp3033: include!("camellia_sp3033.rs"),
    sp4404: include!("camellia_sp4404.rs"),
};

#[derive(Copy, Clone)]
pub struct Camellia256Ctx {
    pub keys: [u64; 32],
}

#[derive(Copy, Clone)]
pub struct CamelliaTable {
    pub sp1110: [u32; 256],
    pub sp0222: [u32; 256],
    pub sp3033: [u32; 256],
    pub sp4404: [u32; 256],
}

fn camellia256_set_encrypt_key(ctx: &mut Camellia256Ctx, k0: u64, k1: u64, k2: u64, k3: u64) {
    let mut subkey = [0u64; 34];
    let mut w: u64;
    
    subkey[0] = k0;
    subkey[1] = k1;
    
    let (k0, k1) = rotate_left_128(k0, k1, 45);
    subkey[12] = k0;
    subkey[13] = k1;
    
    let (k0, k1) = rotate_left_128(k0, k1, 15);
    subkey[16] = k0;
    subkey[17] = k1;
    
    let (k0, k1) = rotate_left_128(k0, k1, 17);
    subkey[22] = k0;
    subkey[23] = k1;
    
    let (k0, k1) = rotate_left_128(k0, k1, 34);
    subkey[30] = k0;
    subkey[31] = k1;
    
    let (k2, k3) = rotate_left_128(k2, k3, 15);
    subkey[4] = k2;
    subkey[5] = k3;
    
    let (k2, k3) = rotate_left_128(k2, k3, 15);
    subkey[8] = k2;
    subkey[9] = k3;
    
    let (k2, k3) = rotate_left_128(k2, k3, 30);
    subkey[18] = k2;
    subkey[19] = k3;
    
    let (k2, k3) = rotate_left_128(k2, k3, 34);
    subkey[26] = k2;
    subkey[27] = k3;
    
    let (k2, k3) = rotate_left_128(k2, k3, 34);
    let k0 = subkey[0] ^ k2;
    let k1 = subkey[1] ^ k3;
    
    w = camellia_feistel(k0, 0xa09e667f3bcc908b);
    let k1 = k1 ^ w;
    
    let k0 = camellia_feistel(k1, 0xb67ae8584caa73b2) ^ k2;
    
    w = camellia_feistel(k0, 0xc6ef372fe94f82be);
    let k1 = k1 ^ w ^ k3;
    
    w = camellia_feistel(k1, 0x54ff53a5f1d36f1c);
    let k0 = k0 ^ w;
    
    let k2 = k2 ^ k0;
    let k3 = k3 ^ k1;
    
    w = camellia_feistel(k2, 0x10e527fade682d1d);
    let k3 = k3 ^ w;
    
    w = camellia_feistel(k3, 0xb05688c2b3e6c1fd);
    let k2 = k2 ^ w;
    
    let (k0, k1) = rotate_left_128(k0, k1, 15);
    subkey[6] = k0;
    subkey[7] = k1;
    
    let (k0, k1) = rotate_left_128(k0, k1, 30);
    subkey[14] = k0;
    subkey[15] = k1;
    
    let (k0, k1) = rotate_left_128(k0, k1, 32);
    subkey[24] = k0;
    subkey[25] = k1;
    
    let (k0, k1) = rotate_left_128(k0, k1, 17);
    subkey[28] = k0;
    subkey[29] = k1;
    
    subkey[2] = k2;
    subkey[3] = k3;
    
    let (k2, k3) = rotate_left_128(k2, k3, 30);
    subkey[10] = k2;
    subkey[11] = k3;
    
    let (k2, k3) = rotate_left_128(k2, k3, 30);
    subkey[20] = k2;
    subkey[21] = k3;
    
    let (k2, k3) = rotate_left_128(k2, k3, 51);
    subkey[32] = k2;
    subkey[33] = k3;
    
    camellia_absorb(32, &mut ctx.keys, &subkey);
}

fn rotate_left_128(a: u64, b: u64, amount: u32) -> (u64, u64) {
    if amount == 0 {
        (a, b)
    } else if amount < 64 {
        let new_a = (a << amount) | (b >> (64 - amount));
        let new_b = (b << amount) | (a >> (64 - amount));
        (new_a, new_b)
    } else {
        rotate_left_128(b, a, amount - 64)
    }
}

fn camellia_feistel(x: u64, k: u64) -> u64 {
    let y = x ^ k;
    let yl = (y >> 32) as u32;
    let yr = y as u32;
    
    let mut zl = CAMELIA_TABLE.sp1110[(yl >> 24) as usize]
        ^ CAMELIA_TABLE.sp0222[((yl >> 16) & 0xff) as usize]
        ^ CAMELIA_TABLE.sp3033[((yl >> 8) & 0xff) as usize]
        ^ CAMELIA_TABLE.sp4404[(yl & 0xff) as usize];
    
    let mut zr = CAMELIA_TABLE.sp1110[(yr >> 24) as usize]
        ^ CAMELIA_TABLE.sp0222[((yr >> 16) & 0xff) as usize]
        ^ CAMELIA_TABLE.sp3033[((yr >> 8) & 0xff) as usize]
        ^ CAMELIA_TABLE.sp4404[(yr & 0xff) as usize];
    
    zl ^= zr;
    zr = zr.rotate_left(24);
    zr ^= zl;
    
    ((zl as u64) << 32) | (zr as u64)
}

fn camellia_absorb(nkeys: usize, dst: &mut [u64; 32], subkey: &[u64; 34]) {
    for i in 0..nkeys {
        dst[i] = subkey[i];
    }
}

pub fn nettle_camellia256_set_encrypt_key(ctx: &mut Camellia256Ctx, key: &[u8; 32]) {
    let k0 = u64::from_be_bytes(key[0..8].try_into().unwrap());
    let k1 = u64::from_be_bytes(key[8..16].try_into().unwrap());
    let k2 = u64::from_be_bytes(key[16..24].try_into().unwrap());
    let k3 = u64::from_be_bytes(key[24..32].try_into().unwrap());
    camellia256_set_encrypt_key(ctx, k0, k1, k2, k3);
}

pub fn nettle_camellia192_set_encrypt_key(ctx: &mut Camellia256Ctx, key: &[u8; 24]) {
    let k0 = u64::from_be_bytes(key[0..8].try_into().unwrap());
    let k1 = u64::from_be_bytes(key[8..16].try_into().unwrap());
    let k2 = u64::from_be_bytes(key[16..24].try_into().unwrap());
    camellia256_set_encrypt_key(ctx, k0, k1, k2, !k2);
}