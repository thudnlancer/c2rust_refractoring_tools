use std::mem::size_of;
use std::os::raw::{c_char, c_uint, c_ulong, c_uchar, c_void};

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

pub type NettleSetKeyFunc = fn(*mut c_void, *const uint8_t);
pub type NettleCryptFunc = fn(*mut c_void, size_t, *mut uint8_t, *const uint8_t);
pub type NettleHashUpdateFunc = fn(*mut c_void, size_t, *const uint8_t);
pub type NettleHashDigestFunc = fn(*mut c_void, size_t, *mut uint8_t);

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
pub struct GcmKey {
    pub h: [NettleBlock16; 256],
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
pub struct Aes256Ctx {
    pub keys: [uint32_t; 60],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct GcmAes256Ctx {
    pub key: GcmKey,
    pub gcm: GcmCtx,
    pub cipher: Aes256Ctx,
}

fn gcm_aes256_set_nonce_wrapper(ctx: *mut c_void, nonce: *const uint8_t) {
    // Safe because we trust the underlying C implementation
    unsafe {
        nettle_gcm_aes256_set_iv(
            ctx as *mut GcmAes256Ctx,
            (16 - 4) as size_t,
            nonce,
        );
    }
}

pub static NETTLE_GCM_AES256: NettleAead = NettleAead {
    name: b"gcm_aes256\0" as *const u8 as *const c_char,
    context_size: size_of::<GcmAes256Ctx>() as c_uint,
    block_size: 16 as c_uint,
    key_size: 32 as c_uint,
    nonce_size: (16 - 4) as c_uint,
    digest_size: 16 as c_uint,
    set_encrypt_key: Some(nettle_gcm_aes256_set_key as NettleSetKeyFunc),
    set_decrypt_key: Some(nettle_gcm_aes256_set_key as NettleSetKeyFunc),
    set_nonce: Some(gcm_aes256_set_nonce_wrapper as NettleSetKeyFunc),
    update: Some(nettle_gcm_aes256_update as NettleHashUpdateFunc),
    encrypt: Some(nettle_gcm_aes256_encrypt as NettleCryptFunc),
    decrypt: Some(nettle_gcm_aes256_decrypt as NettleCryptFunc),
    digest: Some(nettle_gcm_aes256_digest as NettleHashDigestFunc),
};

extern "C" {
    fn nettle_gcm_aes256_set_key(ctx: *mut GcmAes256Ctx, key: *const uint8_t);
    fn nettle_gcm_aes256_update(ctx: *mut GcmAes256Ctx, length: size_t, data: *const uint8_t);
    fn nettle_gcm_aes256_set_iv(ctx: *mut GcmAes256Ctx, length: size_t, iv: *const uint8_t);
    fn nettle_gcm_aes256_encrypt(
        ctx: *mut GcmAes256Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_aes256_decrypt(
        ctx: *mut GcmAes256Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_aes256_digest(
        ctx: *mut GcmAes256Ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
}