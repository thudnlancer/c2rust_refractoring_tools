use std::mem;

const SHA512_DIGEST_SIZE: usize = 64;

#[derive(Clone, Copy)]
struct Sha512Ctx {
    state: [u64; 8],
    count_low: u64,
    count_high: u64,
    index: u32,
    block: [u8; 128],
}

#[derive(Clone, Copy)]
struct HmacSha512Ctx {
    outer: Sha512Ctx,
    inner: Sha512Ctx,
    state: Sha512Ctx,
}

impl HmacSha512Ctx {
    fn new() -> Self {
        HmacSha512Ctx {
            outer: Sha512Ctx {
                state: [0; 8],
                count_low: 0,
                count_high: 0,
                index: 0,
                block: [0; 128],
            },
            inner: Sha512Ctx {
                state: [0; 8],
                count_low: 0,
                count_high: 0,
                index: 0,
                block: [0; 128],
            },
            state: Sha512Ctx {
                state: [0; 8],
                count_low: 0,
                count_high: 0,
                index: 0,
                block: [0; 128],
            },
        }
    }

    fn set_key(&mut self, key: &[u8]) {
        unsafe {
            nettle_hmac_sha512_set_key(
                self as *mut _,
                key.len(),
                key.as_ptr(),
            );
        }
    }

    fn update(&mut self, data: &[u8]) {
        unsafe {
            nettle_hmac_sha512_update(
                self as *mut _,
                data.len(),
                data.as_ptr(),
            );
        }
    }

    fn digest(&mut self, output: &mut [u8]) {
        unsafe {
            nettle_hmac_sha512_digest(
                self as *mut _,
                output.len(),
                output.as_mut_ptr(),
            );
        }
    }
}

pub fn pbkdf2_hmac_sha512(
    key: &[u8],
    iterations: u32,
    salt: &[u8],
    output: &mut [u8],
) {
    let mut ctx = HmacSha512Ctx::new();
    ctx.set_key(key);

    unsafe {
        nettle_pbkdf2(
            &mut ctx as *mut _ as *mut libc::c_void,
            Some(mem::transmute(nettle_hmac_sha512_update as unsafe extern "C" fn(*mut HmacSha512Ctx, usize, *const u8))),
            Some(mem::transmute(nettle_hmac_sha512_digest as unsafe extern "C" fn(*mut HmacSha512Ctx, usize, *mut u8))),
            SHA512_DIGEST_SIZE,
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
        mac_ctx: *mut libc::c_void,
        update: Option<unsafe extern "C" fn(*mut libc::c_void, usize, *const u8)>,
        digest: Option<unsafe extern "C" fn(*mut libc::c_void, usize, *mut u8)>,
        digest_size: usize,
        iterations: u32,
        salt_length: usize,
        salt: *const u8,
        length: usize,
        dst: *mut u8,
    );
    fn nettle_hmac_sha512_update(
        ctx: *mut HmacSha512Ctx,
        length: usize,
        data: *const u8,
    );
    fn nettle_hmac_sha512_set_key(
        ctx: *mut HmacSha512Ctx,
        key_length: usize,
        key: *const u8,
    );
    fn nettle_hmac_sha512_digest(
        ctx: *mut HmacSha512Ctx,
        length: usize,
        digest: *mut u8,
    );
}