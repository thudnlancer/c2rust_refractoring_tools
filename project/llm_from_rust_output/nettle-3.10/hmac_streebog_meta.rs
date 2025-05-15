use std::mem;
use std::os::raw::{c_uchar, c_ulong, c_uint, c_void, c_char};

type size_t = c_ulong;
type uint8_t = c_uchar;
type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct streebog512_ctx {
    pub state: [uint64_t; 8],
    pub count: [uint64_t; 8],
    pub sigma: [uint64_t; 8],
    pub index: c_uint,
    pub block: [uint8_t; 64],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_streebog512_ctx {
    pub outer: streebog512_ctx,
    pub inner: streebog512_ctx,
    pub state: streebog512_ctx,
}

type NettleSetKeyFunc = fn(ctx: &mut hmac_streebog512_ctx, key: &[uint8_t]);
type NettleHashUpdateFunc = fn(ctx: &mut hmac_streebog512_ctx, data: &[uint8_t]);
type NettleHashDigestFunc = fn(ctx: &mut hmac_streebog512_ctx, digest: &mut [uint8_t]);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NettleMac {
    pub name: &'static str,
    pub context_size: c_uint,
    pub digest_size: c_uint,
    pub key_size: c_uint,
    pub set_key: Option<NettleSetKeyFunc>,
    pub update: Option<NettleHashUpdateFunc>,
    pub digest: Option<NettleHashDigestFunc>,
}

fn hmac_streebog256_set_key(ctx: &mut hmac_streebog512_ctx, key: &[uint8_t]) {
    assert_eq!(key.len(), 32);
    // Call the actual implementation
}

fn hmac_streebog256_digest(ctx: &mut hmac_streebog512_ctx, digest: &mut [uint8_t]) {
    assert_eq!(digest.len(), 32);
    // Call the actual implementation
}

fn hmac_streebog512_set_key(ctx: &mut hmac_streebog512_ctx, key: &[uint8_t]) {
    assert_eq!(key.len(), 64);
    // Call the actual implementation
}

fn hmac_streebog512_update(ctx: &mut hmac_streebog512_ctx, data: &[uint8_t]) {
    // Call the actual implementation
}

fn hmac_streebog512_digest(ctx: &mut hmac_streebog512_ctx, digest: &mut [uint8_t]) {
    assert_eq!(digest.len(), 64);
    // Call the actual implementation
}

pub static NETTLE_HMAC_STREEBOG256: NettleMac = NettleMac {
    name: "hmac_streebog256",
    context_size: mem::size_of::<hmac_streebog512_ctx>() as c_uint,
    digest_size: 32,
    key_size: 32,
    set_key: Some(hmac_streebog256_set_key),
    update: Some(hmac_streebog512_update),
    digest: Some(hmac_streebog256_digest),
};

pub static NETTLE_HMAC_STREEBOG512: NettleMac = NettleMac {
    name: "hmac_streebog512",
    context_size: mem::size_of::<hmac_streebog512_ctx>() as c_uint,
    digest_size: 64,
    key_size: 64,
    set_key: Some(hmac_streebog512_set_key),
    update: Some(hmac_streebog512_update),
    digest: Some(hmac_streebog512_digest),
};