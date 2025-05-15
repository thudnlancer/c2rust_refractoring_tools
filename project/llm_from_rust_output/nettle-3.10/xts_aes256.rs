use std::mem::MaybeUninit;

pub type size_t = usize;
pub type uint8_t = u8;
pub type uint32_t = u32;

#[derive(Clone, Copy)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct xts_aes256_key {
    pub cipher: aes256_ctx,
    pub tweak_cipher: aes256_ctx,
}

pub struct XtsAes256 {
    key: xts_aes256_key,
}

impl XtsAes256 {
    pub fn new_encrypt(key: &[u8; 64]) -> Self {
        let mut ctx = MaybeUninit::<xts_aes256_key>::uninit();
        unsafe {
            nettle_aes256_set_encrypt_key(
                &mut (*ctx.as_mut_ptr()).cipher,
                key.as_ptr(),
            );
            nettle_aes256_set_encrypt_key(
                &mut (*ctx.as_mut_ptr()).tweak_cipher,
                key[32..].as_ptr(),
            );
            Self {
                key: ctx.assume_init(),
            }
        }
    }

    pub fn new_decrypt(key: &[u8; 64]) -> Self {
        let mut ctx = MaybeUninit::<xts_aes256_key>::uninit();
        unsafe {
            nettle_aes256_set_decrypt_key(
                &mut (*ctx.as_mut_ptr()).cipher,
                key.as_ptr(),
            );
            nettle_aes256_set_encrypt_key(
                &mut (*ctx.as_mut_ptr()).tweak_cipher,
                key[32..].as_ptr(),
            );
            Self {
                key: ctx.assume_init(),
            }
        }
    }

    pub fn encrypt_message(&self, tweak: &[u8; 16], src: &[u8], dst: &mut [u8]) {
        assert_eq!(src.len(), dst.len());
        unsafe {
            nettle_xts_encrypt_message(
                &self.key.cipher as *const _ as *const libc::c_void,
                &self.key.tweak_cipher as *const _ as *const libc::c_void,
                Some(nettle_aes256_encrypt as _),
                tweak.as_ptr(),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }

    pub fn decrypt_message(&self, tweak: &[u8; 16], src: &[u8], dst: &mut [u8]) {
        assert_eq!(src.len(), dst.len());
        unsafe {
            nettle_xts_decrypt_message(
                &self.key.cipher as *const _ as *const libc::c_void,
                &self.key.tweak_cipher as *const _ as *const libc::c_void,
                Some(nettle_aes256_decrypt as _),
                Some(nettle_aes256_encrypt as _),
                tweak.as_ptr(),
                src.len(),
                dst.as_mut_ptr(),
                src.as_ptr(),
            );
        }
    }
}

extern "C" {
    fn nettle_aes256_decrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes256_set_encrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes256_set_decrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_xts_encrypt_message(
        enc_ctx: *const libc::c_void,
        twk_ctx: *const libc::c_void,
        encf: Option<unsafe extern "C" fn(*const libc::c_void, size_t, *mut uint8_t, *const uint8_t)>,
        tweak: *const uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_xts_decrypt_message(
        dec_ctx: *const libc::c_void,
        twk_ctx: *const libc::c_void,
        decf: Option<unsafe extern "C" fn(*const libc::c_void, size_t, *mut uint8_t, *const uint8_t)>,
        encf: Option<unsafe extern "C" fn(*const libc::c_void, size_t, *mut uint8_t, *const uint8_t)>,
        tweak: *const uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}