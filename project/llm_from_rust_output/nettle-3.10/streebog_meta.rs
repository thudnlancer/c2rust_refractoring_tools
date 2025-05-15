use std::mem;

type SizeT = usize;
type Uint8T = u8;
type Uint64T = u64;

#[derive(Clone, Copy)]
pub struct Streebog512Ctx {
    pub state: [Uint64T; 8],
    pub count: [Uint64T; 8],
    pub sigma: [Uint64T; 8],
    pub index: u32,
    pub block: [Uint8T; 64],
}

pub struct NettleHash {
    pub name: &'static str,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: Option<fn(&mut Streebog512Ctx)>,
    pub update: Option<fn(&mut Streebog512Ctx, SizeT, &[Uint8T])>,
    pub digest: Option<fn(&mut Streebog512Ctx, SizeT, &mut [Uint8T])>,
}

pub fn streebog512_init(ctx: &mut Streebog512Ctx) {
    unsafe { nettle_streebog512_init(ctx) }
}

pub fn streebog512_update(ctx: &mut Streebog512Ctx, length: SizeT, data: &[Uint8T]) {
    unsafe { nettle_streebog512_update(ctx, length, data.as_ptr()) }
}

pub fn streebog512_digest(ctx: &mut Streebog512Ctx, length: SizeT, digest: &mut [Uint8T]) {
    unsafe { nettle_streebog512_digest(ctx, length, digest.as_mut_ptr()) }
}

pub fn streebog256_init(ctx: &mut Streebog512Ctx) {
    unsafe { nettle_streebog256_init(ctx) }
}

pub fn streebog256_digest(ctx: &mut Streebog512Ctx, length: SizeT, digest: &mut [Uint8T]) {
    unsafe { nettle_streebog256_digest(ctx, length, digest.as_mut_ptr()) }
}

pub const NETTLE_STREEBOG512: NettleHash = NettleHash {
    name: "streebog512",
    context_size: mem::size_of::<Streebog512Ctx>() as u32,
    digest_size: 64,
    block_size: 64,
    init: Some(streebog512_init),
    update: Some(streebog512_update),
    digest: Some(streebog512_digest),
};

pub const NETTLE_STREEBOG256: NettleHash = NettleHash {
    name: "streebog256",
    context_size: mem::size_of::<Streebog512Ctx>() as u32,
    digest_size: 32,
    block_size: 64,
    init: Some(streebog256_init),
    update: Some(streebog512_update),
    digest: Some(streebog256_digest),
};

extern "C" {
    fn nettle_streebog512_init(ctx: *mut Streebog512Ctx);
    fn nettle_streebog512_update(ctx: *mut Streebog512Ctx, length: SizeT, data: *const Uint8T);
    fn nettle_streebog512_digest(ctx: *mut Streebog512Ctx, length: SizeT, digest: *mut Uint8T);
    fn nettle_streebog256_init(ctx: *mut Streebog512Ctx);
    fn nettle_streebog256_digest(ctx: *mut Streebog512Ctx, length: SizeT, digest: *mut Uint8T);
}