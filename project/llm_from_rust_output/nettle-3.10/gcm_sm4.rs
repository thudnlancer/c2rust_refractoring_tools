use std::mem;

type size_t = usize;
type uint8_t = u8;
type uint32_t = u32;
type uint64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [u64; 2],
    pub u64_0: [uint64_t; 2],
}

pub type nettle_cipher_func = fn(context: &dyn Cipher, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]);

pub trait Cipher {
    fn encrypt(&self, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]);
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sm4_ctx {
    pub rkey: [uint32_t; 32],
}

impl Cipher for sm4_ctx {
    fn encrypt(&self, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]) {
        unsafe {
            nettle_sm4_crypt(self, length, dst.as_mut_ptr(), src.as_ptr());
        }
    }
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
pub struct gcm_sm4_ctx {
    pub key: gcm_key,
    pub gcm: gcm_ctx,
    pub cipher: sm4_ctx,
}

impl gcm_sm4_ctx {
    pub fn set_key(&mut self, key: &[uint8_t]) {
        unsafe {
            nettle_sm4_set_encrypt_key(&mut self.cipher, key.as_ptr());
            nettle_gcm_set_key(
                &mut self.key,
                &self.cipher as *const _ as *const libc::c_void,
                Some(mem::transmute(Self::sm4_crypt as fn(*const sm4_ctx, size_t, *mut uint8_t, *const uint8_t))),
            );
        }
    }

    pub fn set_iv(&mut self, iv: &[uint8_t]) {
        unsafe {
            nettle_gcm_set_iv(&mut self.gcm, &self.key, iv.len(), iv.as_ptr());
        }
    }

    pub fn update(&mut self, data: &[uint8_t]) {
        unsafe {
            nettle_gcm_update(&mut self.gcm, &self.key, data.len(), data.as_ptr());
        }
    }

    pub fn encrypt(&mut self, dst: &mut [uint8_t], src: &[uint8_t]) {
        unsafe {
            nettle_gcm_encrypt(
                &mut self.gcm,
                &self.key,
                &self.cipher as *const _ as *const libc::c_void,
                Some(mem::transmute(Self::sm4_crypt as fn(*const sm4_ctx, size_t, *mut uint8_t, *const uint8_t))),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt(&mut self, dst: &mut [uint8_t], src: &[uint8_t]) {
        unsafe {
            nettle_gcm_decrypt(
                &mut self.gcm,
                &self.key,
                &self.cipher as *const _ as *const libc::c_void,
                Some(mem::transmute(Self::sm4_crypt as fn(*const sm4_ctx, size_t, *mut uint8_t, *const uint8_t))),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, digest: &mut [uint8_t]) {
        unsafe {
            nettle_gcm_digest(
                &mut self.gcm,
                &self.key,
                &self.cipher as *const _ as *const libc::c_void,
                Some(mem::transmute(Self::sm4_crypt as fn(*const sm4_ctx, size_t, *mut uint8_t, *const uint8_t))),
                digest.len(),
                digest.as_mut_ptr(),
            );
        }
    }

    unsafe extern "C" fn sm4_crypt(
        context: *const sm4_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    ) {
        nettle_sm4_crypt(context, length, dst, src);
    }
}

extern "C" {
    fn nettle_sm4_set_encrypt_key(ctx: *mut sm4_ctx, key: *const uint8_t);
    fn nettle_sm4_crypt(
        context: *const sm4_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_set_key(
        key: *mut gcm_key,
        cipher: *const libc::c_void,
        f: nettle_cipher_func,
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
        f: nettle_cipher_func,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_decrypt(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        cipher: *const libc::c_void,
        f: nettle_cipher_func,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_digest(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        cipher: *const libc::c_void,
        f: nettle_cipher_func,
        length: size_t,
        digest: *mut uint8_t,
    );
}