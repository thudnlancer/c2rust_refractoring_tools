use std::mem;
use std::os::raw::{c_uchar, c_uint, c_ulong};

type size_t = c_ulong;
type uint8_t = c_uchar;
type uint32_t = c_uint;
type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: c_uint,
    pub block: [uint8_t; 64],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_sha256_ctx {
    pub outer: sha256_ctx,
    pub inner: sha256_ctx,
    pub state: sha256_ctx,
}

pub struct NettleMac {
    pub name: &'static str,
    pub context_size: c_uint,
    pub digest_size: c_uint,
    pub key_size: c_uint,
    pub set_key: fn(&mut hmac_sha256_ctx, &[uint8_t]),
    pub update: fn(&mut hmac_sha256_ctx, &[uint8_t]),
    pub digest: fn(&mut hmac_sha256_ctx, &mut [uint8_t]),
}

fn hmac_sha224_set_key(ctx: &mut hmac_sha256_ctx, key: &[uint8_t]) {
    assert_eq!(key.len(), 28);
    // Call to external C function should be wrapped in unsafe block
    unsafe {
        extern "C" {
            fn nettle_hmac_sha224_set_key(
                ctx: *mut hmac_sha256_ctx,
                key_length: size_t,
                key: *const uint8_t,
            );
        }
        nettle_hmac_sha224_set_key(ctx, 28, key.as_ptr());
    }
}

fn hmac_sha256_update(ctx: &mut hmac_sha256_ctx, data: &[uint8_t]) {
    unsafe {
        extern "C" {
            fn nettle_hmac_sha256_update(
                ctx: *mut hmac_sha256_ctx,
                length: size_t,
                data: *const uint8_t,
            );
        }
        nettle_hmac_sha256_update(ctx, data.len() as size_t, data.as_ptr());
    }
}

fn hmac_sha224_digest(ctx: &mut hmac_sha256_ctx, digest: &mut [uint8_t]) {
    assert_eq!(digest.len(), 28);
    unsafe {
        extern "C" {
            fn nettle_hmac_sha224_digest(
                ctx: *mut hmac_sha256_ctx,
                length: size_t,
                digest: *mut uint8_t,
            );
        }
        nettle_hmac_sha224_digest(ctx, 28, digest.as_mut_ptr());
    }
}

pub const NETTLE_HMAC_SHA224: NettleMac = NettleMac {
    name: "hmac_sha224",
    context_size: mem::size_of::<hmac_sha256_ctx>() as c_uint,
    digest_size: 28,
    key_size: 28,
    set_key: hmac_sha224_set_key,
    update: hmac_sha256_update,
    digest: hmac_sha224_digest,
};