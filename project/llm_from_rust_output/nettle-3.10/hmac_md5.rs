use std::ffi::CStr;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Clone, Copy)]
pub struct NettleHash {
    pub name: &'static CStr,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: fn(&mut [u8]),
    pub update: fn(&mut [u8], &[u8]),
    pub digest: fn(&mut [u8], &mut [u8]),
}

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

pub static NETTLE_MD5: NettleHash = NettleHash {
    name: unsafe { CStr::from_bytes_with_nul_unchecked(b"md5\0") },
    context_size: std::mem::size_of::<Md5Ctx>() as u32,
    digest_size: 16,
    block_size: 64,
    init: md5_init,
    update: md5_update,
    digest: md5_digest,
};

pub fn hmac_md5_set_key(ctx: &mut HmacMd5Ctx, key: &[u8]) {
    hmac_set_key(&mut ctx.outer, &mut ctx.inner, &mut ctx.state, &NETTLE_MD5, key);
}

pub fn hmac_md5_update(ctx: &mut HmacMd5Ctx, data: &[u8]) {
    (NETTLE_MD5.update)(&mut ctx.state.block, data);
}

pub fn hmac_md5_digest(ctx: &mut HmacMd5Ctx, digest: &mut [u8]) {
    hmac_digest(&ctx.outer, &ctx.inner, &mut ctx.state, &NETTLE_MD5, digest);
}

fn md5_init(_ctx: &mut [u8]) {
    unimplemented!()
}

fn md5_update(_ctx: &mut [u8], _data: &[u8]) {
    unimplemented!()
}

fn md5_digest(_ctx: &mut [u8], _digest: &mut [u8]) {
    unimplemented!()
}

fn hmac_set_key(
    outer: &mut Md5Ctx,
    inner: &mut Md5Ctx,
    state: &mut Md5Ctx,
    hash: &NettleHash,
    key: &[u8],
) {
    unimplemented!()
}

fn hmac_digest(
    outer: &Md5Ctx,
    inner: &Md5Ctx,
    state: &mut Md5Ctx,
    hash: &NettleHash,
    digest: &mut [u8],
) {
    unimplemented!()
}