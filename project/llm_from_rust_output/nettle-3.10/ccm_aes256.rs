use std::mem;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [u64; 2],
    pub u64_0: [uint64_t; 2],
}

pub type nettle_cipher_func = fn(ctx: &dyn Cipher, length: size_t, dst: &mut [u8], src: &[u8]);

pub trait Cipher {
    fn encrypt(&self, length: size_t, dst: &mut [u8], src: &[u8]);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}

impl Cipher for aes256_ctx {
    fn encrypt(&self, length: size_t, dst: &mut [u8], src: &[u8]) {
        assert!(dst.len() >= length as usize);
        assert!(src.len() >= length as usize);
        unsafe {
            nettle_aes256_encrypt(
                self as *const _,
                length,
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ccm_ctx {
    pub ctr: nettle_block16,
    pub tag: nettle_block16,
    pub blength: u32,
}

#[derive(Copy, Clone)]
pub struct ccm_aes256_ctx {
    pub ccm: ccm_ctx,
    pub cipher: aes256_ctx,
}

impl ccm_aes256_ctx {
    pub fn set_key(&mut self, key: &[u8; 32]) {
        unsafe {
            nettle_aes256_set_encrypt_key(&mut self.cipher as *mut _, key.as_ptr());
        }
    }

    pub fn set_nonce(
        &mut self,
        nonce: &[u8],
        authlen: size_t,
        msglen: size_t,
        taglen: size_t,
    ) {
        unsafe {
            nettle_ccm_set_nonce(
                &mut self.ccm as *mut _,
                &self.cipher as *const _ as *const _,
                Some(mem::transmute(
                    nettle_aes256_encrypt as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ),
                ),
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
                &mut self.ccm as *mut _,
                &self.cipher as *const _ as *const _,
                Some(mem::transmute(
                    nettle_aes256_encrypt as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ),
                ),
                data.len(),
                data.as_ptr(),
            );
        }
    }

    pub fn encrypt(&mut self, dst: &mut [u8], src: &[u8]) {
        assert_eq!(dst.len(), src.len());
        unsafe {
            nettle_ccm_encrypt(
                &mut self.ccm as *mut _,
                &self.cipher as *const _ as *const _,
                Some(mem::transmute(
                    nettle_aes256_encrypt as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ),
                ),
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
                &mut self.ccm as *mut _,
                &self.cipher as *const _ as *const _,
                Some(mem::transmute(
                    nettle_aes256_encrypt as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ),
                ),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        unsafe {
            nettle_ccm_digest(
                &mut self.ccm as *mut _,
                &self.cipher as *const _ as *const _,
                Some(mem::transmute(
                    nettle_aes256_encrypt as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ),
                ),
                digest.len(),
                digest.as_mut_ptr(),
            );
        }
    }

    pub fn encrypt_message(
        &mut self,
        nonce: &[u8],
        adata: &[u8],
        taglen: size_t,
        dst: &mut [u8],
        src: &[u8],
    ) {
        unsafe {
            nettle_ccm_encrypt_message(
                &self.cipher as *const _ as *const _,
                Some(mem::transmute(
                    nettle_aes256_encrypt as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ),
                ),
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
        taglen: size_t,
        dst: &mut [u8],
        src: &[u8],
    ) -> bool {
        unsafe {
            nettle_ccm_decrypt_message(
                &self.cipher as *const _ as *const _,
                Some(mem::transmute(
                    nettle_aes256_encrypt as unsafe extern "C" fn(
                        *const aes256_ctx,
                        size_t,
                        *mut uint8_t,
                        *const uint8_t,
                    ),
                ),
                nonce.len(),
                nonce.as_ptr(),
                adata.len(),
                adata.as_ptr(),
                taglen,
                dst.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            ) != 0
        }
    }
}

extern "C" {
    fn nettle_aes256_set_encrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ccm_set_nonce(
        ctx: *mut ccm_ctx,
        cipher: *const std::ffi::c_void,
        f: Option<nettle_cipher_func>,
        noncelen: size_t,
        nonce: *const uint8_t,
        authlen: size_t,
        msglen: size_t,
        taglen: size_t,
    );
    fn nettle_ccm_update(
        ctx: *mut ccm_ctx,
        cipher: *const std::ffi::c_void,
        f: Option<nettle_cipher_func>,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_ccm_encrypt(
        ctx: *mut ccm_ctx,
        cipher: *const std::ffi::c_void,
        f: Option<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ccm_decrypt(
        ctx: *mut ccm_ctx,
        cipher: *const std::ffi::c_void,
        f: Option<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ccm_digest(
        ctx: *mut ccm_ctx,
        cipher: *const std::ffi::c_void,
        f: Option<nettle_cipher_func>,
        length: size_t,
        digest: *mut uint8_t,
    );
    fn nettle_ccm_encrypt_message(
        cipher: *const std::ffi::c_void,
        f: Option<nettle_cipher_func>,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        tlength: size_t,
        clength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_ccm_decrypt_message(
        cipher: *const std::ffi::c_void,
        f: Option<nettle_cipher_func>,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        tlength: size_t,
        mlength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    ) -> std::os::raw::c_int;
}