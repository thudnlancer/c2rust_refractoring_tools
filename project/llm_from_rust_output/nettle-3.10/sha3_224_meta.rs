use std::mem;

pub type Size = usize;
pub type Uint8 = u8;
pub type Uint64 = u64;

#[derive(Clone, Copy)]
pub struct Sha3State {
    a: [Uint64; 25],
}

#[derive(Clone, Copy)]
pub struct Sha3224Ctx {
    state: Sha3State,
    index: u32,
    block: [Uint8; 144],
}

pub struct NettleHash {
    name: &'static str,
    context_size: u32,
    digest_size: u32,
    block_size: u32,
    init: fn(&mut Sha3224Ctx),
    update: fn(&mut Sha3224Ctx, Size, &[Uint8]),
    digest: fn(&mut Sha3224Ctx, Size, &mut [Uint8]),
}

fn sha3_224_init(ctx: &mut Sha3224Ctx) {
    // Implementation of SHA3-224 initialization
    ctx.state.a = [0; 25];
    ctx.index = 0;
}

fn sha3_224_update(ctx: &mut Sha3224Ctx, length: Size, data: &[Uint8]) {
    // Implementation of SHA3-224 update
}

fn sha3_224_digest(ctx: &mut Sha3224Ctx, length: Size, digest: &mut [Uint8]) {
    // Implementation of SHA3-224 digest
}

pub static NETTLE_SHA3_224: NettleHash = NettleHash {
    name: "sha3_224",
    context_size: mem::size_of::<Sha3224Ctx>() as u32,
    digest_size: 28,
    block_size: 144,
    init: sha3_224_init,
    update: sha3_224_update,
    digest: sha3_224_digest,
};