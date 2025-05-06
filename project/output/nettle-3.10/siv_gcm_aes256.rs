#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    static nettle_aes256: nettle_cipher;
    fn nettle_siv_gcm_encrypt_message(
        nc: *const nettle_cipher,
        ctx: *const libc::c_void,
        ctr_ctx: *mut libc::c_void,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        clength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    );
    fn nettle_siv_gcm_decrypt_message(
        nc: *const nettle_cipher,
        ctx: *const libc::c_void,
        ctr_ctx: *mut libc::c_void,
        nlength: size_t,
        nonce: *const uint8_t,
        alength: size_t,
        adata: *const uint8_t,
        mlength: size_t,
        dst: *mut uint8_t,
        src: *const uint8_t,
    ) -> i32;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type nettle_set_key_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *const uint8_t,
) -> ();
pub type nettle_cipher_func = unsafe extern "C" fn(
    *const libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_cipher {
    pub name: *const i8,
    pub context_size: u32,
    pub block_size: u32,
    pub key_size: u32,
    pub set_encrypt_key: Option<nettle_set_key_func>,
    pub set_decrypt_key: Option<nettle_set_key_func>,
    pub encrypt: Option<nettle_cipher_func>,
    pub decrypt: Option<nettle_cipher_func>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aes256_ctx {
    pub keys: [uint32_t; 60],
}
#[no_mangle]
pub unsafe extern "C" fn nettle_siv_gcm_aes256_encrypt_message(
    mut ctx: *const aes256_ctx,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut clength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) {
    let mut ctr_ctx: aes256_ctx = aes256_ctx { keys: [0; 60] };
    nettle_siv_gcm_encrypt_message(
        &nettle_aes256,
        ctx as *const libc::c_void,
        &mut ctr_ctx as *mut aes256_ctx as *mut libc::c_void,
        nlength,
        nonce,
        alength,
        adata,
        clength,
        dst,
        src,
    );
}
#[no_mangle]
pub unsafe extern "C" fn nettle_siv_gcm_aes256_decrypt_message(
    mut ctx: *const aes256_ctx,
    mut nlength: size_t,
    mut nonce: *const uint8_t,
    mut alength: size_t,
    mut adata: *const uint8_t,
    mut mlength: size_t,
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
) -> i32 {
    let mut ctr_ctx: aes256_ctx = aes256_ctx { keys: [0; 60] };
    return nettle_siv_gcm_decrypt_message(
        &nettle_aes256,
        ctx as *const libc::c_void,
        &mut ctr_ctx as *mut aes256_ctx as *mut libc::c_void,
        nlength,
        nonce,
        alength,
        adata,
        mlength,
        dst,
        src,
    );
}