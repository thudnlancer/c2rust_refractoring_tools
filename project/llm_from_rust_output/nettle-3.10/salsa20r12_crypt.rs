use std::convert::TryInto;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Salsa20Ctx {
    pub input: [uint32_t; 16],
}

impl Salsa20Ctx {
    fn crypt(&mut self, rounds: u32, length: size_t, dst: &mut [u8], src: &[u8]) {
        if length == 0 {
            return;
        }
        unsafe {
            _nettle_salsa20_crypt(
                self as *mut Salsa20Ctx,
                rounds,
                length,
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }
}

extern "C" {
    fn _nettle_salsa20_crypt(
        ctx: *mut Salsa20Ctx,
        rounds: u32,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}

pub fn salsa20r12_crypt(ctx: &mut Salsa20Ctx, length: size_t, c: &mut [u8], m: &[u8]) {
    ctx.crypt(12, length, c, m);
}