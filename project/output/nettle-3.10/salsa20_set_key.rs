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
    fn nettle_salsa20_128_set_key(ctx: *mut salsa20_ctx, key: *const uint8_t);
    fn nettle_salsa20_256_set_key(ctx: *mut salsa20_ctx, key: *const uint8_t);
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct salsa20_ctx {
    pub input: [uint32_t; 16],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_salsa20_set_key(
    mut ctx: *mut salsa20_ctx,
    mut length: size_t,
    mut key: *const uint8_t,
) {
    match length {
        16 => {
            nettle_salsa20_128_set_key(ctx, key);
        }
        32 => {
            nettle_salsa20_256_set_key(ctx, key);
        }
        _ => {
            abort();
        }
    };
}