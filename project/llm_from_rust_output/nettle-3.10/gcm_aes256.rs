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

pub type nettle_cipher_func = fn(cipher: &dyn Aes256Cipher, length: size_t, dst: &mut [u8], src: &[u8]);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_key {
    pub h: [nettle_block16; 256],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_ctx {
    pub iv: nettle_block16,
    pub ctr: nettle_block16,
    pub x: nettle_block16,
    pub auth_size: uint64_t,
    pub data_size: uint64_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct gcm_aes256_ctx {
    pub key: gcm_key,
    pub gcm: gcm_ctx,
    pub cipher: aes256_ctx,
}

pub trait Aes256Cipher {
    fn encrypt(&self, length: size_t, dst: &mut [u8], src: &[u8]);
}

impl Aes256Cipher for aes256_ctx {
    fn encrypt(&self, length: size_t, dst: &mut [u8], src: &[u8]) {
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

pub struct GcmAes256 {
    ctx: gcm_aes256_ctx,
}

impl GcmAes256 {
    pub fn new(key: &[u8]) -> Self {
        let mut ctx = unsafe { mem::zeroed() };
        unsafe {
            nettle_aes256_set_encrypt_key(&mut ctx.cipher, key.as_ptr());
            nettle_gcm_set_key(
                &mut ctx.key,
                &ctx.cipher as *const _ as *const _,
                Some(nettle_aes256_encrypt_wrapper),
            );
        }
        Self { ctx }
    }

    pub fn set_iv(&mut self, iv: &[u8]) {
        unsafe {
            nettle_gcm_set_iv(&mut self.ctx.gcm, &self.ctx.key, iv.len(), iv.as_ptr());
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            nettle_gcm_update(&mut self.ctx.gcm, &self.ctx.key, data.len(), data.as_ptr());
        }
    }

    pub fn encrypt(&mut self, dst: &mut [u8], src: &[u8]) {
        assert_eq!(dst.len(), src.len());
        unsafe {
            nettle_gcm_encrypt(
                &mut self.ctx.gcm,
                &self.ctx.key,
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes256_encrypt_wrapper),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt(&mut self, dst: &mut [u8], src: &[u8]) {
        assert_eq!(dst.len(), src.len());
        unsafe {
            nettle_gcm_decrypt(
                &mut self.ctx.gcm,
                &self.ctx.key,
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes256_encrypt_wrapper),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        unsafe {
            nettle_gcm_digest(
                &mut self.ctx.gcm,
                &self.ctx.key,
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes256_encrypt_wrapper),
                digest.len(),
                digest.as_mut_ptr(),
            );
        }
    }
}

unsafe extern "C" fn nettle_aes256_encrypt_wrapper(
    cipher: *const libc::c_void,
    length: size_t,
    dst: *mut uint8_t,
    src: *const uint8_t,
) {
    let cipher = &*(cipher as *const aes256_ctx);
    cipher.encrypt(length, std::slice::from_raw_parts_mut(dst, length), std::slice::from_raw_parts(src, length));
}

extern "C" {
    fn nettle_aes256_set_encrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_set_key(
        key: *mut gcm_key,
        cipher: *const libc::c_void,
        f: Option<nettle_cipher_func>,
    );
    fn nettle_gcm_set_iv(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        length: size_t,
        iv: *const uint8_t,
    );
    fn nettle_gcm_update(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_gcm_encrypt(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        cipher: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_decrypt(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        cipher: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_digest(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        cipher: *const libc::c_void,
        f: Option<nettle_cipher_func>,
        length: size_t,
        digest: *mut uint8_t,
    );
}