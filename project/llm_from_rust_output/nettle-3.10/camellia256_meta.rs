use std::mem;
use std::os::raw::{c_char, c_uint, c_ulong, c_uchar, c_void};

type size_t = c_ulong;
type uint8_t = c_uchar;
type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia256_ctx {
    pub keys: [uint64_t; 32],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_cipher {
    pub name: *const c_char,
    pub context_size: c_uint,
    pub block_size: c_uint,
    pub key_size: c_uint,
    pub set_encrypt_key: Option<unsafe extern "C" fn(*mut c_void, *const uint8_t)>,
    pub set_decrypt_key: Option<unsafe extern "C" fn(*mut c_void, *const uint8_t)>,
    pub encrypt: Option<unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t)>,
    pub decrypt: Option<unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t)>,
}

extern "C" {
    fn nettle_camellia256_set_encrypt_key(ctx: *mut camellia256_ctx, key: *const uint8_t);
    fn nettle_camellia256_set_decrypt_key(ctx: *mut camellia256_ctx, key: *const uint8_t);
    fn nettle_camellia256_crypt(
        ctx: *const camellia256_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}

pub static NETTLE_CAMELLIA256: nettle_cipher = nettle_cipher {
    name: b"camellia256\0".as_ptr() as *const c_char,
    context_size: mem::size_of::<camellia256_ctx>() as c_uint,
    block_size: 16,
    key_size: 32,
    set_encrypt_key: Some(unsafe {
        mem::transmute::<
            unsafe extern "C" fn(*mut camellia256_ctx, *const uint8_t),
            unsafe extern "C" fn(*mut c_void, *const uint8_t),
        >(nettle_camellia256_set_encrypt_key)
    }),
    set_decrypt_key: Some(unsafe {
        mem::transmute::<
            unsafe extern "C" fn(*mut camellia256_ctx, *const uint8_t),
            unsafe extern "C" fn(*mut c_void, *const uint8_t),
        >(nettle_camellia256_set_decrypt_key)
    }),
    encrypt: Some(unsafe {
        mem::transmute::<
            unsafe extern "C" fn(*const camellia256_ctx, size_t, *mut uint8_t, *const uint8_t),
            unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t),
        >(nettle_camellia256_crypt)
    }),
    decrypt: Some(unsafe {
        mem::transmute::<
            unsafe extern "C" fn(*const camellia256_ctx, size_t, *mut uint8_t, *const uint8_t),
            unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t),
        >(nettle_camellia256_crypt)
    }),
};