use std::ptr;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Clone, Copy)]
pub struct NettleHash {
    pub name: &'static str,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: Option<fn(*mut ())>,
    pub update: Option<fn(*mut (), size_t, *const uint8_t)>,
    pub digest: Option<fn(*mut (), size_t, *mut uint8_t)>,
}

#[derive(Clone, Copy)]
pub struct Sm3Ctx {
    pub state: [uint32_t; 8],
    pub count: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 64],
}

#[derive(Clone, Copy)]
pub struct HmacSm3Ctx {
    pub outer: Sm3Ctx,
    pub inner: Sm3Ctx,
    pub state: Sm3Ctx,
}

static NETTLE_SM3: NettleHash = NettleHash {
    name: "sm3",
    context_size: std::mem::size_of::<Sm3Ctx>() as u32,
    digest_size: 32,
    block_size: 64,
    init: None,
    update: None,
    digest: None,
};

pub fn nettle_hmac_sm3_set_key(ctx: &mut HmacSm3Ctx, key: &[u8]) {
    unsafe {
        let outer_ptr = &mut ctx.outer as *mut Sm3Ctx as *mut libc::c_void;
        let inner_ptr = &mut ctx.inner as *mut Sm3Ctx as *mut libc::c_void;
        let state_ptr = &mut ctx.state as *mut Sm3Ctx as *mut libc::c_void;
        
        nettle_hmac_set_key(
            outer_ptr,
            inner_ptr,
            state_ptr,
            &NETTLE_SM3 as *const _ as *const libc::c_void,
            key.len(),
            key.as_ptr(),
        );
    }
}

pub fn nettle_hmac_sm3_update(ctx: &mut HmacSm3Ctx, data: &[u8]) {
    unsafe {
        nettle_sm3_update(&mut ctx.state, data.len(), data.as_ptr());
    }
}

pub fn nettle_hmac_sm3_digest(ctx: &mut HmacSm3Ctx, digest: &mut [u8]) {
    unsafe {
        let outer_ptr = &ctx.outer as *const Sm3Ctx as *const libc::c_void;
        let inner_ptr = &ctx.inner as *const Sm3Ctx as *const libc::c_void;
        let state_ptr = &mut ctx.state as *mut Sm3Ctx as *mut libc::c_void;
        
        nettle_hmac_digest(
            outer_ptr,
            inner_ptr,
            state_ptr,
            &NETTLE_SM3 as *const _ as *const libc::c_void,
            digest.len(),
            digest.as_mut_ptr(),
        );
    }
}

extern "C" {
    fn nettle_sm3_update(ctx: *mut Sm3Ctx, length: size_t, data: *const uint8_t);
    fn nettle_hmac_set_key(
        outer: *mut libc::c_void,
        inner: *mut libc::c_void,
        state: *mut libc::c_void,
        hash: *const libc::c_void,
        length: size_t,
        key: *const uint8_t,
    );
    fn nettle_hmac_digest(
        outer: *const libc::c_void,
        inner: *const libc::c_void,
        state: *mut libc::c_void,
        hash: *const libc::c_void,
        length: size_t,
        digest: *mut uint8_t,
    );
}