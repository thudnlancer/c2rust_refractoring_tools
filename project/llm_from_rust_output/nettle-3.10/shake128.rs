use std::mem::MaybeUninit;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint64_t = u64;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Sha3State {
    pub a: [uint64_t; 25],
}

impl Default for Sha3State {
    fn default() -> Self {
        Sha3State { a: [0; 25] }
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct Sha3_128Ctx {
    pub state: Sha3State,
    pub index: u32,
    pub block: [uint8_t; 168],
}

impl Default for Sha3_128Ctx {
    fn default() -> Self {
        Sha3_128Ctx {
            state: Sha3State::default(),
            index: 0,
            block: [0; 168],
        }
    }
}

pub fn sha3_128_init(ctx: &mut Sha3_128Ctx) {
    *ctx = Sha3_128Ctx::default();
}

pub fn sha3_128_update(ctx: &mut Sha3_128Ctx, data: &[u8]) {
    ctx.index = nettle_sha3_update(
        &mut ctx.state,
        168,
        &mut ctx.block,
        ctx.index,
        data.len(),
        data,
    );
}

pub fn sha3_128_shake(ctx: &mut Sha3_128Ctx, dst: &mut [u8]) {
    nettle_sha3_shake(
        &mut ctx.state,
        168,
        &mut ctx.block,
        ctx.index,
        dst.len(),
        dst,
    );
    sha3_128_init(ctx);
}

pub fn sha3_128_shake_output(ctx: &mut Sha3_128Ctx, digest: &mut [u8]) {
    ctx.index = nettle_sha3_shake_output(
        &mut ctx.state,
        168,
        &mut ctx.block,
        ctx.index,
        digest.len(),
        digest,
    );
}

fn nettle_sha3_update(
    state: &mut Sha3State,
    block_size: u32,
    block: &mut [u8],
    pos: u32,
    length: usize,
    data: &[u8],
) -> u32 {
    // Implementation of SHA3 update
    unimplemented!()
}

fn nettle_sha3_shake(
    state: &mut Sha3State,
    block_size: u32,
    block: &mut [u8],
    index: u32,
    length: usize,
    dst: &mut [u8],
) {
    // Implementation of SHA3 shake
    unimplemented!()
}

fn nettle_sha3_shake_output(
    state: &mut Sha3State,
    block_size: u32,
    block: &mut [u8],
    index: u32,
    length: usize,
    dst: &mut [u8],
) -> u32 {
    // Implementation of SHA3 shake output
    unimplemented!()
}