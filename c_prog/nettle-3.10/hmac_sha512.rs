#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
extern "C" {
    static nettle_sha512: nettle_hash;
    fn nettle_sha512_update(ctx: *mut sha512_ctx, length: size_t, data: *const uint8_t);
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
pub type size_t = libc::c_ulong;
pub type __uint8_t = libc::c_uchar;
pub type __uint64_t = libc::c_ulong;
pub type uint8_t = __uint8_t;
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
    pub name: *const libc::c_char,
    pub context_size: libc::c_uint,
    pub digest_size: libc::c_uint,
    pub block_size: libc::c_uint,
    pub init: Option::<nettle_hash_init_func>,
    pub update: Option::<nettle_hash_update_func>,
    pub digest: Option::<nettle_hash_digest_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_ctx {
    pub state: [uint64_t; 8],
    pub count_low: uint64_t,
    pub count_high: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_sha512_ctx {
    pub outer: sha512_ctx,
    pub inner: sha512_ctx,
    pub state: sha512_ctx,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_hmac_sha512_set_key(
    mut ctx: *mut hmac_sha512_ctx,
    mut key_length: size_t,
    mut key: *const uint8_t,
) {
    nettle_hmac_set_key(
        &mut (*ctx).outer as *mut sha512_ctx as *mut libc::c_void,
        &mut (*ctx).inner as *mut sha512_ctx as *mut libc::c_void,
        &mut (*ctx).state as *mut sha512_ctx as *mut libc::c_void,
        &nettle_sha512,
        key_length,
        key,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_hmac_sha512_update(
    mut ctx: *mut hmac_sha512_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    nettle_sha512_update(&mut (*ctx).state, length, data);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_hmac_sha512_digest(
    mut ctx: *mut hmac_sha512_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    nettle_hmac_digest(
        &mut (*ctx).outer as *mut sha512_ctx as *const libc::c_void,
        &mut (*ctx).inner as *mut sha512_ctx as *const libc::c_void,
        &mut (*ctx).state as *mut sha512_ctx as *mut libc::c_void,
        &nettle_sha512,
        length,
        digest,
    );
}
