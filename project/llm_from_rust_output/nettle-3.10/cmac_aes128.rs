use libc::{c_uchar, c_uint, c_ulong, c_void};
use std::mem;

type size_t = c_ulong;
type uint8_t = c_uchar;
type uint32_t = c_uint;
type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}

pub type nettle_cipher_func = unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes128_ctx {
    pub keys: [uint32_t; 44],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac128_key {
    pub K1: nettle_block16,
    pub K2: nettle_block16,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac128_ctx {
    pub X: nettle_block16,
    pub block: nettle_block16,
    pub index: size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac_aes128_ctx {
    pub key: cmac128_key,
    pub ctx: cmac128_ctx,
    pub cipher: aes128_ctx,
}

pub struct CmacAes128 {
    inner: cmac_aes128_ctx,
}

impl CmacAes128 {
    pub fn new(key: &[u8; 16]) -> Self {
        unsafe {
            let mut ctx = mem::zeroed();
            nettle_aes128_set_encrypt_key(&mut ctx.cipher, key.as_ptr());
            nettle_cmac128_set_key(
                &mut ctx.key,
                &mut ctx.cipher as *mut _ as *const c_void,
                Some(mem::transmute(nettle_aes128_encrypt as unsafe extern "C" fn(*const aes128_ctx, size_t, *mut uint8_t, *const uint8_t))),
            );
            nettle_cmac128_init(&mut ctx.ctx);
            Self { inner: ctx }
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            nettle_cmac128_update(
                &mut self.inner.ctx,
                &mut self.inner.cipher as *mut _ as *const c_void,
                Some(mem::transmute(nettle_aes128_encrypt as unsafe extern "C" fn(*const aes128_ctx, size_t, *mut uint8_t, *const uint8_t))),
                data.len(),
                data.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        unsafe {
            nettle_cmac128_digest(
                &mut self.inner.ctx,
                &mut self.inner.key,
                &mut self.inner.cipher as *mut _ as *const c_void,
                Some(mem::transmute(nettle_aes128_encrypt as unsafe extern "C" fn(*const aes128_ctx, size_t, *mut uint8_t, *const uint8_t))),
                digest.len() as c_uint,
                digest.as_mut_ptr(),
            );
        }
    }
}

extern "C" {
    fn nettle_aes128_set_encrypt_key(ctx: *mut aes128_ctx, key: *const uint8_t);
    fn nettle_aes128_encrypt(ctx: *const aes128_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn nettle_cmac128_set_key(key: *mut cmac128_key, cipher: *const c_void, encrypt: Option<nettle_cipher_func>);
    fn nettle_cmac128_init(ctx: *mut cmac128_ctx);
    fn nettle_cmac128_update(ctx: *mut cmac128_ctx, cipher: *const c_void, encrypt: Option<nettle_cipher_func>, msg_len: size_t, msg: *const uint8_t);
    fn nettle_cmac128_digest(ctx: *mut cmac128_ctx, key: *const cmac128_key, cipher: *const c_void, encrypt: Option<nettle_cipher_func>, length: c_uint, digest: *mut uint8_t);
}