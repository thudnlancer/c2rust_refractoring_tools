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
    fn nettle_md5_digest(ctx: *mut md5_ctx, length: size_t, digest: *mut uint8_t);
    fn nettle_md5_update(ctx: *mut md5_ctx, length: size_t, data: *const uint8_t);
    fn nettle_md5_init(ctx: *mut md5_ctx);
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_ctx {
    pub state: [uint32_t; 4],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}
pub type MD5_CTX = md5_ctx;
#[no_mangle]
pub unsafe extern "C" fn nettle_MD5Init(mut ctx: *mut MD5_CTX) {
    nettle_md5_init(ctx);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_MD5Update(
    mut ctx: *mut MD5_CTX,
    mut data: *const u8,
    mut length: u32,
) {
    nettle_md5_update(ctx, length as size_t, data);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_MD5Final(mut out: *mut u8, mut ctx: *mut MD5_CTX) {
    nettle_md5_digest(ctx, 16 as i32 as size_t, out);
}