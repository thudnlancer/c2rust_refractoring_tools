use std::mem;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union NettleBlock16 {
    pub b: [uint8_t; 16],
    pub w: [u64; 2],
    pub u64_0: [uint64_t; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes192Ctx {
    pub keys: [uint32_t; 52],
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
pub struct GcmAes192Ctx {
    pub key: GcmKey,
    pub gcm: GcmCtx,
    pub cipher: Aes192Ctx,
}

pub struct NettleAead {
    pub name: &'static str,
    pub context_size: u32,
    pub block_size: u32,
    pub key_size: u32,
    pub nonce_size: u32,
    pub digest_size: u32,
}

impl GcmAes192Ctx {
    pub fn set_key(&mut self, key: &[uint8_t]) {
        assert_eq!(key.len(), 24);
        unsafe {
            nettle_gcm_aes192_set_key(self as *mut _, key.as_ptr());
        }
    }

    pub fn update(&mut self, data: &[uint8_t]) {
        unsafe {
            nettle_gcm_aes192_update(self as *mut _, data.len(), data.as_ptr());
        }
    }

    pub fn set_iv(&mut self, iv: &[uint8_t]) {
        assert_eq!(iv.len(), 12);
        unsafe {
            nettle_gcm_aes192_set_iv(self as *mut _, iv.len(), iv.as_ptr());
        }
    }

    pub fn encrypt(&mut self, src: &[uint8_t], dst: &mut [uint8_t]) {
        assert_eq!(src.len(), dst.len());
        unsafe {
            nettle_gcm_aes192_encrypt(self as *mut _, src.len(), dst.as_mut_ptr(), src.as_ptr());
        }
    }

    pub fn decrypt(&mut self, src: &[uint8_t], dst: &mut [uint8_t]) {
        assert_eq!(src.len(), dst.len());
        unsafe {
            nettle_gcm_aes192_decrypt(self as *mut _, src.len(), dst.as_mut_ptr(), src.as_ptr());
        }
    }

    pub fn digest(&mut self, digest: &mut [uint8_t]) {
        assert_eq!(digest.len(), 16);
        unsafe {
            nettle_gcm_aes192_digest(self as *mut _, digest.len(), digest.as_mut_ptr());
        }
    }
}

pub const NETTLE_GCM_AES192: NettleAead = NettleAead {
    name: "gcm_aes192",
    context_size: mem::size_of::<GcmAes192Ctx>() as u32,
    block_size: 16,
    key_size: 24,
    nonce_size: 12,
    digest_size: 16,
};

extern "C" {
    fn nettle_gcm_aes192_set_key(ctx: *mut GcmAes192Ctx, key: *const uint8_t);
    fn nettle_gcm_aes192_update(ctx: *mut GcmAes192Ctx, length: size_t, data: *const uint8_t);
    fn nettle_gcm_aes192_set_iv(ctx: *mut GcmAes192Ctx, length: size_t, iv: *const uint8_t);
    fn nettle_gcm_aes192_encrypt(
        ctx: *mut GcmAes192Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_aes192_decrypt(
        ctx: *mut GcmAes192Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_aes192_digest(
        ctx: *mut GcmAes192Ctx,
        length: size_t,
        digest: *mut uint8_t,
    );
}