use std::mem::size_of;
use std::os::raw::{c_char, c_uchar, c_ulong, c_void};

pub type size_t = c_ulong;
pub type uint8_t = c_uchar;
pub type uint64_t = c_ulong;

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
    pub context_size: u32,
    pub block_size: u32,
    pub key_size: u32,
    pub nonce_size: u32,
    pub digest_size: u32,
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
pub struct GcmCamellia128Ctx {
    pub key: GcmKey,
    pub gcm: GcmCtx,
    pub cipher: Camellia128Ctx,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Camellia128Ctx {
    pub keys: [uint64_t; 24],
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

fn gcm_camellia128_set_nonce_wrapper(ctx: &mut c_void, nonce: &[uint8_t]) {
    unsafe {
        nettle_gcm_camellia128_set_iv(
            ctx as *mut _ as *mut GcmCamellia128Ctx,
            (16 - 4) as size_t,
            nonce.as_ptr(),
        );
    }
}

pub static NETTLE_GCM_CAMELLIA128: NettleAead = NettleAead {
    name: b"gcm_camellia128\0" as *const u8 as *const c_char,
    context_size: size_of::<GcmCamellia128Ctx>() as u32,
    block_size: 16,
    key_size: 16,
    nonce_size: (16 - 4) as u32,
    digest_size: 16,
    set_encrypt_key: Some(nettle_gcm_camellia128_set_key as NettleSetKeyFunc),
    set_decrypt_key: Some(nettle_gcm_camellia128_set_key as NettleSetKeyFunc),
    set_nonce: Some(gcm_camellia128_set_nonce_wrapper as NettleSetKeyFunc),
    update: Some(nettle_gcm_camellia128_update as NettleHashUpdateFunc),
    encrypt: Some(nettle_gcm_camellia128_encrypt as NettleCryptFunc),
    decrypt: Some(nettle_gcm_camellia128_decrypt as NettleCryptFunc),
    digest: Some(nettle_gcm_camellia128_digest as NettleHashDigestFunc),
};

extern "C" {
    fn nettle_gcm_camellia128_set_key(ctx: *mut GcmCamellia128Ctx, key: *const uint8_t);
    fn nettle_gcm_camellia128_set_iv(ctx: *mut GcmCamellia128Ctx, length: size_t, iv: *const uint8_t);
    fn nettle_gcm_camellia128_update(ctx: *mut GcmCamellia128Ctx, length: size_t, data: *const uint8_t);
    fn nettle_gcm_camellia128_encrypt(
        ctx: *mut GcmCamellia128Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_camellia128_decrypt(
        ctx: *mut GcmCamellia128Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_camellia128_digest(
        ctx: *mut GcmCamellia128Ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
}