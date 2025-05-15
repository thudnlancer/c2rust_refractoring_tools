use std::mem;
use std::os::raw::{c_char, c_uint, c_ulong, c_uchar, c_void};

pub type size_t = c_ulong;
pub type uint8_t = c_uchar;
pub type uint32_t = c_uint;
pub type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}

pub type nettle_set_key_func = fn(*mut c_void, *const uint8_t);
pub type nettle_hash_update_func = fn(*mut c_void, size_t, *const uint8_t);
pub type nettle_hash_digest_func = fn(*mut c_void, size_t, *mut uint8_t);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_mac {
    pub name: *const c_char,
    pub context_size: c_uint,
    pub digest_size: c_uint,
    pub key_size: c_uint,
    pub set_key: Option<nettle_set_key_func>,
    pub update: Option<nettle_hash_update_func>,
    pub digest: Option<nettle_hash_digest_func>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac_aes256_ctx {
    pub key: cmac128_key,
    pub ctx: cmac128_ctx,
    pub cipher: aes256_ctx,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
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
pub struct cmac128_key {
    pub K1: nettle_block16,
    pub K2: nettle_block16,
}

pub fn nettle_cmac_aes256_set_key(ctx: *mut cmac_aes256_ctx, key: *const uint8_t) {
    // Implementation would wrap the unsafe call
    unsafe {
        nettle_cmac_aes256_set_key(ctx, key);
    }
}

pub fn nettle_cmac_aes256_update(ctx: *mut cmac_aes256_ctx, length: size_t, data: *const uint8_t) {
    // Implementation would wrap the unsafe call
    unsafe {
        nettle_cmac_aes256_update(ctx, length, data);
    }
}

pub fn nettle_cmac_aes256_digest(ctx: *mut cmac_aes256_ctx, length: size_t, digest: *mut uint8_t) {
    // Implementation would wrap the unsafe call
    unsafe {
        nettle_cmac_aes256_digest(ctx, length, digest);
    }
}

pub static NETTLE_CMAC_AES256: nettle_mac = nettle_mac {
    name: b"cmac_aes256\0".as_ptr() as *const c_char,
    context_size: mem::size_of::<cmac_aes256_ctx>() as c_uint,
    digest_size: 16,
    key_size: 32,
    set_key: Some(nettle_cmac_aes256_set_key as nettle_set_key_func),
    update: Some(nettle_cmac_aes256_update as nettle_hash_update_func),
    digest: Some(nettle_cmac_aes256_digest as nettle_hash_digest_func),
};