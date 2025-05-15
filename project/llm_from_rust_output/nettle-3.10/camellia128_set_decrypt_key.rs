use libc::{c_uchar, c_ulong};

pub type uint8_t = c_uchar;
pub type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Camellia128Ctx {
    pub keys: [uint64_t; 24],
}

extern "C" {
    fn nettle_camellia128_set_encrypt_key(ctx: *mut Camellia128Ctx, key: *const uint8_t);
    fn _nettle_camellia_invert_key(nkeys: libc::c_uint, dst: *mut uint64_t, src: *const uint64_t);
}

pub fn camellia128_invert_key(dst: &mut Camellia128Ctx, src: &Camellia128Ctx) {
    unsafe {
        _nettle_camellia_invert_key(
            24,
            dst.keys.as_mut_ptr(),
            src.keys.as_ptr(),
        );
    }
}

pub fn camellia_set_decrypt_key(ctx: &mut Camellia128Ctx, key: &[uint8_t]) {
    unsafe {
        nettle_camellia128_set_encrypt_key(ctx, key.as_ptr());
    }
    camellia128_invert_key(ctx, ctx);
}