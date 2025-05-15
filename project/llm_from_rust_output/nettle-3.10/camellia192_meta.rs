use std::mem;
use std::os::raw::{c_uchar, c_ulong, c_uint, c_void, c_char};

pub type size_t = c_ulong;
pub type uint8_t = c_uchar;
pub type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Camellia256Ctx {
    pub keys: [uint64_t; 32],
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
    fn nettle_camellia256_crypt(
        ctx: *const Camellia256Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_camellia192_set_encrypt_key(
        ctx: *mut Camellia256Ctx,
        key: *const uint8_t,
    );
    fn nettle_camellia192_set_decrypt_key(
        ctx: *mut Camellia256Ctx,
        key: *const uint8_t,
    );
}

pub static NETTLE_CAMELLIA192: NettleCipher = NettleCipher {
    name: b"camellia192\0".as_ptr() as *const c_char,
    context_size: mem::size_of::<Camellia256Ctx>() as c_uint,
    block_size: 16,
    key_size: 24,
    set_encrypt_key: Some(nettle_camellia192_set_encrypt_key),
    set_decrypt_key: Some(nettle_camellia192_set_decrypt_key),
    encrypt: Some(nettle_camellia256_crypt),
    decrypt: Some(nettle_camellia256_crypt),
};