use nettle_sys::{
    aes256_ctx, cmac128_ctx, cmac128_key, cmac_aes256_ctx, nettle_block16,
    nettle_aes256_set_encrypt_key, nettle_aes256_encrypt, nettle_cmac128_set_key,
    nettle_cmac128_init, nettle_cmac128_update, nettle_cmac128_digest,
};
use std::convert::TryInto;

pub struct CmacAes256 {
    ctx: cmac_aes256_ctx,
}

impl CmacAes256 {
    pub fn new(key: &[u8; 32]) -> Self {
        let mut ctx = unsafe { std::mem::zeroed() };
        unsafe {
            nettle_aes256_set_encrypt_key(&mut ctx.cipher, key.as_ptr());
            nettle_cmac128_set_key(
                &mut ctx.key,
                &mut ctx.cipher as *mut _ as *const _,
                Some(nettle_aes256_encrypt as _),
            );
            nettle_cmac128_init(&mut ctx.ctx);
        }
        Self { ctx }
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            nettle_cmac128_update(
                &mut self.ctx.ctx,
                &mut self.ctx.cipher as *mut _ as *const _,
                Some(nettle_aes256_encrypt as _),
                data.len(),
                data.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        let length = digest.len().try_into().expect("digest length too large");
        unsafe {
            nettle_cmac128_digest(
                &mut self.ctx.ctx,
                &self.ctx.key,
                &mut self.ctx.cipher as *mut _ as *const _,
                Some(nettle_aes256_encrypt as _),
                length,
                digest.as_mut_ptr(),
            );
        }
    }
}