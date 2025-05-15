#[derive(Copy, Clone)]
pub struct Salsa20Ctx {
    pub input: [u32; 16],
}

pub fn nettle_salsa20_256_set_key(ctx: &mut Salsa20Ctx, key: &[u8; 32]) {
    ctx.input[1] = u32::from_le_bytes([key[0], key[1], key[2], key[3]]);
    ctx.input[2] = u32::from_le_bytes([key[4], key[5], key[6], key[7]]);
    ctx.input[3] = u32::from_le_bytes([key[8], key[9], key[10], key[11]]);
    ctx.input[4] = u32::from_le_bytes([key[12], key[13], key[14], key[15]]);
    ctx.input[11] = u32::from_le_bytes([key[16], key[17], key[18], key[19]]);
    ctx.input[12] = u32::from_le_bytes([key[20], key[21], key[22], key[23]]);
    ctx.input[13] = u32::from_le_bytes([key[24], key[25], key[26], key[27]]);
    ctx.input[14] = u32::from_le_bytes([key[28], key[29], key[30], key[31]]);
    
    ctx.input[0] = 0x61707865;
    ctx.input[5] = 0x3320646e;
    ctx.input[10] = 0x79622d32;
    ctx.input[15] = 0x6b206574;
}