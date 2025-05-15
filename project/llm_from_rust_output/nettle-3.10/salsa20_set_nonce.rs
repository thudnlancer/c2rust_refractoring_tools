use std::convert::TryInto;

pub type uint8_t = u8;
pub type uint32_t = u32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Salsa20Ctx {
    pub input: [uint32_t; 16],
}

impl Salsa20Ctx {
    pub fn set_nonce(&mut self, nonce: &[uint8_t; 8]) {
        self.input[6] = u32::from_le_bytes(nonce[0..4].try_into().unwrap());
        self.input[7] = u32::from_le_bytes(nonce[4..8].try_into().unwrap());
        self.input[8] = 0;
        self.input[9] = 0;
    }
}

#[no_mangle]
pub extern "C" fn nettle_salsa20_set_nonce(ctx: &mut Salsa20Ctx, nonce: *const uint8_t) {
    let nonce_slice = unsafe { std::slice::from_raw_parts(nonce, 8) };
    ctx.set_nonce(nonce_slice.try_into().unwrap());
}