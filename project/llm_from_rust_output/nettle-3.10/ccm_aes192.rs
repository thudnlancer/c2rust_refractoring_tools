use nettle_sys::{
    aes192_ctx, ccm_aes192_ctx, ccm_ctx, nettle_block16, nettle_cipher_func,
    nettle_aes192_set_encrypt_key, nettle_aes192_encrypt,
    nettle_ccm_set_nonce, nettle_ccm_update, nettle_ccm_encrypt,
    nettle_ccm_decrypt, nettle_ccm_digest, nettle_ccm_encrypt_message,
    nettle_ccm_decrypt_message
};
use std::mem;

pub struct CcmAes192 {
    ctx: ccm_aes192_ctx,
}

impl CcmAes192 {
    pub fn new() -> Self {
        unsafe {
            CcmAes192 {
                ctx: mem::zeroed(),
            }
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        unsafe {
            nettle_aes192_set_encrypt_key(&mut self.ctx.cipher, key.as_ptr());
        }
    }

    pub fn set_nonce(&mut self, nonce: &[u8], authlen: usize, msglen: usize, taglen: usize) {
        unsafe {
            nettle_ccm_set_nonce(
                &mut self.ctx.ccm,
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes192_encrypt as nettle_cipher_func),
                nonce.len(),
                nonce.as_ptr(),
                authlen,
                msglen,
                taglen,
            );
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            nettle_ccm_update(
                &mut self.ctx.ccm,
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes192_encrypt as nettle_cipher_func),
                data.len(),
                data.as_ptr(),
            );
        }
    }

    pub fn encrypt(&mut self, dst: &mut [u8], src: &[u8]) {
        assert_eq!(dst.len(), src.len());
        unsafe {
            nettle_ccm_encrypt(
                &mut self.ctx.ccm,
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes192_encrypt as nettle_cipher_func),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt(&mut self, dst: &mut [u8], src: &[u8]) {
        assert_eq!(dst.len(), src.len());
        unsafe {
            nettle_ccm_decrypt(
                &mut self.ctx.ccm,
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes192_encrypt as nettle_cipher_func),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        unsafe {
            nettle_ccm_digest(
                &mut self.ctx.ccm,
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes192_encrypt as nettle_cipher_func),
                digest.len(),
                digest.as_mut_ptr(),
            );
        }
    }

    pub fn encrypt_message(
        &mut self,
        nonce: &[u8],
        adata: &[u8],
        taglen: usize,
        dst: &mut [u8],
        src: &[u8],
    ) {
        unsafe {
            nettle_ccm_encrypt_message(
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes192_encrypt as nettle_cipher_func),
                nonce.len(),
                nonce.as_ptr(),
                adata.len(),
                adata.as_ptr(),
                taglen,
                dst.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt_message(
        &mut self,
        nonce: &[u8],
        adata: &[u8],
        taglen: usize,
        dst: &mut [u8],
        src: &[u8],
    ) -> bool {
        let res = unsafe {
            nettle_ccm_decrypt_message(
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes192_encrypt as nettle_cipher_func),
                nonce.len(),
                nonce.as_ptr(),
                adata.len(),
                adata.as_ptr(),
                taglen,
                dst.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            )
        };
        res == 1
    }
}