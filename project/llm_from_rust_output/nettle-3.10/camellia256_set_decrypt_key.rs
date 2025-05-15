use libc::{c_uchar, c_ulong, c_uint};

pub type uint8_t = c_uchar;
pub type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Camellia256Ctx {
    pub keys: [uint64_t; 32],
}

extern "C" {
    fn nettle_camellia192_set_encrypt_key(ctx: *mut Camellia256Ctx, key: *const uint8_t);
    fn nettle_camellia256_set_encrypt_key(ctx: *mut Camellia256Ctx, key: *const uint8_t);
    fn _nettle_camellia_invert_key(nkeys: c_uint, dst: *mut uint64_t, src: *const uint64_t);
}

pub fn camellia256_invert_key(dst: &mut Camellia256Ctx, src: &Camellia256Ctx) {
    unsafe {
        _nettle_camellia_invert_key(
            32 as c_uint,
            dst.keys.as_mut_ptr(),
            src.keys.as_ptr(),
        );
    }
}

pub fn camellia256_set_decrypt_key(ctx: &mut Camellia256Ctx, key: &[uint8_t]) {
    unsafe {
        nettle_camellia256_set_encrypt_key(ctx, key.as_ptr());
    }
    camellia256_invert_key(ctx, ctx);
}

pub fn camellia192_set_decrypt_key(ctx: &mut Camellia256Ctx, key: &[uint8_t]) {
    unsafe {
        nettle_camellia192_set_encrypt_key(ctx, key.as_ptr());
    }
    camellia256_invert_key(ctx, ctx);
}