use std::mem;
use std::os::raw::{c_uchar, c_uint, c_ulong, c_char};

pub type size_t = c_ulong;
pub type uint8_t = c_uchar;
pub type uint32_t = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes128Ctx {
    pub keys: [uint32_t; 44],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NettleCipher {
    pub name: *const c_char,
    pub context_size: c_uint,
    pub block_size: c_uint,
    pub key_size: c_uint,
    pub set_encrypt_key: Option<unsafe extern "C" fn(*mut libc::c_void, *const uint8_t)>,
    pub set_decrypt_key: Option<unsafe extern "C" fn(*mut libc::c_void, *const uint8_t)>,
    pub encrypt: Option<unsafe extern "C" fn(*const libc::c_void, size_t, *mut uint8_t, *const uint8_t)>,
    pub decrypt: Option<unsafe extern "C" fn(*const libc::c_void, size_t, *mut uint8_t, *const uint8_t)>,
}

extern "C" {
    fn nettle_aes128_set_encrypt_key(ctx: *mut Aes128Ctx, key: *const uint8_t);
    fn nettle_aes128_set_decrypt_key(ctx: *mut Aes128Ctx, key: *const uint8_t);
    fn nettle_aes128_encrypt(ctx: *const Aes128Ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
    fn nettle_aes128_decrypt(ctx: *const Aes128Ctx, length: size_t, dst: *mut uint8_t, src: *const uint8_t);
}

pub static NETTLE_AES128: NettleCipher = NettleCipher {
    name: b"aes128\0".as_ptr() as *const c_char,
    context_size: mem::size_of::<Aes128Ctx>() as c_uint,
    block_size: 16,
    key_size: 16,
    set_encrypt_key: Some(nettle_aes128_set_encrypt_key as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t)),
    set_decrypt_key: Some(nettle_aes128_set_decrypt_key as unsafe extern "C" fn(*mut libc::c_void, *const uint8_t)),
    encrypt: Some(nettle_aes128_encrypt as unsafe extern "C" fn(*const libc::c_void, size_t, *mut uint8_t, *const uint8_t)),
    decrypt: Some(nettle_aes128_decrypt as unsafe extern "C" fn(*const libc::c_void, size_t, *mut uint8_t, *const uint8_t)),
};