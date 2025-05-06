#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    fn abort() -> !;
    fn nettle_aes256_set_encrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes192_set_encrypt_key(ctx: *mut aes192_ctx, key: *const uint8_t);
    fn nettle_aes128_set_encrypt_key(ctx: *mut aes128_ctx, key: *const uint8_t);
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes192_ctx {
    pub keys: [uint32_t; 52],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes_ctx {
    pub key_size: u32,
    pub u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub ctx128: aes128_ctx,
    pub ctx192: aes192_ctx,
    pub ctx256: aes256_ctx,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes_set_encrypt_key(
    mut ctx: *mut aes_ctx,
    mut key_size: size_t,
    mut key: *const uint8_t,
) {
    match key_size {
        16 => {
            nettle_aes128_set_encrypt_key(&mut (*ctx).u.ctx128, key);
        }
        24 => {
            nettle_aes192_set_encrypt_key(&mut (*ctx).u.ctx192, key);
        }
        32 => {
            nettle_aes256_set_encrypt_key(&mut (*ctx).u.ctx256, key);
        }
        _ => {
            abort();
        }
    }
    (*ctx).key_size = key_size as u32;
}