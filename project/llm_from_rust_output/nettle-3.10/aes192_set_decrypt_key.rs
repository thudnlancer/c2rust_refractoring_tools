use std::mem::MaybeUninit;

pub type uint8_t = u8;
pub type uint32_t = u32;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes192Ctx {
    pub keys: [uint32_t; 52],
}

extern "C" {
    fn nettle_aes192_set_encrypt_key(ctx: *mut Aes192Ctx, key: *const uint8_t);
    fn _nettle_aes_invert(rounds: u32, dst: *mut uint32_t, src: *const uint32_t);
}

pub fn nettle_aes192_invert_key(dst: &mut Aes192Ctx, src: &Aes192Ctx) {
    unsafe {
        _nettle_aes_invert(
            12,
            dst.keys.as_mut_ptr(),
            src.keys.as_ptr(),
        );
    }
}

pub fn nettle_aes192_set_decrypt_key(ctx: &mut Aes192Ctx, key: &[uint8_t]) {
    unsafe {
        nettle_aes192_set_encrypt_key(ctx, key.as_ptr());
    }
    nettle_aes192_invert_key(ctx, ctx);
}