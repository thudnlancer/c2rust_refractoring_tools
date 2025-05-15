use libc::{c_uchar, c_uint};

type uint8_t = c_uchar;
type uint32_t = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes256Ctx {
    keys: [uint32_t; 60],
}

extern "C" {
    fn _nettle_aes_set_key(
        nr: c_uint,
        nk: c_uint,
        subkeys: *mut uint32_t,
        key: *const uint8_t,
    );
}

pub fn nettle_aes256_set_encrypt_key(ctx: &mut Aes256Ctx, key: &[uint8_t; 32]) {
    unsafe {
        _nettle_aes_set_key(
            14,
            (32 / 4) as c_uint,
            ctx.keys.as_mut_ptr(),
            key.as_ptr(),
        );
    }
}