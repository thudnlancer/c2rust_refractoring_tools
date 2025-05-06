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
    fn nettle_aes128_encrypt(
        ctx: *const aes128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes192_encrypt(
        ctx: *const aes192_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
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
pub unsafe extern "C" fn nettle_aes_encrypt(
    mut ctx: *const aes_ctx,
    mut length: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    match (*ctx).key_size {
        16 => {
            nettle_aes128_encrypt(&(*ctx).u.ctx128, length, dst, src);
        }
        24 => {
            nettle_aes192_encrypt(&(*ctx).u.ctx192, length, dst, src);
        }
        32 => {
            nettle_aes256_encrypt(&(*ctx).u.ctx256, length, dst, src);
        }
        _ => {
            abort();
        }
    };
}