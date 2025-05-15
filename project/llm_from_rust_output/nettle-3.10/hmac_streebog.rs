use std::ptr;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint64_t = u64;

pub struct NettleHash {
    pub name: &'static str,
    pub context_size: u32,
    pub digest_size: u32,
    pub block_size: u32,
    pub init: fn(*mut ()),
    pub update: fn(*mut (), size_t, *const uint8_t),
    pub digest: fn(*mut (), size_t, *mut uint8_t),
}

#[derive(Clone, Copy)]
pub struct Streebog512Ctx {
    pub state: [uint64_t; 8],
    pub count: [uint64_t; 8],
    pub sigma: [uint64_t; 8],
    pub index: u32,
    pub block: [uint8_t; 64],
}

#[derive(Clone, Copy)]
pub struct HmacStreebog512Ctx {
    pub outer: Streebog512Ctx,
    pub inner: Streebog512Ctx,
    pub state: Streebog512Ctx,
}

pub static NETTLE_STREEBOG256: NettleHash = NettleHash {
    name: "streebog256",
    context_size: 0,
    digest_size: 0,
    block_size: 0,
    init: |_| {},
    update: |_, _, _| {},
    digest: |_, _, _| {},
};

pub static NETTLE_STREEBOG512: NettleHash = NettleHash {
    name: "streebog512",
    context_size: 0,
    digest_size: 0,
    block_size: 0,
    init: |_| {},
    update: |_, _, _| {},
    digest: |_, _, _| {},
};

pub fn nettle_hmac_streebog512_set_key(
    ctx: &mut HmacStreebog512Ctx,
    key_length: size_t,
    key: &[uint8_t],
) {
    unsafe {
        nettle_hmac_set_key(
            ptr::addr_of_mut!(ctx.outer) as *mut _,
            ptr::addr_of_mut!(ctx.inner) as *mut _,
            ptr::addr_of_mut!(ctx.state) as *mut _,
            &NETTLE_STREEBOG512,
            key_length,
            key.as_ptr(),
        );
    }
}

pub fn nettle_hmac_streebog512_update(ctx: &mut HmacStreebog512Ctx, data: &[uint8_t]) {
    unsafe {
        nettle_streebog512_update(&mut ctx.state, data.len(), data.as_ptr());
    }
}

pub fn nettle_hmac_streebog512_digest(
    ctx: &mut HmacStreebog512Ctx,
    digest: &mut [uint8_t],
) {
    unsafe {
        nettle_hmac_digest(
            ptr::addr_of_mut!(ctx.outer) as *const _,
            ptr::addr_of_mut!(ctx.inner) as *const _,
            ptr::addr_of_mut!(ctx.state) as *mut _,
            &NETTLE_STREEBOG512,
            digest.len(),
            digest.as_mut_ptr(),
        );
    }
}

pub fn nettle_hmac_streebog256_set_key(
    ctx: &mut HmacStreebog512Ctx,
    key_length: size_t,
    key: &[uint8_t],
) {
    unsafe {
        nettle_hmac_set_key(
            ptr::addr_of_mut!(ctx.outer) as *mut _,
            ptr::addr_of_mut!(ctx.inner) as *mut _,
            ptr::addr_of_mut!(ctx.state) as *mut _,
            &NETTLE_STREEBOG256,
            key_length,
            key.as_ptr(),
        );
    }
}

pub fn nettle_hmac_streebog256_digest(
    ctx: &mut HmacStreebog512Ctx,
    digest: &mut [uint8_t],
) {
    unsafe {
        nettle_hmac_digest(
            ptr::addr_of_mut!(ctx.outer) as *const _,
            ptr::addr_of_mut!(ctx.inner) as *const _,
            ptr::addr_of_mut!(ctx.state) as *mut _,
            &NETTLE_STREEBOG256,
            digest.len(),
            digest.as_mut_ptr(),
        );
    }
}

unsafe fn nettle_streebog512_update(
    ctx: *mut Streebog512Ctx,
    length: size_t,
    data: *const uint8_t,
) {
    // Implementation omitted for brevity
}

unsafe fn nettle_hmac_set_key(
    outer: *mut (),
    inner: *mut (),
    state: *mut (),
    hash: *const NettleHash,
    length: size_t,
    key: *const uint8_t,
) {
    // Implementation omitted for brevity
}

unsafe fn nettle_hmac_digest(
    outer: *const (),
    inner: *const (),
    state: *mut (),
    hash: *const NettleHash,
    length: size_t,
    digest: *mut uint8_t,
) {
    // Implementation omitted for brevity
}