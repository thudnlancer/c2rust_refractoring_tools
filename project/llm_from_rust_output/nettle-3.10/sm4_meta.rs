use std::mem;
use std::os::raw::{c_char, c_uint, c_uchar, c_ulong, c_void};

pub type size_t = c_ulong;
pub type uint8_t = c_uchar;
pub type uint32_t = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct sm4_ctx {
    pub rkey: [uint32_t; 32],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NettleCipher {
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
    fn nettle_sm4_set_encrypt_key(ctx: *mut sm4_ctx, key: *const uint8_t);
    fn nettle_sm4_set_decrypt_key(ctx: *mut sm4_ctx, key: *const uint8_t);
    fn nettle_sm4_crypt(
        context: *const sm4_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}

pub static NETTLE_SM4: NettleCipher = NettleCipher {
    name: b"sm4\0".as_ptr() as *const c_char,
    context_size: mem::size_of::<sm4_ctx>() as c_uint,
    block_size: 16,
    key_size: 16,
    set_encrypt_key: Some(unsafe {
        mem::transmute::<
            unsafe extern "C" fn(*mut sm4_ctx, *const uint8_t),
            unsafe extern "C" fn(*mut c_void, *const uint8_t),
        >(nettle_sm4_set_encrypt_key)
    }),
    set_decrypt_key: Some(unsafe {
        mem::transmute::<
            unsafe extern "C" fn(*mut sm4_ctx, *const uint8_t),
            unsafe extern "C" fn(*mut c_void, *const uint8_t),
        >(nettle_sm4_set_decrypt_key)
    }),
    encrypt: Some(unsafe {
        mem::transmute::<
            unsafe extern "C" fn(*const sm4_ctx, size_t, *mut uint8_t, *const uint8_t),
            unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t),
        >(nettle_sm4_crypt)
    }),
    decrypt: Some(unsafe {
        mem::transmute::<
            unsafe extern "C" fn(*const sm4_ctx, size_t, *mut uint8_t, *const uint8_t),
            unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t),
        >(nettle_sm4_crypt)
    }),
};