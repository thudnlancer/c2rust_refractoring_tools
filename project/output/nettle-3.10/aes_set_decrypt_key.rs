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
    fn nettle_aes192_invert_key(dst: *mut aes192_ctx, src: *const aes192_ctx);
    fn nettle_aes256_invert_key(dst: *mut aes256_ctx, src: *const aes256_ctx);
    fn nettle_aes_set_encrypt_key(
        ctx: *mut aes_ctx,
        length: size_t,
        key: *const uint8_t,
    );
    fn nettle_aes128_invert_key(dst: *mut aes128_ctx, src: *const aes128_ctx);
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
pub unsafe extern "C" fn nettle_aes_invert_key(
    mut dst: *mut aes_ctx,
    mut src: *const aes_ctx,
) {
    match (*src).key_size {
        16 => {
            nettle_aes128_invert_key(&mut (*dst).u.ctx128, &(*src).u.ctx128);
        }
        24 => {
            nettle_aes192_invert_key(&mut (*dst).u.ctx192, &(*src).u.ctx192);
        }
        32 => {
            nettle_aes256_invert_key(&mut (*dst).u.ctx256, &(*src).u.ctx256);
        }
        _ => {
            abort();
        }
    }
    (*dst).key_size = (*src).key_size;
}
#[no_mangle]
pub unsafe extern "C" fn nettle_aes_set_decrypt_key(
    mut ctx: *mut aes_ctx,
    mut keysize: size_t,
    mut key: *const uint8_t,
) {
    nettle_aes_set_encrypt_key(ctx, keysize, key);
    nettle_aes_invert_key(ctx, ctx);
}