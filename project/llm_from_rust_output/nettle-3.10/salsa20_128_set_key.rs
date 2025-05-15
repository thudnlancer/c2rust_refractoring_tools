#[repr(C)]
#[derive(Copy, Clone)]
pub struct Salsa20Ctx {
    pub input: [u32; 16],
}

pub fn nettle_salsa20_128_set_key(ctx: &mut Salsa20Ctx, key: &[u8; 16]) {
    ctx.input[1] = u32::from_le_bytes([key[0], key[1], key[2], key[3]]);
    ctx.input[11] = ctx.input[1];
    
    ctx.input[2] = u32::from_le_bytes([key[4], key[5], key[6], key[7]]);
    ctx.input[12] = ctx.input[2];
    
    ctx.input[3] = u32::from_le_bytes([key[8], key[9], key[10], key[11]]);
    ctx.input[13] = ctx.input[3];
    
    ctx.input[4] = u32::from_le_bytes([key[12], key[13], key[14], key[15]]);
    ctx.input[14] = ctx.input[4];
    
    ctx.input[0] = 0x61707865;
    ctx.input[5] = 0x3120646e;
    ctx.input[10] = 0x79622d36;
    ctx.input[15] = 0x6b206574;
}