use libc::{c_uchar, c_uint, c_ulong, c_void};

type size_t = c_ulong;
type uint8_t = c_uchar;
type uint32_t = c_uint;

type NettleCipherFunc = unsafe extern "C" fn(*const c_void, size_t, *mut uint8_t, *const uint8_t);

#[derive(Copy, Clone)]
#[repr(C)]
pub struct Aes128Ctx {
    pub keys: [uint32_t; 44],
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct XtsAes128Key {
    pub cipher: Aes128Ctx,
    pub tweak_cipher: Aes128Ctx,
}

pub fn xts_aes128_set_encrypt_key(xts_key: &mut XtsAes128Key, key: &[u8; 32]) {
    assert!(key.len() >= 32);
    
    unsafe {
        nettle_aes128_set_encrypt_key(&mut xts_key.cipher, key.as_ptr());
        nettle_aes128_set_encrypt_key(&mut xts_key.tweak_cipher, key[16..].as_ptr());
    }
}

pub fn xts_aes128_set_decrypt_key(xts_key: &mut XtsAes128Key, key: &[u8; 32]) {
    assert!(key.len() >= 32);
    
    unsafe {
        nettle_aes128_set_decrypt_key(&mut xts_key.cipher, key.as_ptr());
        nettle_aes128_set_encrypt_key(&mut xts_key.tweak_cipher, key[16..].as_ptr());
    }
}

pub fn xts_aes128_encrypt_message(
    xts_key: &XtsAes128Key,
    tweak: &[u8; 16],
    data: &[u8],
    output: &mut [u8],
) {
    assert_eq!(data.len(), output.len());
    
    unsafe {
        nettle_xts_encrypt_message(
            &xts_key.cipher as *const _ as *const c_void,
            &xts_key.tweak_cipher as *const _ as *const c_void,
            Some(nettle_aes128_encrypt_wrapper),
            tweak.as_ptr(),
            data.len(),
            output.as_mut_ptr(),
            data.as_ptr(),
        );
    }
}

pub fn xts_aes128_decrypt_message(
    xts_key: &XtsAes128Key,
    tweak: &[u8; 16],
    data: &[u8],
    output: &mut [u8],
) {
    assert_eq!(data.len(), output.len());
    
    unsafe {
        nettle_xts_decrypt_message(
            &xts_key.cipher as *const _ as *const c_void,
            &xts_key.tweak_cipher as *const _ as *const c_void,
            Some(nettle_aes128_decrypt_wrapper),
            Some(nettle_aes128_encrypt_wrapper),
            tweak.as_ptr(),
            data.len(),
            output.as_mut_ptr(),
            data.as_ptr(),
        );
    }
}

unsafe extern "C" fn nettle_aes128_encrypt_wrapper(
    ctx: *const c_void,
    length: size_t,
    dst: *mut uint8_t,
    src: *const uint8_t,
) {
    nettle_aes128_encrypt(ctx as *const Aes128Ctx, length, dst, src)
}

unsafe extern "C" fn nettle_aes128_decrypt_wrapper(
    ctx: *const c_void,
    length: size_t,
    dst: *mut uint8_t,
    src: *const uint8_t,
) {
    nettle_aes128_decrypt(ctx as *const Aes128Ctx, length, dst, src)
}

extern "C" {
    fn nettle_aes128_encrypt(
        ctx: *const Aes128Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes128_decrypt(
        ctx: *const Aes128Ctx,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_aes128_set_encrypt_key(ctx: *mut Aes128Ctx, key: *const uint8_t);
    fn nettle_aes128_set_decrypt_key(ctx: *mut Aes128Ctx, key: *const uint8_t);
    fn nettle_xts_encrypt_message(
        enc_ctx: *const c_void,
        twk_ctx: *const c_void,
        encf: Option<NettleCipherFunc>,
        tweak: *const uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_xts_decrypt_message(
        dec_ctx: *const c_void,
        twk_ctx: *const c_void,
        decf: Option<NettleCipherFunc>,
        encf: Option<NettleCipherFunc>,
        tweak: *const uint8_t,
        length: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
}