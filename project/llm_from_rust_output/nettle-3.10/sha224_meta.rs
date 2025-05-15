use std::mem;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Sha256Ctx {
    pub state: [u32; 8],
    pub count: u64,
    pub index: u32,
    pub block: [u8; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NettleHash {
    pub name: &'static str,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: Option<fn(&mut Sha256Ctx)>,
    pub update: Option<fn(&mut Sha256Ctx, usize, &[u8])>,
    pub digest: Option<fn(&mut Sha256Ctx, usize, &mut [u8])>,
}

extern "C" {
    fn nettle_sha256_update(ctx: *mut Sha256Ctx, length: usize, data: *const u8);
    fn nettle_sha224_init(ctx: *mut Sha256Ctx);
    fn nettle_sha224_digest(ctx: *mut Sha256Ctx, length: usize, digest: *mut u8);
}

pub fn sha224_update(ctx: &mut Sha256Ctx, data: &[u8]) {
    unsafe {
        nettle_sha256_update(ctx, data.len(), data.as_ptr());
    }
}

pub fn sha224_init(ctx: &mut Sha256Ctx) {
    unsafe {
        nettle_sha224_init(ctx);
    }
}

pub fn sha224_digest(ctx: &mut Sha256Ctx, digest: &mut [u8]) {
    unsafe {
        nettle_sha224_digest(ctx, digest.len(), digest.as_mut_ptr());
    }
}

pub static NETTLE_SHA224: NettleHash = NettleHash {
    name: "sha224",
    context_size: mem::size_of::<Sha256Ctx>() as u32,
    digest_size: 28,
    block_size: 64,
    init: Some(|ctx| sha224_init(ctx)),
    update: Some(|ctx, len, data| sha224_update(ctx, data)),
    digest: Some(|ctx, len, digest| sha224_digest(ctx, digest)),
};