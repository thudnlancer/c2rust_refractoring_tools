use std::mem;
use std::os::raw::{c_char, c_uint, c_uchar, c_void};
use std::ffi::CStr;

type size_t = usize;
type uint8_t = c_uchar;
type uint32_t = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NettleCipher {
    pub name: *const c_char,
    pub context_size: c_uint,
    pub block_size: c_uint,
    pub key_size: c_uint,
    pub set_encrypt_key: Option<extern "C" fn(*mut c_void, *const uint8_t)>,
    pub set_decrypt_key: Option<extern "C" fn(*mut c_void, *const uint8_t)>,
    pub encrypt: Option<extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t)>,
    pub decrypt: Option<extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t)>,
}

extern "C" {
    fn nettle_aes256_set_encrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes256_set_decrypt_key(ctx: *mut aes256_ctx, key: *const uint8_t);
    fn nettle_aes256_encrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes256_decrypt(
        ctx: *const aes256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}

pub const NETTLE_AES256: NettleCipher = NettleCipher {
    name: b"aes256\0".as_ptr() as *const c_char,
    context_size: mem::size_of::<aes256_ctx>() as c_uint,
    block_size: 16,
    key_size: 32,
    set_encrypt_key: Some(nettle_aes256_set_encrypt_key_wrapper),
    set_decrypt_key: Some(nettle_aes256_set_decrypt_key_wrapper),
    encrypt: Some(nettle_aes256_encrypt_wrapper),
    decrypt: Some(nettle_aes256_decrypt_wrapper),
};

extern "C" fn nettle_aes256_set_encrypt_key_wrapper(ctx: *mut c_void, key: *const uint8_t) {
    unsafe {
        nettle_aes256_set_encrypt_key(ctx as *mut aes256_ctx, key);
    }
}

extern "C" fn nettle_aes256_set_decrypt_key_wrapper(ctx: *mut c_void, key: *const uint8_t) {
    unsafe {
        nettle_aes256_set_decrypt_key(ctx as *mut aes256_ctx, key);
    }
}

extern "C" fn nettle_aes256_encrypt_wrapper(
    ctx: *const c_void,
    length: size_t,
    dst: *mut uint8_t,
    src: *const uint8_t,
) {
    unsafe {
        nettle_aes256_encrypt(ctx as *const aes256_ctx, length, dst, src);
    }
}

extern "C" fn nettle_aes256_decrypt_wrapper(
    ctx: *const c_void,
    length: size_t,
    dst: *mut uint8_t,
    src: *const uint8_t,
) {
    unsafe {
        nettle_aes256_decrypt(ctx as *const aes256_ctx, length, dst, src);
    }
}

pub fn get_aes256_cipher() -> &'static NettleCipher {
    &NETTLE_AES256
}