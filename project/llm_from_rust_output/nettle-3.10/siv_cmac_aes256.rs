use std::ptr;
use std::mem::MaybeUninit;
use std::os::raw::{c_uchar, c_uint, c_ulong, c_char, c_void};

type size_t = c_ulong;
type uint8_t = c_uchar;
type uint32_t = c_uint;
type uint64_t = c_ulong;

#[derive(Copy, Clone)]
#[repr(C)]
pub union nettle_block16 {
    pub b: [uint8_t; 16],
    pub w: [c_ulong; 2],
    pub u64_0: [uint64_t; 2],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmac128_key {
    pub K1: nettle_block16,
    pub K2: nettle_block16,
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct siv_cmac_aes256_ctx {
    pub cmac_key: cmac128_key,
    pub cmac_cipher: aes256_ctx,
    pub ctr_cipher: aes256_ctx,
}

static NETTLE_AES256: nettle_cipher = nettle_cipher {
    name: ptr::null(),
    context_size: 0,
    block_size: 0,
    key_size: 0,
    set_encrypt_key: None,
    set_decrypt_key: None,
    encrypt: None,
    decrypt: None,
};

pub fn nettle_siv_cmac_aes256_set_key(
    ctx: &mut siv_cmac_aes256_ctx,
    key: &[u8],
) {
    unsafe {
        extern "C" {
            fn nettle_siv_cmac_set_key(
                cmac_key: *mut cmac128_key,
                cmac_cipher: *mut c_void,
                ctr_cipher: *mut c_void,
                nc: *const nettle_cipher,
                key: *const uint8_t,
            );
        }
        nettle_siv_cmac_set_key(
            &mut ctx.cmac_key,
            &mut ctx.cmac_cipher as *mut aes256_ctx as *mut c_void,
            &mut ctx.ctr_cipher as *mut aes256_ctx as *mut c_void,
            &NETTLE_AES256,
            key.as_ptr(),
        );
    }
}

pub fn nettle_siv_cmac_aes256_encrypt_message(
    ctx: &siv_cmac_aes256_ctx,
    nonce: &[u8],
    adata: &[u8],
    dst: &mut [u8],
    src: &[u8],
) {
    unsafe {
        extern "C" {
            fn nettle_siv_cmac_encrypt_message(
                cmac_key: *const cmac128_key,
                cmac_cipher_ctx: *const c_void,
                nc: *const nettle_cipher,
                ctr_ctx: *const c_void,
                nlength: size_t,
                nonce: *const uint8_t,
                alength: size_t,
                adata: *const uint8_t,
                clength: size_t,
                dst: *mut uint8_t,
                src: *const uint8_t,
            );
        }
        nettle_siv_cmac_encrypt_message(
            &ctx.cmac_key,
            &ctx.cmac_cipher as *const aes256_ctx as *const c_void,
            &NETTLE_AES256,
            &ctx.ctr_cipher as *const aes256_ctx as *const c_void,
            nonce.len(),
            nonce.as_ptr(),
            adata.len(),
            adata.as_ptr(),
            dst.len(),
            dst.as_mut_ptr(),
            src.as_ptr(),
        );
    }
}

pub fn nettle_siv_cmac_aes256_decrypt_message(
    ctx: &siv_cmac_aes256_ctx,
    nonce: &[u8],
    adata: &[u8],
    dst: &mut [u8],
    src: &[u8],
) -> i32 {
    unsafe {
        extern "C" {
            fn nettle_siv_cmac_decrypt_message(
                cmac_key: *const cmac128_key,
                cmac_cipher: *const c_void,
                nc: *const nettle_cipher,
                ctr_cipher: *const c_void,
                nlength: size_t,
                nonce: *const uint8_t,
                alength: size_t,
                adata: *const uint8_t,
                mlength: size_t,
                dst: *mut uint8_t,
                src: *const uint8_t,
            ) -> i32;
        }
        nettle_siv_cmac_decrypt_message(
            &ctx.cmac_key,
            &ctx.cmac_cipher as *const aes256_ctx as *const c_void,
            &NETTLE_AES256,
            &ctx.ctr_cipher as *const aes256_ctx as *const c_void,
            nonce.len(),
            nonce.as_ptr(),
            adata.len(),
            adata.as_ptr(),
            dst.len(),
            dst.as_mut_ptr(),
            src.as_ptr(),
        )
    }
}