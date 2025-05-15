use libc::{c_uchar, c_uint};

pub type uint8_t = c_uchar;
pub type uint32_t = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes128Ctx {
    pub keys: [uint32_t; 44],
}

extern "C" {
    fn nettle_aes128_set_encrypt_key(ctx: *mut Aes128Ctx, key: *const uint8_t);
    fn _nettle_aes_invert(rounds: c_uint, dst: *mut uint32_t, src: *const uint32_t);
}

pub fn nettle_aes128_invert_key(dst: &mut Aes128Ctx, src: &Aes128Ctx) {
    unsafe {
        _nettle_aes_invert(10, dst.keys.as_mut_ptr(), src.keys.as_ptr());
    }
}

pub fn nettle_aes128_set_decrypt_key(ctx: &mut Aes128Ctx, key: &[uint8_t; 16]) {
    unsafe {
        nettle_aes128_set_encrypt_key(ctx, key.as_ptr());
    }
    nettle_aes128_invert_key(ctx, ctx);
}