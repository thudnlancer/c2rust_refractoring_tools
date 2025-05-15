use std::mem::MaybeUninit;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint64_t = u64;

#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [u64; 2],
    pub u64_0: [uint64_t; 2],
}

pub type NettleCipherFunc = fn(ctx: &dyn std::any::Any, length: size_t, dst: &mut [u8], src: &[u8]);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia128_ctx {
    pub keys: [uint64_t; 24],
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
pub struct gcm_camellia128_ctx {
    pub key: gcm_key,
    pub gcm: gcm_ctx,
    pub cipher: camellia128_ctx,
}

impl gcm_camellia128_ctx {
    pub fn set_key(&mut self, key: &[u8]) {
        unsafe {
            nettle_camellia128_set_encrypt_key(&mut self.cipher, key.as_ptr());
        }
        
        let cipher_ptr = &self.cipher as *const camellia128_ctx as *const std::ffi::c_void;
        let cipher_fn = Some(nettle_camellia128_crypt_wrapper as nettle_cipher_func);
        
        unsafe {
            nettle_gcm_set_key(&mut self.key, cipher_ptr, cipher_fn);
        }
    }

    pub fn set_iv(&mut self, iv: &[u8]) {
        unsafe {
            nettle_gcm_set_iv(&mut self.gcm, &self.key, iv.len(), iv.as_ptr());
        }
    }

    pub fn update(&mut self, data: &[u8]) {
        unsafe {
            nettle_gcm_update(&mut self.gcm, &self.key, data.len(), data.as_ptr());
        }
    }

    pub fn encrypt(&mut self, dst: &mut [u8], src: &[u8]) {
        assert_eq!(dst.len(), src.len());
        unsafe {
            nettle_gcm_encrypt(
                &mut self.gcm,
                &self.key,
                &self.cipher as *const camellia128_ctx as *const std::ffi::c_void,
                Some(nettle_camellia128_crypt_wrapper as nettle_cipher_func),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt(&mut self, dst: &mut [u8], src: &[u8]) {
        assert_eq!(dst.len(), src.len());
        unsafe {
            nettle_gcm_decrypt(
                &mut self.gcm,
                &self.key,
                &self.cipher as *const camellia128_ctx as *const std::ffi::c_void,
                Some(nettle_camellia128_crypt_wrapper as nettle_cipher_func),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn digest(&mut self, digest: &mut [u8]) {
        unsafe {
            nettle_gcm_digest(
                &mut self.gcm,
                &self.key,
                &self.cipher as *const camellia128_ctx as *const std::ffi::c_void,
                Some(nettle_camellia128_crypt_wrapper as nettle_cipher_func),
                digest.len(),
                digest.as_mut_ptr(),
            );
        }
    }
}

fn nettle_camellia128_crypt_wrapper(
    ctx: *const std::ffi::c_void,
    length: size_t,
    dst: *mut uint8_t,
    src: *const uint8_t,
) {
    let ctx = unsafe { &*(ctx as *const camellia128_ctx) };
    unsafe {
        nettle_camellia128_crypt(ctx, length, dst, src);
    }
}

extern "C" {
    fn nettle_camellia128_set_encrypt_key(ctx: *mut camellia128_ctx, key: *const uint8_t);
    fn nettle_camellia128_crypt(
        ctx: *const camellia128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_set_key(
        key: *mut gcm_key,
        cipher: *const std::ffi::c_void,
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
        cipher: *const std::ffi::c_void,
        f: Option<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_decrypt(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        cipher: *const std::ffi::c_void,
        f: Option<nettle_cipher_func>,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_gcm_digest(
        ctx: *mut gcm_ctx,
        key: *const gcm_key,
        cipher: *const std::ffi::c_void,
        f: Option<nettle_cipher_func>,
        length: size_t,
        digest: *mut uint8_t,
    );
}