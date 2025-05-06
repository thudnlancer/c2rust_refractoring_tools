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
    static nettle_chacha_poly1305: nettle_aead;
    static nettle_eax_aes128: nettle_aead;
    static nettle_gcm_sm4: nettle_aead;
    static nettle_gcm_camellia256: nettle_aead;
    static nettle_gcm_camellia128: nettle_aead;
    static nettle_gcm_aes256: nettle_aead;
    static nettle_gcm_aes192: nettle_aead;
    static nettle_gcm_aes128: nettle_aead;
}
pub type size_t = u64;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type nettle_set_key_func = unsafe extern "C" fn(
    *mut libc::c_void,
    *const uint8_t,
) -> ();
pub type nettle_crypt_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_update_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *const uint8_t,
) -> ();
pub type nettle_hash_digest_func = unsafe extern "C" fn(
    *mut libc::c_void,
    size_t,
    *mut uint8_t,
) -> ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct nettle_aead {
    pub name: *const i8,
    pub context_size: u32,
    pub block_size: u32,
    pub key_size: u32,
    pub nonce_size: u32,
    pub digest_size: u32,
    pub set_encrypt_key: Option<nettle_set_key_func>,
    pub set_decrypt_key: Option<nettle_set_key_func>,
    pub set_nonce: Option<nettle_set_key_func>,
    pub update: Option<nettle_hash_update_func>,
    pub encrypt: Option<nettle_crypt_func>,
    pub decrypt: Option<nettle_crypt_func>,
    pub digest: Option<nettle_hash_digest_func>,
}
#[no_mangle]
pub static mut _nettle_aeads: [*const nettle_aead; 9] = unsafe {
    [
        &nettle_gcm_aes128 as *const nettle_aead,
        &nettle_gcm_aes192 as *const nettle_aead,
        &nettle_gcm_aes256 as *const nettle_aead,
        &nettle_gcm_camellia128 as *const nettle_aead,
        &nettle_gcm_camellia256 as *const nettle_aead,
        &nettle_gcm_sm4 as *const nettle_aead,
        &nettle_eax_aes128 as *const nettle_aead,
        &nettle_chacha_poly1305 as *const nettle_aead,
        0 as *const nettle_aead,
    ]
};
#[no_mangle]
pub unsafe extern "C" fn nettle_get_aeads() -> *const *const nettle_aead {
    return _nettle_aeads.as_ptr();
}