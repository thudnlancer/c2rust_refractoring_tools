use std::mem::size_of;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct hmac_sha256_ctx {
    pub outer: sha256_ctx,
    pub inner: sha256_ctx,
    pub state: sha256_ctx,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_mac {
    pub name: &'static str,
    pub context_size: u32,
    pub digest_size: u32,
    pub key_size: u32,
    pub set_key: fn(&mut hmac_sha256_ctx, &[uint8_t]),
    pub update: fn(&mut hmac_sha256_ctx, &[uint8_t]),
    pub digest: fn(&mut hmac_sha256_ctx, &mut [uint8_t]),
}

fn hmac_sha256_set_key(ctx: &mut hmac_sha256_ctx, key: &[uint8_t]) {
    assert_eq!(key.len(), 32);
    // Implementation would call the actual HMAC-SHA256 key setting function
}

fn hmac_sha256_update(ctx: &mut hmac_sha256_ctx, data: &[uint8_t]) {
    // Implementation would call the actual HMAC-SHA256 update function
}

fn hmac_sha256_digest(ctx: &mut hmac_sha256_ctx, digest: &mut [uint8_t]) {
    assert_eq!(digest.len(), 32);
    // Implementation would call the actual HMAC-SHA256 digest function
}

pub const NETTLE_HMAC_SHA256: nettle_mac = nettle_mac {
    name: "hmac_sha256",
    context_size: size_of::<hmac_sha256_ctx>() as u32,
    digest_size: 32,
    key_size: 32,
    set_key: hmac_sha256_set_key,
    update: hmac_sha256_update,
    digest: hmac_sha256_digest,
};