use std::mem;

type size_t = usize;
type uint8_t = u8;
type uint32_t = u32;
type uint64_t = u64;

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

pub struct SafeGcmAes192 {
    ctx: Box<GcmAes192Ctx>,
}

impl SafeGcmAes192 {
    pub fn new() -> Self {
        unsafe {
            let ctx = Box::new(mem::zeroed::<GcmAes192Ctx>());
            Self { ctx }
        }
    }

    pub fn set_key(&mut self, key: &[uint8_t]) {
        unsafe {
            nettle_aes192_set_encrypt_key(&mut self.ctx.cipher, key.as_ptr());
            nettle_gcm_set_key(
                &mut self.ctx.key,
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes192_encrypt as _),
            );
        }
    }

    pub fn set_iv(&mut self, iv: &[uint8_t]) {
        unsafe {
            nettle_gcm_set_iv(
                &mut self.ctx.gcm,
                &self.ctx.key,
                iv.len(),
                iv.as_ptr(),
            );
        }
    }

    pub fn update(&mut self, data: &[uint8_t]) {
        unsafe {
            nettle_gcm_update(
                &mut self.ctx.gcm,
                &self.ctx.key,
                data.len(),
                data.as_ptr(),
            );
        }
    }

    pub fn encrypt(&mut self, src: &[uint8_t], dst: &mut [uint8_t]) {
        assert_eq!(src.len(), dst.len());
        unsafe {
            nettle_gcm_encrypt(
                &mut self.ctx.gcm,
                &self.ctx.key,
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes192_encrypt as _),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt(&mut self, src: &[uint8_t], dst: &mut [uint8_t]) {
        assert_eq!(src.len(), dst.len());
        unsafe {
            nettle_gcm_decrypt(
                &mut self.ctx.gcm,
                &self.ctx.key,
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes192_encrypt as _),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, digest: &mut [uint8_t]) {
        unsafe {
            nettle_gcm_digest(
                &mut self.ctx.gcm,
                &self.ctx.key,
                &self.ctx.cipher as *const _ as *const _,
                Some(nettle_aes192_encrypt as _),
                digest.len(),
                digest.as_mut_ptr(),
            );
        }
    }
}

extern "C" {
    fn nettle_aes192_encrypt(
        ctx: *const Aes192Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes192_set_encrypt_key(ctx: *mut Aes192Ctx, key: *const uint8_t);
    fn nettle_gcm_set_key(
        key: *mut GcmKey,
        cipher: *const std::ffi::c_void,
        f: Option<extern "C" fn(*const std::ffi::c_void, size_t, *mut uint8_t, *const uint8_t)>,
    );
    fn nettle_gcm_set_iv(
        ctx: *mut GcmCtx,
        key: *const GcmKey,
        length: size_t,
        iv: *const uint8_t,
    );
    fn nettle_gcm_update(
        ctx: *mut GcmCtx,
        key: *const GcmKey,
        length: size_t,
        data: *const uint8_t,
    );
    fn nettle_gcm_encrypt(
        ctx: *mut GcmCtx,
        key: *const GcmKey,
        cipher: *const std::ffi::c_void,
        f: Option<extern "C" fn(*const std::ffi::c_void, size_t, *mut uint8_t, *const uint8_t)>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_decrypt(
        ctx: *mut GcmCtx,
        key: *const GcmKey,
        cipher: *const std::ffi::c_void,
        f: Option<extern "C" fn(*const std::ffi::c_void, size_t, *mut uint8_t, *const uint8_t)>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_digest(
        ctx: *mut GcmCtx,
        key: *const GcmKey,
        cipher: *const std::ffi::c_void,
        f: Option<extern "C" fn(*const std::ffi::c_void, size_t, *mut uint8_t, *const uint8_t)>,
        length: size_t,
        digest: *mut uint8_t,
    );
}