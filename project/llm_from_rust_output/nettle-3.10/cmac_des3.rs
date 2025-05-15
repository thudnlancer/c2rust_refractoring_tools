use libc::{c_int, c_uchar, c_uint, c_ulong, c_void};
use std::mem;

type size_t = c_ulong;
type uint8_t = c_uchar;
type uint32_t = c_uint;
type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block8 {
    pub b: [uint8_t; 8],
    pub u64_0: uint64_t,
}

type nettle_cipher_func = unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct des_ctx {
    pub key: [uint32_t; 32],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct des3_ctx {
    pub des: [des_ctx; 3],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac64_key {
    pub K1: nettle_block8,
    pub K2: nettle_block8,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac64_ctx {
    pub X: nettle_block8,
    pub block: nettle_block8,
    pub index: size_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac_des3_ctx {
    pub key: cmac64_key,
    pub ctx: cmac64_ctx,
    pub cipher: des3_ctx,
}

extern "C" {
    fn nettle_des3_set_key(ctx: *mut des3_ctx, key: *const uint8_t) -> c_int;
    fn nettle_des3_encrypt(ctx: *const des3_ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn nettle_cmac64_set_key(key: *mut cmac64_key, cipher: *const c_void, encrypt: Option<nettle_cipher_func>);
    fn nettle_cmac64_init(ctx: *mut cmac64_ctx);
    fn nettle_cmac64_update(
        ctx: *mut cmac64_ctx,
        cipher: *const c_void,
        encrypt: Option<nettle_cipher_func>,
        msg_len: size_t,
        msg: *const uint8_t,
    );
    fn nettle_cmac64_digest(
        ctx: *mut cmac64_ctx,
        key: *const cmac64_key,
        cipher: *const c_void,
        encrypt: Option<nettle_cipher_func>,
        length: c_uint,
        digest: *mut uint8_t,
    );
}

pub fn nettle_cmac_des3_set_key(ctx: &mut cmac_des3_ctx, key: &[uint8_t]) {
    unsafe {
        nettle_des3_set_key(&mut ctx.cipher, key.as_ptr());
        nettle_cmac64_set_key(
            &mut ctx.key,
            &mut ctx.cipher as *mut des3_ctx as *const c_void,
            Some(mem::transmute(nettle_des3_encrypt as unsafe extern "C" fn(*const des3_ctx, size_t, *mut uint8_t, *const uint8_t))),
        );
        nettle_cmac64_init(&mut ctx.ctx);
    }
}

pub fn nettle_cmac_des3_update(ctx: &mut cmac_des3_ctx, data: &[uint8_t]) {
    unsafe {
        nettle_cmac64_update(
            &mut ctx.ctx,
            &mut ctx.cipher as *mut des3_ctx as *const c_void,
            Some(mem::transmute(nettle_des3_encrypt as unsafe extern "C" fn(*const des3_ctx, size_t, *mut uint8_t, *const uint8_t))),
            data.len() as size_t,
            data.as_ptr(),
        );
    }
}

pub fn nettle_cmac_des3_digest(ctx: &mut cmac_des3_ctx, digest: &mut [uint8_t]) {
    unsafe {
        nettle_cmac64_digest(
            &mut ctx.ctx,
            &ctx.key,
            &mut ctx.cipher as *mut des3_ctx as *const c_void,
            Some(mem::transmute(nettle_des3_encrypt as unsafe extern "C" fn(*const des3_ctx, size_t, *mut uint8_t, *const uint8_t))),
            digest.len() as c_uint,
            digest.as_mut_ptr(),
        );
    }
}