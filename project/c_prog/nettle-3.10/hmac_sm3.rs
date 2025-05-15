use ::libc;
extern "C" {
    static nettle_sm3: nettle_hash;
    fn nettle_sm3_update(ctx: *mut sm3_ctx, length: size_t, data: *const uint8_t);
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
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
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
pub struct sm3_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: libc::c_uint,
    pub block: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_sm3_ctx {
    pub outer: sm3_ctx,
    pub inner: sm3_ctx,
    pub state: sm3_ctx,
}
#[no_mangle]
pub unsafe extern "C" fn nettle_hmac_sm3_set_key(
    mut ctx: *mut hmac_sm3_ctx,
    mut key_length: size_t,
    mut key: *const uint8_t,
) {
    nettle_hmac_set_key(
        &mut (*ctx).outer as *mut sm3_ctx as *mut libc::c_void,
        &mut (*ctx).inner as *mut sm3_ctx as *mut libc::c_void,
        &mut (*ctx).state as *mut sm3_ctx as *mut libc::c_void,
        &nettle_sm3,
        key_length,
        key,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_hmac_sm3_update(
    mut ctx: *mut hmac_sm3_ctx,
    mut length: size_t,
    mut data: *const uint8_t,
) {
    nettle_sm3_update(&mut (*ctx).state, length, data);
}
#[no_mangle]
pub unsafe extern "C" fn nettle_hmac_sm3_digest(
    mut ctx: *mut hmac_sm3_ctx,
    mut length: size_t,
    mut digest: *mut uint8_t,
) {
    nettle_hmac_digest(
        &mut (*ctx).outer as *mut sm3_ctx as *const libc::c_void,
        &mut (*ctx).inner as *mut sm3_ctx as *const libc::c_void,
        &mut (*ctx).state as *mut sm3_ctx as *mut libc::c_void,
        &nettle_sm3,
        length,
        digest,
    );
}
