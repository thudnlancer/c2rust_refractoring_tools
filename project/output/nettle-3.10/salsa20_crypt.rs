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
    fn _nettle_salsa20_crypt(
        ctx: *mut salsa20_ctx,
        rounds: u32,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
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
pub unsafe extern "C" fn nettle_salsa20_crypt(
    mut ctx: *mut salsa20_ctx,
    mut length: size_t,
    mut c: *mut uint8_t,
    mut m: *const uint8_t,
) {
    if length == 0 {
        return;
    }
    _nettle_salsa20_crypt(ctx, 20 as i32 as u32, length, c, m);
}