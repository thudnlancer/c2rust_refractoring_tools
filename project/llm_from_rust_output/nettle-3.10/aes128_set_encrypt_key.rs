use libc::{c_uint, c_uchar};

pub type uint8_t = c_uchar;
pub type uint32_t = c_uint;

#[derive(Copy, Clone)]
pub struct Aes128Ctx {
    keys: [uint32_t; 44],
}

extern "C" {
    fn _nettle_aes_set_key(
        nr: c_uint,
        nk: c_uint,
        subkeys: *mut uint32_t,
        key: *const uint8_t,
    );
}

pub fn nettle_aes128_set_encrypt_key(ctx: &mut Aes128Ctx, key: &[uint8_t; 16]) {
    unsafe {
        _nettle_aes_set_key(
            10,
            (16 / 4) as c_uint,
            ctx.keys.as_mut_ptr(),
            key.as_ptr(),
        );
    }
}