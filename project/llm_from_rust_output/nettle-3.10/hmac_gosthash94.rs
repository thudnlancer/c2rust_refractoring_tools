use std::ffi::CStr;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Copy, Clone)]
pub struct NettleHash {
    pub name: &'static CStr,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: fn(&mut [u8]),
    pub update: fn(&mut [u8], size_t, &[uint8_t]),
    pub digest: fn(&mut [u8], size_t, &mut [uint8_t]),
}

#[derive(Copy, Clone)]
pub struct GostHash94Ctx {
    pub hash: [uint32_t; 8],
    pub sum: [uint32_t; 8],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 32],
}

#[derive(Copy, Clone)]
pub struct HmacGostHash94Ctx {
    pub outer: GostHash94Ctx,
    pub inner: GostHash94Ctx,
    pub state: GostHash94Ctx,
}

#[derive(Copy, Clone)]
pub struct HmacGostHash94CpCtx {
    pub outer: GostHash94Ctx,
    pub inner: GostHash94Ctx,
    pub state: GostHash94Ctx,
}

pub static NETTLE_GOSTHASH94: NettleHash = NettleHash {
    name: CStr::from_bytes_with_nul(b"gosthash94\0").unwrap(),
    context_size: std::mem::size_of::<GostHash94Ctx>() as u32,
    digest_size: 32,
    block_size: 32,
    init: gosthash94_init,
    update: gosthash94_update,
    digest: gosthash94_digest,
};

pub static NETTLE_GOSTHASH94CP: NettleHash = NettleHash {
    name: CStr::from_bytes_with_nul(b"gosthash94cp\0").unwrap(),
    context_size: std::mem::size_of::<GostHash94Ctx>() as u32,
    digest_size: 32,
    block_size: 32,
    init: gosthash94cp_init,
    update: gosthash94cp_update,
    digest: gosthash94cp_digest,
};

pub fn hmac_gosthash94_set_key(
    ctx: &mut HmacGostHash94Ctx,
    key_length: size_t,
    key: &[uint8_t],
) {
    hmac_set_key(
        &mut ctx.outer,
        &mut ctx.inner,
        &mut ctx.state,
        &NETTLE_GOSTHASH94,
        key_length,
        key,
    );
}

pub fn hmac_gosthash94_update(
    ctx: &mut HmacGostHash94Ctx,
    length: size_t,
    data: &[uint8_t],
) {
    (NETTLE_GOSTHASH94.update)(&mut ctx.state, length, data);
}

pub fn hmac_gosthash94_digest(
    ctx: &mut HmacGostHash94Ctx,
    length: size_t,
    digest: &mut [uint8_t],
) {
    hmac_digest(
        &ctx.outer,
        &ctx.inner,
        &mut ctx.state,
        &NETTLE_GOSTHASH94,
        length,
        digest,
    );
}

pub fn hmac_gosthash94cp_set_key(
    ctx: &mut HmacGostHash94CpCtx,
    key_length: size_t,
    key: &[uint8_t],
) {
    hmac_set_key(
        &mut ctx.outer,
        &mut ctx.inner,
        &mut ctx.state,
        &NETTLE_GOSTHASH94CP,
        key_length,
        key,
    );
}

pub fn hmac_gosthash94cp_update(
    ctx: &mut HmacGostHash94CpCtx,
    length: size_t,
    data: &[uint8_t],
) {
    (NETTLE_GOSTHASH94CP.update)(&mut ctx.state, length, data);
}

pub fn hmac_gosthash94cp_digest(
    ctx: &mut HmacGostHash94CpCtx,
    length: size_t,
    digest: &mut [uint8_t],
) {
    hmac_digest(
        &ctx.outer,
        &ctx.inner,
        &mut ctx.state,
        &NETTLE_GOSTHASH94CP,
        length,
        digest,
    );
}

// These would be implemented elsewhere in the module
fn gosthash94_init(_ctx: &mut [u8]) {}
fn gosthash94_update(_ctx: &mut [u8], _length: size_t, _msg: &[uint8_t]) {}
fn gosthash94_digest(_ctx: &mut [u8], _length: size_t, _digest: &mut [uint8_t]) {}
fn gosthash94cp_init(_ctx: &mut [u8]) {}
fn gosthash94cp_update(_ctx: &mut [u8], _length: size_t, _msg: &[uint8_t]) {}
fn gosthash94cp_digest(_ctx: &mut [u8], _length: size_t, _digest: &mut [uint8_t]) {}
fn hmac_set_key(
    _outer: &mut GostHash94Ctx,
    _inner: &mut GostHash94Ctx,
    _state: &mut GostHash94Ctx,
    _hash: &NettleHash,
    _length: size_t,
    _key: &[uint8_t],
) {}
fn hmac_digest(
    _outer: &GostHash94Ctx,
    _inner: &GostHash94Ctx,
    _state: &mut GostHash94Ctx,
    _hash: &NettleHash,
    _length: size_t,
    _digest: &mut [uint8_t],
) {}