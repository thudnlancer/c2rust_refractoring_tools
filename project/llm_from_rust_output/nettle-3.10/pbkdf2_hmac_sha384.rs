use std::mem::MaybeUninit;

const SHA384_DIGEST_SIZE: usize = 48;

#[derive(Clone, Copy)]
pub struct Sha512Ctx {
    state: [u64; 8],
    count_low: u64,
    count_high: u64,
    index: u32,
    block: [u8; 128],
}

#[derive(Clone, Copy)]
pub struct HmacSha512Ctx {
    outer: Sha512Ctx,
    inner: Sha512Ctx,
    state: Sha512Ctx,
}

pub fn pbkdf2_hmac_sha384(
    key: &[u8],
    iterations: u32,
    salt: &[u8],
    output: &mut [u8],
) {
    let mut ctx = MaybeUninit::<HmacSha512Ctx>::uninit();
    
    // Initialize HMAC context with key
    unsafe {
        nettle_hmac_sha384_set_key(ctx.as_mut_ptr(), key.len(), key.as_ptr());
    }

    // Perform PBKDF2
    unsafe {
        nettle_pbkdf2(
            ctx.as_mut_ptr() as *mut libc::c_void,
            Some(nettle_hmac_sha512_update),
            Some(nettle_hmac_sha384_digest),
            SHA384_DIGEST_SIZE,
            iterations,
            salt.len(),
            salt.as_ptr(),
            output.len(),
            output.as_mut_ptr(),
        );
    }
}

// Safe wrappers around the unsafe FFI functions
mod ffi {
    use super::*;
    
    #[link(name = "nettle")]
    extern "C" {
        pub(super) fn nettle_pbkdf2(
            mac_ctx: *mut libc::c_void,
            update: Option<nettle_hash_update_func>,
            digest: Option<nettle_hash_digest_func>,
            digest_size: usize,
            iterations: u32,
            salt_length: usize,
            salt: *const u8,
            length: usize,
            dst: *mut u8,
        );
        
        pub(super) fn nettle_hmac_sha512_update(
            ctx: *mut HmacSha512Ctx,
            length: usize,
            data: *const u8,
        );
        
        pub(super) fn nettle_hmac_sha384_set_key(
            ctx: *mut HmacSha512Ctx,
            key_length: usize,
            key: *const u8,
        );
        
        pub(super) fn nettle_hmac_sha384_digest(
            ctx: *mut HmacSha512Ctx,
            length: usize,
            digest: *mut u8,
        );
    }
    
    pub type nettle_hash_update_func = unsafe extern "C" fn(
        *mut libc::c_void,
        usize,
        *const u8,
    );
    
    pub type nettle_hash_digest_func = unsafe extern "C" fn(
        *mut libc::c_void,
        usize,
        *mut u8,
    );
}

use ffi::*;