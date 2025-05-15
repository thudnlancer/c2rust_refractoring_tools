use std::os::raw::{c_uchar, c_uint, c_ulong, c_void, c_char};

pub type size_t = c_ulong;
pub type uint8_t = c_uchar;
pub type uint32_t = c_uint;

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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TwofishCtx {
    pub keys: [uint32_t; 40],
    pub s_box: [[uint32_t; 256]; 4],
}

extern "C" {
    fn nettle_twofish128_set_key(context: *mut TwofishCtx, key: *const uint8_t);
    fn nettle_twofish192_set_key(context: *mut TwofishCtx, key: *const uint8_t);
    fn nettle_twofish256_set_key(context: *mut TwofishCtx, key: *const uint8_t);
    fn nettle_twofish_encrypt(
        ctx: *const TwofishCtx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_twofish_decrypt(
        ctx: *const TwofishCtx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}

pub static NETTLE_TWOFISH128: NettleCipher = NettleCipher {
    name: b"twofish128\0".as_ptr() as *const c_char,
    context_size: std::mem::size_of::<TwofishCtx>() as c_uint,
    block_size: 16,
    key_size: 16,
    set_encrypt_key: Some(nettle_twofish128_set_key as unsafe extern "C" fn(*mut c_void, *const uint8_t)),
    set_decrypt_key: Some(nettle_twofish128_set_key as unsafe extern "C" fn(*mut c_void, *const uint8_t)),
    encrypt: Some(nettle_twofish_encrypt as unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t)),
    decrypt: Some(nettle_twofish_decrypt as unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t)),
};

pub static NETTLE_TWOFISH192: NettleCipher = NettleCipher {
    name: b"twofish192\0".as_ptr() as *const c_char,
    context_size: std::mem::size_of::<TwofishCtx>() as c_uint,
    block_size: 16,
    key_size: 24,
    set_encrypt_key: Some(nettle_twofish192_set_key as unsafe extern "C" fn(*mut c_void, *const uint8_t)),
    set_decrypt_key: Some(nettle_twofish192_set_key as unsafe extern "C" fn(*mut c_void, *const uint8_t)),
    encrypt: Some(nettle_twofish_encrypt as unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t)),
    decrypt: Some(nettle_twofish_decrypt as unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t)),
};

pub static NETTLE_TWOFISH256: NettleCipher = NettleCipher {
    name: b"twofish256\0".as_ptr() as *const c_char,
    context_size: std::mem::size_of::<TwofishCtx>() as c_uint,
    block_size: 16,
    key_size: 32,
    set_encrypt_key: Some(nettle_twofish256_set_key as unsafe extern "C" fn(*mut c_void, *const uint8_t)),
    set_decrypt_key: Some(nettle_twofish256_set_key as unsafe extern "C" fn(*mut c_void, *const uint8_t)),
    encrypt: Some(nettle_twofish_encrypt as unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t)),
    decrypt: Some(nettle_twofish_decrypt as unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t)),
};