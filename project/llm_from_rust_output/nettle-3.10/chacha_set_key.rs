use std::convert::TryInto;

#[derive(Copy, Clone)]
pub struct ChachaCtx {
    pub state: [u32; 16],
}

const SIGMA: [u32; 4] = [
    0x61707865,
    0x3320646e,
    0x79622d32,
    0x6b206574,
];

pub fn nettle_chacha_set_key(ctx: &mut ChachaCtx, key: &[u8]) {
    assert!(key.len() >= 32, "Key must be at least 32 bytes long");
    
    ctx.state[4] = u32::from_le_bytes(key[0..4].try_into().unwrap());
    ctx.state[5] = u32::from_le_bytes(key[4..8].try_into().unwrap());
    ctx.state[6] = u32::from_le_bytes(key[8..12].try_into().unwrap());
    ctx.state[7] = u32::from_le_bytes(key[12..16].try_into().unwrap());
    ctx.state[8] = u32::from_le_bytes(key[16..20].try_into().unwrap());
    ctx.state[9] = u32::from_le_bytes(key[20..24].try_into().unwrap());
    ctx.state[10] = u32::from_le_bytes(key[24..28].try_into().unwrap());
    ctx.state[11] = u32::from_le_bytes(key[28..32].try_into().unwrap());
    
    ctx.state[..4].copy_from_slice(&SIGMA);
}