use std::mem;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Clone, Copy)]
pub struct Sha1Ctx {
    pub state: [uint32_t; 5],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}

#[derive(Clone, Copy)]
pub struct HmacSha1Ctx {
    pub outer: Sha1Ctx,
    pub inner: Sha1Ctx,
    pub state: Sha1Ctx,
}

pub struct NettleMac {
    pub name: &'static str,
    pub context_size: u32,
    pub digest_size: u32,
    pub key_size: u32,
    pub set_key: fn(&mut HmacSha1Ctx, &[uint8_t]),
    pub update: fn(&mut HmacSha1Ctx, &[uint8_t]),
    pub digest: fn(&mut HmacSha1Ctx, &mut [uint8_t]),
}

fn hmac_sha1_set_key(ctx: &mut HmacSha1Ctx, key: &[uint8_t]) {
    // Implementation of nettle_hmac_sha1_set_key
}

fn hmac_sha1_update(ctx: &mut HmacSha1Ctx, data: &[uint8_t]) {
    // Implementation of nettle_hmac_sha1_update
}

fn hmac_sha1_digest(ctx: &mut HmacSha1Ctx, digest: &mut [uint8_t]) {
    // Implementation of nettle_hmac_sha1_digest
}

pub static NETTLE_HMAC_SHA1: NettleMac = NettleMac {
    name: "hmac_sha1",
    context_size: mem::size_of::<HmacSha1Ctx>() as u32,
    digest_size: 20,
    key_size: 20,
    set_key: hmac_sha1_set_key,
    update: hmac_sha1_update,
    digest: hmac_sha1_digest,
};