use std::mem;
use std::os::raw::{c_char, c_uchar, c_ulong, c_uint, c_void};

type size_t = c_ulong;
type uint8_t = c_uchar;
type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_ctx {
    pub state: [uint64_t; 8],
    pub count_low: uint64_t,
    pub count_high: uint64_t,
    pub index: c_uint,
    pub block: [uint8_t; 128],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_sha512_ctx {
    pub outer: sha512_ctx,
    pub inner: sha512_ctx,
    pub state: sha512_ctx,
}

type NettleSetKeyFunc = unsafe extern "C" fn(*mut c_void, *const uint8_t);
type NettleHashUpdateFunc = unsafe extern "C" fn(*mut c_void, size_t, *const uint8_t);
type NettleHashDigestFunc = unsafe extern "C" fn(*mut c_void, size_t, *mut uint8_t);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NettleMac {
    pub name: *const c_char,
    pub context_size: c_uint,
    pub digest_size: c_uint,
    pub key_size: c_uint,
    pub set_key: Option<NettleSetKeyFunc>,
    pub update: Option<NettleHashUpdateFunc>,
    pub digest: Option<NettleHashDigestFunc>,
}

extern "C" {
    fn nettle_hmac_sha512_update(ctx: *mut hmac_sha512_ctx, length: size_t, data: *const uint8_t);
    fn nettle_hmac_sha384_set_key(ctx: *mut hmac_sha512_ctx, key_length: size_t, key: *const uint8_t);
    fn nettle_hmac_sha384_digest(ctx: *mut hmac_sha512_ctx, length: size_t, digest: *mut uint8_t);
}

unsafe extern "C" fn hmac_sha384_set_key_wrapper(ctx: *mut c_void, key: *const uint8_t) {
    nettle_hmac_sha384_set_key(ctx as *mut hmac_sha512_ctx, 48, key);
}

pub static NETTLE_HMAC_SHA384: NettleMac = NettleMac {
    name: b"hmac_sha384\0".as_ptr() as *const c_char,
    context_size: mem::size_of::<hmac_sha512_ctx>() as c_uint,
    digest_size: 48,
    key_size: 48,
    set_key: Some(hmac_sha384_set_key_wrapper),
    update: Some(nettle_hmac_sha512_update as NettleHashUpdateFunc),
    digest: Some(nettle_hmac_sha384_digest as NettleHashDigestFunc),
};