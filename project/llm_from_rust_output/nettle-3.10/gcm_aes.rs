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
pub struct Aes128Ctx {
    pub keys: [uint32_t; 44],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes192Ctx {
    pub keys: [uint32_t; 52],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes256Ctx {
    pub keys: [uint32_t; 60],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct AesCtx {
    pub key_size: u32,
    pub u: AesUnion,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union AesUnion {
    pub ctx128: Aes128Ctx,
    pub ctx192: Aes192Ctx,
    pub ctx256: Aes256Ctx,
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
pub struct GcmAesCtx {
    pub key: GcmKey,
    pub gcm: GcmCtx,
    pub cipher: AesCtx,
}

pub struct SafeGcmAesCtx {
    inner: Box<GcmAesCtx>,
}

impl SafeGcmAesCtx {
    pub fn new() -> Self {
        unsafe {
            let ctx = Box::new(mem::zeroed());
            Self { inner: ctx }
        }
    }

    pub fn set_key(&mut self, key: &[u8]) {
        unsafe {
            nettle_aes_set_encrypt_key(
                &mut self.inner.cipher,
                key.len(),
                key.as_ptr(),
            );
            nettle_gcm_set_key(
                &mut self.inner.key,
                &self.inner.cipher as *const _ as *const _,
                Some(nettle_aes_encrypt_wrapper),
            );
        }
    }

    pub fn set_iv(&mut self, iv: &[u8]) {
        unsafe {
            nettle_gcm_set_iv(
                &mut self.inner.gcm,
                &self.inner.key,
                iv.len(),
                iv.as_ptr(),
            );
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            nettle_gcm_update(
                &mut self.inner.gcm,
                &self.inner.key,
                data.len(),
                data.as_ptr(),
            );
        }
    }

    pub fn encrypt(&mut self, src: &[u8], dst: &mut [u8]) {
        assert_eq!(src.len(), dst.len());
        unsafe {
            nettle_gcm_encrypt(
                &mut self.inner.gcm,
                &self.inner.key,
                &self.inner.cipher as *const _ as *const _,
                Some(nettle_aes_encrypt_wrapper),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt(&mut self, src: &[u8], dst: &mut [u8]) {
        assert_eq!(src.len(), dst.len());
        unsafe {
            nettle_gcm_decrypt(
                &mut self.inner.gcm,
                &self.inner.key,
                &self.inner.cipher as *const _ as *const _,
                Some(nettle_aes_encrypt_wrapper),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        unsafe {
            nettle_gcm_digest(
                &mut self.inner.gcm,
                &self.inner.key,
                &self.inner.cipher as *const _ as *const _,
                Some(nettle_aes_encrypt_wrapper),
                digest.len(),
                digest.as_mut_ptr(),
            );
        }
    }
}

unsafe extern "C" fn nettle_aes_encrypt_wrapper(
    ctx: *const libc::c_void,
    length: size_t,
    dst: *mut uint8_t,
    src: *const uint8_t,
) {
    let ctx = ctx as *const AesCtx;
    nettle_aes_encrypt(ctx, length, dst, src);
}

extern "C" {
    fn nettle_aes_set_encrypt_key(
        ctx: *mut AesCtx,
        length: size_t,
        key: *const uint8_t,
    );
    fn nettle_aes_encrypt(
        ctx: *const AesCtx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_set_key(
        key: *mut GcmKey,
        cipher: *const libc::c_void,
        f: Option<unsafe extern "C" fn(*const libc::c_void, size_t, *mut uint8_t, *const uint8_t)>,
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
        cipher: *const libc::c_void,
        f: Option<unsafe extern "C" fn(*const libc::c_void, size_t, *mut uint8_t, *const uint8_t)>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_decrypt(
        ctx: *mut GcmCtx,
        key: *const GcmKey,
        cipher: *const libc::c_void,
        f: Option<unsafe extern "C" fn(*const libc::c_void, size_t, *mut uint8_t, *const uint8_t)>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_digest(
        ctx: *mut GcmCtx,
        key: *const GcmKey,
        cipher: *const libc::c_void,
        f: Option<unsafe extern "C" fn(*const libc::c_void, size_t, *mut uint8_t, *const uint8_t)>,
        length: size_t,
        digest: *mut uint8_t,
    );
}