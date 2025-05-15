use std::mem::size_of;
use std::os::raw::{c_char, c_uint, c_uchar, c_void};
use std::slice;

type size_t = usize;
type uint8_t = c_uchar;
type uint32_t = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct serpent_ctx {
    pub keys: [[uint32_t; 4]; 33],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_cipher {
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
    fn nettle_serpent128_set_key(ctx: *mut serpent_ctx, key: *const uint8_t);
    fn nettle_serpent192_set_key(ctx: *mut serpent_ctx, key: *const uint8_t);
    fn nettle_serpent256_set_key(ctx: *mut serpent_ctx, key: *const uint8_t);
    fn nettle_serpent_encrypt(
        ctx: *const serpent_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_serpent_decrypt(
        ctx: *const serpent_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}

pub fn serpent128() -> nettle_cipher {
    nettle_cipher {
        name: b"serpent128\0".as_ptr() as *const c_char,
        context_size: size_of::<serpent_ctx>() as c_uint,
        block_size: 16,
        key_size: 16,
        set_encrypt_key: Some(nettle_serpent128_set_key_wrapper),
        set_decrypt_key: Some(nettle_serpent128_set_key_wrapper),
        encrypt: Some(nettle_serpent_encrypt_wrapper),
        decrypt: Some(nettle_serpent_decrypt_wrapper),
    }
}

pub fn serpent192() -> nettle_cipher {
    nettle_cipher {
        name: b"serpent192\0".as_ptr() as *const c_char,
        context_size: size_of::<serpent_ctx>() as c_uint,
        block_size: 16,
        key_size: 24,
        set_encrypt_key: Some(nettle_serpent192_set_key_wrapper),
        set_decrypt_key: Some(nettle_serpent192_set_key_wrapper),
        encrypt: Some(nettle_serpent_encrypt_wrapper),
        decrypt: Some(nettle_serpent_decrypt_wrapper),
    }
}

pub fn serpent256() -> nettle_cipher {
    nettle_cipher {
        name: b"serpent256\0".as_ptr() as *const c_char,
        context_size: size_of::<serpent_ctx>() as c_uint,
        block_size: 16,
        key_size: 32,
        set_encrypt_key: Some(nettle_serpent256_set_key_wrapper),
        set_decrypt_key: Some(nettle_serpent256_set_key_wrapper),
        encrypt: Some(nettle_serpent_encrypt_wrapper),
        decrypt: Some(nettle_serpent_decrypt_wrapper),
    }
}

extern "C" fn nettle_serpent128_set_key_wrapper(ctx: *mut c_void, key: *const uint8_t) {
    unsafe {
        nettle_serpent128_set_key(ctx as *mut serpent_ctx, key);
    }
}

extern "C" fn nettle_serpent192_set_key_wrapper(ctx: *mut c_void, key: *const uint8_t) {
    unsafe {
        nettle_serpent192_set_key(ctx as *mut serpent_ctx, key);
    }
}

extern "C" fn nettle_serpent256_set_key_wrapper(ctx: *mut c_void, key: *const uint8_t) {
    unsafe {
        nettle_serpent256_set_key(ctx as *mut serpent_ctx, key);
    }
}

extern "C" fn nettle_serpent_encrypt_wrapper(
    ctx: *const c_void,
    length: size_t,
    dst: *mut uint8_t,
    src: *const uint8_t,
) {
    unsafe {
        nettle_serpent_encrypt(ctx as *const serpent_ctx, length, dst, src);
    }
}

extern "C" fn nettle_serpent_decrypt_wrapper(
    ctx: *const c_void,
    length: size_t,
    dst: *mut uint8_t,
    src: *const uint8_t,
) {
    unsafe {
        nettle_serpent_decrypt(ctx as *const serpent_ctx, length, dst, src);
    }
}