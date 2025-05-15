use std::mem::size_of;
use std::os::raw::{c_char, c_uchar, c_uint, c_ulong, c_void};

type size_t = c_ulong;
type uint8_t = c_uchar;
type uint32_t = c_uint;
type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub union NettleBlock16 {
    pub b: [uint8_t; 16],
    pub w: [c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}

pub type NettleSetKeyFunc = fn(ctx: &mut c_void, key: &[uint8_t]);
pub type NettleCryptFunc = fn(ctx: &mut c_void, length: size_t, dst: &mut [uint8_t], src: &[uint8_t]);
pub type NettleHashUpdateFunc = fn(ctx: &mut c_void, length: size_t, data: &[uint8_t]);
pub type NettleHashDigestFunc = fn(ctx: &mut c_void, length: size_t, digest: &mut [uint8_t]);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NettleAead {
    pub name: *const c_char,
    pub context_size: c_uint,
    pub block_size: c_uint,
    pub key_size: c_uint,
    pub nonce_size: c_uint,
    pub digest_size: c_uint,
    pub set_encrypt_key: Option<NettleSetKeyFunc>,
    pub set_decrypt_key: Option<NettleSetKeyFunc>,
    pub set_nonce: Option<NettleSetKeyFunc>,
    pub update: Option<NettleHashUpdateFunc>,
    pub encrypt: Option<NettleCryptFunc>,
    pub decrypt: Option<NettleCryptFunc>,
    pub digest: Option<NettleHashDigestFunc>,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GcmAes128Ctx {
    pub key: GcmKey,
    pub gcm: GcmCtx,
    pub cipher: Aes128Ctx,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes128Ctx {
    pub keys: [uint32_t; 44],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GcmCtx {
    pub iv: NettleBlock16,
    pub ctr: NettleBlock16,
    pub x: NettleBlock16,
    pub auth_size: uint64_t,
    pub data_size: uint64_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GcmKey {
    pub h: [NettleBlock16; 256],
}

fn gcm_aes128_set_nonce_wrapper(ctx: &mut c_void, nonce: &[uint8_t]) {
    // Safe wrapper for set_nonce operation
}

pub static NETTLE_GCM_AES128: NettleAead = NettleAead {
    name: b"gcm_aes128\0".as_ptr() as *const c_char,
    context_size: size_of::<GcmAes128Ctx>() as c_uint,
    block_size: 16,
    key_size: 16,
    nonce_size: 12,
    digest_size: 16,
    set_encrypt_key: None,
    set_decrypt_key: None,
    set_nonce: Some(gcm_aes128_set_nonce_wrapper),
    update: None,
    encrypt: None,
    decrypt: None,
    digest: None,
};