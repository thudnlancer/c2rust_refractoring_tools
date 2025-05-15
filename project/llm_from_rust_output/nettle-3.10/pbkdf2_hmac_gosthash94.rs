use std::mem::MaybeUninit;

const GOSTHASH94_DIGEST_SIZE: usize = 32;

#[derive(Clone, Copy)]
pub struct GostHash94Ctx {
    hash: [u32; 8],
    sum: [u32; 8],
    count: u64,
    index: u32,
    block: [u8; 32],
}

#[derive(Clone, Copy)]
pub struct HmacGostHash94CpCtx {
    outer: GostHash94Ctx,
    inner: GostHash94Ctx,
    state: GostHash94Ctx,
}

impl HmacGostHash94CpCtx {
    pub fn new() -> Self {
        Self {
            outer: GostHash94Ctx {
                hash: [0; 8],
                sum: [0; 8],
                count: 0,
                index: 0,
                block: [0; 32],
            },
            inner: GostHash94Ctx {
                hash: [0; 8],
                sum: [0; 8],
                count: 0,
                index: 0,
                block: [0; 32],
            },
            state: GostHash94Ctx {
                hash: [0; 8],
                sum: [0; 8],
                count: 0,
                index: 0,
                block: [0; 32],
            },
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        // Implementation of key setting
        unsafe {
            nettle_hmac_gosthash94cp_set_key(
                self as *mut _,
                key.len(),
                key.as_ptr(),
            );
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            nettle_hmac_gosthash94cp_update(
                self as *mut _,
                data.len(),
                data.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, output: &mut [u8]) {
        unsafe {
            nettle_hmac_gosthash94cp_digest(
                self as *mut _,
                output.len(),
                output.as_mut_ptr(),
            );
        }
    }
}

pub fn pbkdf2_hmac_gosthash94cp(
    key: &[u8],
    iterations: u32,
    salt: &[u8],
    output: &mut [u8],
) {
    let mut ctx = HmacGostHash94CpCtx::new();
    ctx.set_key(key);

    let update_fn = |ctx: *mut std::ffi::c_void, len: usize, data: *const u8| {
        unsafe {
            nettle_hmac_gosthash94cp_update(
                ctx as *mut HmacGostHash94CpCtx,
                len,
                data,
            );
        }
    };

    let digest_fn = |ctx: *mut std::ffi::c_void, len: usize, data: *mut u8| {
        unsafe {
            nettle_hmac_gosthash94cp_digest(
                ctx as *mut HmacGostHash94CpCtx,
                len,
                data,
            );
        }
    };

    unsafe {
        nettle_pbkdf2(
            &mut ctx as *mut _ as *mut std::ffi::c_void,
            Some(update_fn),
            Some(digest_fn),
            GOSTHASH94_DIGEST_SIZE,
            iterations,
            salt.len(),
            salt.as_ptr(),
            output.len(),
            output.as_mut_ptr(),
        );
    }
}

extern "C" {
    fn nettle_pbkdf2(
        mac_ctx: *mut std::ffi::c_void,
        update: Option<extern "C" fn(*mut std::ffi::c_void, usize, *const u8)>,
        digest: Option<extern "C" fn(*mut std::ffi::c_void, usize, *mut u8)>,
        digest_size: usize,
        iterations: u32,
        salt_length: usize,
        salt: *const u8,
        length: usize,
        dst: *mut u8,
    );
    fn nettle_hmac_gosthash94cp_update(
        ctx: *mut HmacGostHash94CpCtx,
        length: usize,
        data: *const u8,
    );
    fn nettle_hmac_gosthash94cp_set_key(
        ctx: *mut HmacGostHash94CpCtx,
        key_length: usize,
        key: *const u8,
    );
    fn nettle_hmac_gosthash94cp_digest(
        ctx: *mut HmacGostHash94CpCtx,
        length: usize,
        digest: *mut u8,
    );
}