use std::mem;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Clone, Copy)]
pub struct Md5Ctx {
    pub state: [uint32_t; 4],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}

#[derive(Clone, Copy)]
pub struct HmacMd5Ctx {
    pub outer: Md5Ctx,
    pub inner: Md5Ctx,
    pub state: Md5Ctx,
}

pub struct NettleMac {
    pub name: &'static str,
    pub context_size: u32,
    pub digest_size: u32,
    pub key_size: u32,
    pub set_key: fn(&mut HmacMd5Ctx, &[uint8_t]),
    pub update: fn(&mut HmacMd5Ctx, &[uint8_t]),
    pub digest: fn(&mut HmacMd5Ctx, &mut [uint8_t]),
}

fn hmac_md5_set_key(ctx: &mut HmacMd5Ctx, key: &[uint8_t]) {
    // Implementation of nettle_hmac_md5_set_key
}

fn hmac_md5_update(ctx: &mut HmacMd5Ctx, data: &[uint8_t]) {
    // Implementation of nettle_hmac_md5_update
}

fn hmac_md5_digest(ctx: &mut HmacMd5Ctx, digest: &mut [uint8_t]) {
    // Implementation of nettle_hmac_md5_digest
}

pub static NETTLE_HMAC_MD5: NettleMac = NettleMac {
    name: "hmac_md5",
    context_size: mem::size_of::<HmacMd5Ctx>() as u32,
    digest_size: 16,
    key_size: 16,
    set_key: hmac_md5_set_key,
    update: hmac_md5_update,
    digest: hmac_md5_digest,
};