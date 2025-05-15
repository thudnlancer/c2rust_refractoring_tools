use std::mem::MaybeUninit;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint64_t = u64;

#[derive(Clone, Copy)]
pub struct Sha512Ctx {
    pub state: [uint64_t; 8],
    pub count_low: uint64_t,
    pub count_high: uint64_t,
    pub index: u32,
    pub block: [uint8_t; 128],
}

#[derive(Clone, Copy)]
pub struct HmacSha512Ctx {
    pub outer: Sha512Ctx,
    pub inner: Sha512Ctx,
    pub state: Sha512Ctx,
}

pub struct NettleHash {
    pub name: &'static str,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: fn(&mut Sha512Ctx),
    pub update: fn(&mut Sha512Ctx, size_t, &[uint8_t]),
    pub digest: fn(&mut Sha512Ctx, size_t, &mut [uint8_t]),
}

static NETTLE_SHA384: NettleHash = {
    extern "C" {
        fn nettle_sha512_init(ctx: *mut Sha512Ctx);
        fn nettle_sha512_update(ctx: *mut Sha512Ctx, length: size_t, data: *const uint8_t);
        fn nettle_sha512_digest(ctx: *mut Sha512Ctx, length: size_t, digest: *mut uint8_t);
    }

    NettleHash {
        name: "sha384",
        context_size: std::mem::size_of::<Sha512Ctx>() as u32,
        digest_size: 48,
        block_size: 128,
        init: |ctx| unsafe { nettle_sha512_init(ctx) },
        update: |ctx, length, data| unsafe { nettle_sha512_update(ctx, length, data.as_ptr()) },
        digest: |ctx, length, digest| unsafe { nettle_sha512_digest(ctx, length, digest.as_mut_ptr()) },
    }
};

pub fn hmac_sha384_set_key(ctx: &mut HmacSha512Ctx, key: &[uint8_t]) {
    let mut outer = MaybeUninit::new(ctx.outer);
    let mut inner = MaybeUninit::new(ctx.inner);
    let mut state = MaybeUninit::new(ctx.state);

    unsafe {
        (NETTLE_SHA384.init)(outer.as_mut_ptr());
        (NETTLE_SHA384.init)(inner.as_mut_ptr());
        (NETTLE_SHA384.init)(state.as_mut_ptr());
    }

    ctx.outer = unsafe { outer.assume_init() };
    ctx.inner = unsafe { inner.assume_init() };
    ctx.state = unsafe { state.assume_init() };

    // Actual HMAC key setup would go here
}

pub fn hmac_sha384_digest(ctx: &mut HmacSha512Ctx, digest: &mut [uint8_t]) {
    let mut outer = MaybeUninit::new(ctx.outer);
    let mut inner = MaybeUninit::new(ctx.inner);
    let mut state = MaybeUninit::new(ctx.state);

    unsafe {
        (NETTLE_SHA384.digest)(
            outer.as_mut_ptr(),
            digest.len(),
            digest.as_mut_ptr(),
        );
        (NETTLE_SHA384.digest)(
            inner.as_mut_ptr(),
            digest.len(),
            digest.as_mut_ptr(),
        );
        (NETTLE_SHA384.digest)(
            state.as_mut_ptr(),
            digest.len(),
            digest.as_mut_ptr(),
        );
    }

    ctx.outer = unsafe { outer.assume_init() };
    ctx.inner = unsafe { inner.assume_init() };
    ctx.state = unsafe { state.assume_init() };
}