use std::ffi::CStr;
use std::mem::size_of;
use std::os::raw::{c_char, c_uint, c_uchar, c_void};

type size_t = usize;
type uint8_t = c_uchar;
type uint32_t = c_uint;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes192_ctx {
    pub keys: [uint32_t; 52],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct NettleCipher {
    pub name: &'static CStr,
    pub context_size: c_uint,
    pub block_size: c_uint,
    pub key_size: c_uint,
    pub set_encrypt_key: fn(&mut aes192_ctx, &[uint8_t]),
    pub set_decrypt_key: fn(&mut aes192_ctx, &[uint8_t]),
    pub encrypt: fn(&aes192_ctx, &[uint8_t]) -> Vec<uint8_t>,
    pub decrypt: fn(&aes192_ctx, &[uint8_t]) -> Vec<uint8_t>,
}

fn aes192_set_encrypt_key(ctx: &mut aes192_ctx, key: &[uint8_t]) {
    assert_eq!(key.len(), 24);
    unsafe { nettle_aes192_set_encrypt_key(ctx, key.as_ptr()) }
}

fn aes192_set_decrypt_key(ctx: &mut aes192_ctx, key: &[uint8_t]) {
    assert_eq!(key.len(), 24);
    unsafe { nettle_aes192_set_decrypt_key(ctx, key.as_ptr()) }
}

fn aes192_encrypt(ctx: &aes192_ctx, src: &[uint8_t]) -> Vec<uint8_t> {
    let mut dst = vec![0; src.len()];
    unsafe {
        nettle_aes192_encrypt(ctx, src.len(), dst.as_mut_ptr(), src.as_ptr());
    }
    dst
}

fn aes192_decrypt(ctx: &aes192_ctx, src: &[uint8_t]) -> Vec<uint8_t> {
    let mut dst = vec![0; src.len()];
    unsafe {
        nettle_aes192_decrypt(ctx, src.len(), dst.as_mut_ptr(), src.as_ptr());
    }
    dst
}

pub static NETTLE_AES192: NettleCipher = NettleCipher {
    name: CStr::from_bytes_with_nul(b"aes192\0").unwrap(),
    context_size: size_of::<aes192_ctx>() as c_uint,
    block_size: 16,
    key_size: 24,
    set_encrypt_key: aes192_set_encrypt_key,
    set_decrypt_key: aes192_set_decrypt_key,
    encrypt: aes192_encrypt,
    decrypt: aes192_decrypt,
};

extern "C" {
    fn nettle_aes192_set_encrypt_key(ctx: *mut aes192_ctx, key: *const uint8_t);
    fn nettle_aes192_set_decrypt_key(ctx: *mut aes192_ctx, key: *const uint8_t);
    fn nettle_aes192_encrypt(
        ctx: *const aes192_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes192_decrypt(
        ctx: *const aes192_ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}