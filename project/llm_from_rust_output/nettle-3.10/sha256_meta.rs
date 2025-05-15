use std::mem;

pub type SizeT = usize;
pub type Uint8T = u8;
pub type Uint32T = u32;
pub type Uint64T = u64;

pub trait NettleHash {
    fn name(&self) -> &'static str;
    fn context_size(&self) -> usize;
    fn digest_size(&self) -> usize;
    fn block_size(&self) -> usize;
    fn init(&self, ctx: &mut Sha256Ctx);
    fn update(&self, ctx: &mut Sha256Ctx, data: &[u8]);
    fn digest(&self, ctx: &mut Sha256Ctx, digest: &mut [u8]);
}

#[derive(Clone, Copy)]
pub struct Sha256Ctx {
    pub state: [Uint32T; 8],
    pub count: Uint64T,
    pub index: usize,
    pub block: [Uint8T; 64],
}

pub struct Sha256;

impl NettleHash for Sha256 {
    fn name(&self) -> &'static str {
        "sha256"
    }

    fn context_size(&self) -> usize {
        mem::size_of::<Sha256Ctx>()
    }

    fn digest_size(&self) -> usize {
        32
    }

    fn block_size(&self) -> usize {
        64
    }

    fn init(&self, ctx: &mut Sha256Ctx) {
        nettle_sha256_init(ctx);
    }

    fn update(&self, ctx: &mut Sha256Ctx, data: &[u8]) {
        nettle_sha256_update(ctx, data.len(), data);
    }

    fn digest(&self, ctx: &mut Sha256Ctx, digest: &mut [u8]) {
        nettle_sha256_digest(ctx, digest.len(), digest);
    }
}

pub fn nettle_sha256_init(ctx: &mut Sha256Ctx) {
    // Implementation of SHA-256 initialization
}

pub fn nettle_sha256_update(ctx: &mut Sha256Ctx, length: SizeT, data: &[Uint8T]) {
    // Implementation of SHA-256 update
}

pub fn nettle_sha256_digest(ctx: &mut Sha256Ctx, length: SizeT, digest: &mut [Uint8T]) {
    // Implementation of SHA-256 digest
}

pub static NETTLE_SHA256: Sha256 = Sha256;