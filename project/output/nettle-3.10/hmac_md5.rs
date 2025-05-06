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
    static nettle_md5: nettle_hash;
    fn nettle_md5_update(ctx: *mut md5_ctx, length: size_t, data: *const uint8_t);
    fn nettle_hmac_set_key(
        outer: *mut libc::c_void,
        inner: *mut libc::c_void,
        state: *mut libc::c_void,
        hash: *const nettle_hash,
        length: size_t,
        key: *const uint8_t,
    );
    fn nettle_hmac_digest(
        outer: *const libc::c_void,
        inner: *const libc::c_void,
        state: *mut libc::c_void,
        hash: *const nettle_hash,
        length: size_t,
        digest: *mut uint8_t,
    );
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type nettle_hash_init_func = unsafe extern "C" fn(*mut libc::c_void) -> ();
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_hash {
    pub name: *const i8,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: Option<nettle_hash_init_func>,
    pub update: Option<nettle_hash_update_func>,
    pub digest: Option<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_ctx {
    pub state: [uint32_t; 4],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_md5_ctx {
    pub outer: md5_ctx,
    pub inner: md5_ctx,
    pub state: md5_ctx,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_hmac_md5_set_key(
    mut ctx: *mut hmac_md5_ctx,
    mut key_length: size_t,
    mut key: *const uint8_t,
) {
    nettle_hmac_set_key(
        &mut (*ctx).outer as *mut md5_ctx as *mut libc::c_void,
        &mut (*ctx).inner as *mut md5_ctx as *mut libc::c_void,
        &mut (*ctx).state as *mut md5_ctx as *mut libc::c_void,
        &nettle_md5,
        key_length,
        key,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_hmac_md5_update(
    mut ctx: *mut hmac_md5_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    nettle_md5_update(&mut (*ctx).state, length, data);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_hmac_md5_digest(
    mut ctx: *mut hmac_md5_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    nettle_hmac_digest(
        &mut (*ctx).outer as *mut md5_ctx as *const libc::c_void,
        &mut (*ctx).inner as *mut md5_ctx as *const libc::c_void,
        &mut (*ctx).state as *mut md5_ctx as *mut libc::c_void,
        &nettle_md5,
        length,
        digest,
    );
}