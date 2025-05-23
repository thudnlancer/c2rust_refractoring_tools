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
    fn nettle_camellia192_set_encrypt_key(
        ctx: *mut camellia256_ctx,
        key: *const uint8_t,
    );
    fn nettle_camellia256_set_encrypt_key(
        ctx: *mut camellia256_ctx,
        key: *const uint8_t,
    );
    fn _nettle_camellia_invert_key(nkeys: u32, dst: *mut uint64_t, src: *const uint64_t);
}
pub type __uint8_t = u8;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia256_ctx {
    pub keys: [uint64_t; 32],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_camellia256_invert_key(
    mut dst: *mut camellia256_ctx,
    mut src: *const camellia256_ctx,
) {
    _nettle_camellia_invert_key(
        32 as i32 as u32,
        ((*dst).keys).as_mut_ptr(),
        ((*src).keys).as_ptr(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_camellia256_set_decrypt_key(
    mut ctx: *mut camellia256_ctx,
    mut key: *const uint8_t,
) {
    nettle_camellia256_set_encrypt_key(ctx, key);
    nettle_camellia256_invert_key(ctx, ctx);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_camellia192_set_decrypt_key(
    mut ctx: *mut camellia256_ctx,
    mut key: *const uint8_t,
) {
    nettle_camellia192_set_encrypt_key(ctx, key);
    nettle_camellia256_invert_key(ctx, ctx);
}