use libc::{c_uint, c_uchar, c_uint as uint32_t, c_uchar as uint8_t};

extern "C" {
    fn _nettle_aes_invert(rounds: c_uint, dst: *mut uint32_t, src: *const uint32_t);
    fn nettle_aes256_set_encrypt_key(ctx: *mut Aes256Ctx, key: *const uint8_t);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes256Ctx {
    pub keys: [uint32_t; 60],
}

pub fn nettle_aes256_invert_key(dst: &mut Aes256Ctx, src: &Aes256Ctx) {
    unsafe {
        _nettle_aes_invert(14 as c_uint, dst.keys.as_mut_ptr(), src.keys.as_ptr());
    }
}

pub fn nettle_aes256_set_decrypt_key(ctx: &mut Aes256Ctx, key: &[uint8_t]) {
    unsafe {
        nettle_aes256_set_encrypt_key(ctx, key.as_ptr());
    }
    nettle_aes256_invert_key(ctx, ctx);
}