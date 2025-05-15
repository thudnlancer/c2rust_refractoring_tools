use std::mem;

type size_t = usize;
type uint8_t = u8;
type uint64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [u64; 2],
    pub u64_0: [uint64_t; 2],
}

pub type nettle_cipher_func = fn(cipher: &dyn Cipher, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia256_ctx {
    pub keys: [uint64_t; 32],
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
pub struct gcm_camellia256_ctx {
    pub key: gcm_key,
    pub gcm: gcm_ctx,
    pub cipher: camellia256_ctx,
}

pub trait Cipher {
    fn encrypt(&self, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]);
    fn decrypt(&self, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]);
}

impl Cipher for camellia256_ctx {
    fn encrypt(&self, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]) {
        unsafe {
            nettle_camellia256_crypt(self, length, dst.as_mut_ptr(), src.as_ptr());
        }
    }
    
    fn decrypt(&self, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]) {
        self.encrypt(length, dst, src);
    }
}

pub fn nettle_gcm_camellia256_set_key(ctx: &mut gcm_camellia256_ctx, key: &[uint8_t]) {
    unsafe {
        nettle_camellia256_set_encrypt_key(&mut ctx.cipher, key.as_ptr());
        nettle_gcm_set_key(
            &mut ctx.key,
            &ctx.cipher as *const _ as *const libc::c_void,
            Some(mem::transmute(nettle_camellia256_crypt as fn(_, _, _, _))),
        );
    }
}

pub fn nettle_gcm_camellia256_set_iv(ctx: &mut gcm_camellia256_ctx, length: size_t, iv: &[uint8_t]) {
    unsafe {
        nettle_gcm_set_iv(&mut ctx.gcm, &ctx.key, length, iv.as_ptr());
    }
}

pub fn nettle_gcm_camellia256_update(ctx: &mut gcm_camellia256_ctx, length: size_t, data: &[uint8_t]) {
    unsafe {
        nettle_gcm_update(&mut ctx.gcm, &ctx.key, length, data.as_ptr());
    }
}

pub fn nettle_gcm_camellia256_encrypt(
    ctx: &mut gcm_camellia256_ctx,
    length: size_t,
    dst: &mut [uint8_t],
    src: &[uint8_t],
) {
    unsafe {
        nettle_gcm_encrypt(
            &mut ctx.gcm,
            &ctx.key,
            &ctx.cipher as *const _ as *const libc::c_void,
            Some(mem::transmute(nettle_camellia256_crypt as fn(_, _, _, _))),
            length,
            dst.as_mut_ptr(),
            src.as_ptr(),
        );
    }
}

pub fn nettle_gcm_camellia256_decrypt(
    ctx: &mut gcm_camellia256_ctx,
    length: size_t,
    dst: &mut [uint8_t],
    src: &[uint8_t],
) {
    unsafe {
        nettle_gcm_decrypt(
            &mut ctx.gcm,
            &ctx.key,
            &ctx.cipher as *const _ as *const libc::c_void,
            Some(mem::transmute(nettle_camellia256_crypt as fn(_, _, _, _))),
            length,
            dst.as_mut_ptr(),
            src.as_ptr(),
        );
    }
}

pub fn nettle_gcm_camellia256_digest(
    ctx: &mut gcm_camellia256_ctx,
    length: size_t,
    digest: &mut [uint8_t],
) {
    unsafe {
        nettle_gcm_digest(
            &mut ctx.gcm,
            &ctx.key,
            &ctx.cipher as *const _ as *const libc::c_void,
            Some(mem::transmute(nettle_camellia256_crypt as fn(_, _, _, _))),
            length,
            digest.as_mut_ptr(),
        );
    }
}

extern "C" {
    fn nettle_camellia256_set_encrypt_key(ctx: *mut camellia256_ctx, key: *const uint8_t);
    fn nettle_camellia256_crypt(
        ctx: *const camellia256_ctx,
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