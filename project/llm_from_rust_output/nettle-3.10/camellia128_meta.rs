use std::mem;
use std::os::raw::{c_uchar, c_ulong, c_uint, c_void, c_char};

pub type size_t = c_ulong;
pub type uint8_t = c_uchar;
pub type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct camellia128_ctx {
    pub keys: [uint64_t; 24],
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
    fn nettle_camellia128_set_encrypt_key(ctx: *mut camellia128_ctx, key: *const uint8_t);
    fn nettle_camellia_set_decrypt_key(ctx: *mut camellia128_ctx, key: *const uint8_t);
    fn nettle_camellia128_crypt(
        ctx: *const camellia128_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}

pub static NETTLE_CAMELLIA128: nettle_cipher = nettle_cipher {
    name: b"camellia128\0".as_ptr() as *const c_char,
    context_size: mem::size_of::<camellia128_ctx>() as c_uint,
    block_size: 16,
    key_size: 16,
    set_encrypt_key: Some(
        unsafe {
            mem::transmute::<
                extern "C" fn(*mut camellia128_ctx, *const uint8_t),
                extern "C" fn(*mut c_void, *const uint8_t),
            >(nettle_camellia128_set_encrypt_key)
        }
    ),
    set_decrypt_key: Some(
        unsafe {
            mem::transmute::<
                extern "C" fn(*mut camellia128_ctx, *const uint8_t),
                extern "C" fn(*mut c_void, *const uint8_t),
            >(nettle_camellia_set_decrypt_key)
        }
    ),
    encrypt: Some(
        unsafe {
            mem::transmute::<
                extern "C" fn(*const camellia128_ctx, size_t, *mut uint8_t, *const uint8_t),
                extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t),
            >(nettle_camellia128_crypt)
        }
    ),
    decrypt: Some(
        unsafe {
            mem::transmute::<
                extern "C" fn(*const camellia128_ctx, size_t, *mut uint8_t, *const uint8_t),
                extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t),
            >(nettle_camellia128_crypt)
        }
    ),
};