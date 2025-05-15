use std::mem::MaybeUninit;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Clone, Copy)]
pub struct Sha256Ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}

#[derive(Clone, Copy)]
pub struct HmacSha256Ctx {
    pub outer: Sha256Ctx,
    pub inner: Sha256Ctx,
    pub state: Sha256Ctx,
}

pub struct NettleHash {
    pub name: &'static str,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: fn(&mut Sha256Ctx),
    pub update: fn(&mut Sha256Ctx, size_t, &[uint8_t]),
    pub digest: fn(&mut Sha256Ctx, size_t, &mut [uint8_t]),
}

pub static NETTLE_SHA224: NettleHash = NettleHash {
    name: "sha224",
    context_size: std::mem::size_of::<Sha256Ctx>() as u32,
    digest_size: 28,
    block_size: 64,
    init: sha256_init,
    update: sha256_update,
    digest: sha256_digest,
};

pub fn hmac_sha224_set_key(ctx: &mut HmacSha256Ctx, key: &[uint8_t]) {
    unsafe {
        let outer = &mut ctx.outer as *mut _ as *mut std::ffi::c_void;
        let inner = &mut ctx.inner as *mut _ as *mut std::ffi::c_void;
        let state = &mut ctx.state as *mut _ as *mut std::ffi::c_void;
        
        nettle_hmac_set_key(
            outer,
            inner,
            state,
            &NETTLE_SHA224,
            key.len(),
            key.as_ptr(),
        );
    }
}

pub fn hmac_sha224_digest(ctx: &mut HmacSha256Ctx, digest: &mut [uint8_t]) {
    unsafe {
        let outer = &ctx.outer as *const _ as *const std::ffi::c_void;
        let inner = &ctx.inner as *const _ as *const std::ffi::c_void;
        let state = &mut ctx.state as *mut _ as *mut std::ffi::c_void;
        
        nettle_hmac_digest(
            outer,
            inner,
            state,
            &NETTLE_SHA224,
            digest.len(),
            digest.as_mut_ptr(),
        );
    }
}

extern "C" {
    fn nettle_hmac_set_key(
        outer: *mut std::ffi::c_void,
        inner: *mut std::ffi::c_void,
        state: *mut std::ffi::c_void,
        hash: *const NettleHash,
        length: size_t,
        key: *const uint8_t,
    );
    
    fn nettle_hmac_digest(
        outer: *const std::ffi::c_void,
        inner: *const std::ffi::c_void,
        state: *mut std::ffi::c_void,
        hash: *const NettleHash,
        length: size_t,
        digest: *mut uint8_t,
    );
}

fn sha256_init(_ctx: &mut Sha256Ctx) {
    unimplemented!()
}

fn sha256_update(_ctx: &mut Sha256Ctx, _length: size_t, _data: &[uint8_t]) {
    unimplemented!()
}

fn sha256_digest(_ctx: &mut Sha256Ctx, _length: size_t, _digest: &mut [uint8_t]) {
    unimplemented!()
}